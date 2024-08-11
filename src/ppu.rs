use std::ops::BitAnd;

use bitflags::{bitflags, Flags};

use crate::frame::Frame;
use crate::{mapper::Mapper, rom::Mirroring};

const VRAM_SIZE: usize = 2048;
const PALETTE_SIZE: usize = 0x20;

#[rustfmt::skip]
pub static SYSTEM_PALLETE: [(u8,u8,u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];

// 0. Load Donkey Kong :)
//
// 1. Make sure you have NMI implemented on CPU (pretty straightforward)
//
// 2. Implement PPUSTATUS vblank flag (simple) and PPUCTRL NMI flag + background address flag (simple)
//
// 3. Implement PPUADDR/PPUDATA so that the nametables are filled out
//
// 4. Now you have some data your PPU can actually read for rendering background. Render it scanline by scanline - just follow the wiki on this. Maybe the timing will be bad, it doesn't matter for this game. Start off with rendering tiles based on the pattern table ID, don't try and fetch patterns.
//
// 5. Fix the inevitable bugs with your PPUDATA implementation until you see a blocky version of the Donkey Kong screen.
//
// 6. Now fetch pattern table data using the nametable data. If it looks "wrong" make sure you are consuming the background address flag. Start off with black and white, then pick two colors to mix for the two bits. Now you should have something like https://i.imgur.com/7OIpHgd.png
//
// 7. (Optional) implement palette reads (I'm skipping this for now).
//
// 8. Implement OAMDMA (and OAMDATA I guess, I implemented one on top of the other)
//
// 9. Now you should have sprite data to render. Implement the logic for copying from primary OAM to scanline OAM. I'm doing it all as one step (not smearing it over up to 256 cycles like the actual hardware). Skip the confusing sprite overflow junk.
//
//    This is where I'm stuck. I think I need to read the "sprites" section of https://wiki.nesdev.com/w/index.php/PPU_rendering very carefully.
//

enum SpriteEvaluationState {
    Copying,
    Evaluation,
    OverflowLogic,
}

type Rgb = (u8, u8, u8);

pub struct Ppu {
    pub frame: Frame,

    pub chr_rom: Vec<u8>,
    pub vram: [u8; VRAM_SIZE],
    pub palette_ram: [u8; PALETTE_SIZE],
    pub screen_mirroring: Mirroring,

    pub control_register: ControlFlags,
    pub status_register: StatusFlags,

    pub cycle: usize,
    pub scanline: usize,

    // 0 to 3 is background palette, 4 to 7 is sprite palette
    pub oam: [u8; 256],
    pub oam_address: u8,
    pub oam_data: u8,
    pub sec_oam: [u8; 32], // 32-byte buffer for current sprites on scanline
    pub sec_oam_address: usize,
    pub sec_oam_writes_disabled: bool,
    pub copy_sprite_signal: i32,
    pub oam_address_overflow: bool,
    pub sec_oam_address_overflow: bool,
    pub overflow_detection: bool,

    pub nametable_byte: u8,
    pub attribute_byte: u8,
    pub pattern_table_low_byte: u8,
    pub pattern_table_high_byte: u8,

    pub pattern_low_shift_register: u16,
    pub pattern_high_shift_register: u16,
    pub attribute_low_shift_register: u8,
    pub attribute_high_shift_register: u8,
    pub attribute_low_bit_latch: u8,  // 1 bit latch
    pub attribute_high_bit_latch: u8, // 1 bit latch

    // Internal registers
    pub v: u16,
    pub t: u16,
    pub x: u8,
    pub w: bool,
    pub x_scroll: u8,
    pub y_scroll: u8,
}

#[allow(clippy::unusual_byte_groupings)]
impl Ppu {
    pub fn new(chr_rom: Vec<u8>, screen_mirroring: Mirroring) -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            control_register: ControlFlags::empty(),
            chr_rom,
            screen_mirroring,
            cycle: 0,
            scanline: 261,
            nametable_byte: 0,
            attribute_byte: 0,
            pattern_table_low_byte: 0,
            pattern_table_high_byte: 0,
            frame: Frame::default(),
            oam_data: 0,
            sec_oam: [0; 32],
            status_register: StatusFlags::empty(),
            oam: [0; 256],
            oam_address: 0,
            sec_oam_address: 0,
            sec_oam_writes_disabled: false,
            copy_sprite_signal: 0,
            oam_address_overflow: false,
            sec_oam_address_overflow: false,
            overflow_detection: false,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            x_scroll: 0,
            y_scroll: 0,
            pattern_low_shift_register: 0,
            pattern_high_shift_register: 0,
            attribute_low_shift_register: 0,
            attribute_high_shift_register: 0,
            attribute_low_bit_latch: 0,
            attribute_high_bit_latch: 0,
            palette_ram: [0; PALETTE_SIZE],
        }
    }

    // NOTE: Read and writes to ppudata during rendering updates v in an odd way, but that
    // behaviour is rarely used. See https://www.nesdev.org/wiki/PPU_scrolling#$2007_(PPUDATA)_reads_and_writes for more info
    pub fn read_ppudata(&mut self) -> u8 {
        let value = self.read(self.v);
        self.v += if self.control_register.vram_address_increment() == 0 { 1 } else { 32 };
        println!("READ PPUDATA VRAM_ADDRESS: {:X}", self.v);
        value
    }

    pub fn write_ppudata(&mut self, value: u8) {
        self.write(self.v, value);
        self.v += if self.control_register.vram_address_increment() == 0 { 1 } else { 32 };
        println!("WRITE PPUDATA VRAM_ADDRESS: {:X}", self.v);
    }

    pub fn read_status(&mut self) -> StatusFlags {
        self.w = false;
        self.status_register.set(StatusFlags::VerticalBlankStarted, false);
        self.status_register
    }

    pub fn write_oam_dma(&mut self, oam_data: &[u8; 256]) {}

    pub fn write_ppuaddr(&mut self, value: u8) {
        // Upper byte is written first, then lower byte
        if !self.w {
            self.t &= 0xFF;
            self.t |= ((value as u16) & 0b111111) << 8;
        } else {
            self.t &= 0xFF00;
            self.t |= value as u16;
            self.v = self.t;
            println!("WRITE PPUADDR VRAM_ADDRESS: {:X}", self.v);
        }
        self.w = !self.w;
    }

    pub fn write_control(&mut self, value: u8) {
        self.control_register = ControlFlags::from_bits_truncate(value);
        let value_bits = (value as u16 & 0b11) << 10;
        self.t = (self.t & 0b111_00_11111_11111) | value_bits;
    }
    pub fn write_oam_data(&mut self, value: u8) {
        self.oam[self.oam_address as usize] = value;
        self.oam_address += 1;
    }

    pub fn read_oam_data(&mut self) -> u8 {
        if self.cycle >= 1 && self.cycle <= 64 {
            return 0xFF;
        }
        self.oam[self.oam_address as usize]
    }

    pub fn write_scroll(&mut self, value: u8) {
        // TODO: Implement the complicated logic explained here: https://www.nesdev.org/wiki/PPU_scrolling

        // 15 bit registers t and v are composed this way during rendering
        // yyy NN YYYYY XXXXX
        // ||| || ||||| +++++-- coarse X scroll
        // ||| || +++++-------- coarse Y scroll
        // ||| ++-------------- nametable select
        // +++----------------- fine Y scroll
        if self.w {
            // Second write
            self.t &= 0b000_11_00000_11111;
            self.t |= (value as u16 & 0b111) << 12;
            self.t |= (value as u16 & 0b11111000) << 5;
            self.y_scroll = value;
        } else {
            // First write
            self.t = (self.t & !0b11111) | (value as u16 >> 3);
            self.x = value & 0b111;
            self.x_scroll = value;
        }
        self.w = !self.w;
    }

    fn nametable_byte(&mut self) -> u8 {
        // Here we & the vram address with a mask including nametable select, coarse y scroll, and coarse x scroll
        let address = 0x2000 | (self.v & 0b000_11_11111_11111);
        let nametable_base = self.control_register.bitand(ControlFlags::BaseNametableMask).bits();
        self.read(address | ((nametable_base as u16) << 12))
    }

    fn pattern_table_address(&mut self) -> u16 {
        // 0HNNNN NNNNPyyy
        // |||||| |||||+++- T: Fine Y offset, the row number within a tile
        // |||||| ||||+---- P: Bit plane (0: less significant bit; 1: more significant bit)
        // ||++++-++++----- N: Tile number from name table
        // |+-------------- H: Half of pattern table (0: "left"; 1: "right")
        // +--------------- 0: Pattern table is at $0000-$1FFF

        let fine_y = (self.v & 0b111_00_00000_00000) >> 12;
        let right_half_bit = if self.control_register.contains(ControlFlags::BackgroundPatternTableAddress) {
            1 << 12
        } else {
            0
        };

        right_half_bit | (self.nametable_byte() << 4) as u16 | fine_y
    }

    // Attribute determines what palette to use
    fn attribute_address(&self) -> u16 {
        0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 0x07)
    }

    fn increment_coarse_x(&mut self) {
        if self.v & 0b11111 == 31 {
            self.v &= !0b11111; // Coarse x = 0
            self.v ^= 0b000_01_00000_0000; // switch horizontal nametable
        } else {
            self.v += 1;
        }
        println!("COARSE X VRAM_ADDRESS: {:X}", self.v);
    }
    fn increment_fine_y(&mut self) {
        if (self.v & 0b111_00_00000_00000) != 0b111_00_00000_00000 {
            // if fine Y < 7
            self.v += 0b001_00_00000_00000; // increment fine Y
        } else {
            self.v &= !0b111_00_00000_00000; // fine Y = 0
            let mut y = (self.v & 0b000_00_11111_00000) >> 5; // let y = coarse Y
            if y == 29 {
                y = 0; // coarse Y = 0
                self.v ^= 0b000_10_00000_00000; // switch vertical nametable
            } else if y == 31 {
                y = 0; // coarse Y = 0, nametable not switched
            } else {
                y += 1; // increment coarse Y
            }
            self.v = (self.v & !0b000_00_11111_00000) | (y << 5); // put coarse Y back into v
        }
        println!("FINE_Y VRAM_ADDRESS: {:X}", self.v);
    }

    pub fn step(&mut self) {
        //println!("Scanline: {}, Cycle: {}", self.scanline, self.cycle);
        match self.scanline {
            0..=239 => self.render(), // Visible scanlines
            240 => {}                 // Post-render scanline
            241..=260 => {
                // Vertical blanking lines
                if self.scanline == 241 && self.cycle == 0 {
                    self.status_register.set(StatusFlags::VerticalBlankStarted, true);
                }
            } // Vertical blanking lines
            261 => {
                // Pre-render scanline
                //self.frame.canvas.present();
                if self.cycle == 0 {
                    self.status_register.set(StatusFlags::VerticalBlankStarted, false);
                }
            }
            _ => unreachable!("scanline should never be {}", self.scanline),
        };

        // Increment the cycle and handle the end of scanlines and frames
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;

            if self.cycle == 257 {
                self.v = (self.v & 0b111_10_11111_00000) | (self.t & 0b111_10_11111_00000);
            }

            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
            }
        }
    }

    fn render(&mut self) {
        match self.cycle {
            // Idle cycle
            0 => {}
            // Data for each tile is fetched, in reality they should be fed to shift registers, but
            // for now we're just rendering them directly
            1..=256 => {
                self.render_pixel();
                self.shift_shift_registers();

                if self.cycle % 8 == 0 {
                    self.increment_coarse_x();
                    self.update_shift_registers();

                    self.nametable_byte = self.nametable_byte();
                    self.attribute_byte = self.read(self.attribute_address());

                    let pattern_table_address = self.pattern_table_address();
                    self.pattern_table_low_byte = self.read(pattern_table_address);
                    self.pattern_table_high_byte = self.read(pattern_table_address + 8);
                }
                if self.cycle == 256 {
                    // Increment effective y scroll
                    self.increment_fine_y();
                }
            }
            257..=320 => {
                // Sprite fetches (8 cycles per sprite)
                //if self.cycle % 8 == 0 {
                //    let sprite_index = (self.cycle - 257) / 8;
                //    if sprite_index < 8 {
                //        let sprite_address = (self.sec_oam[sprite_index as usize * 4 + 1] as u16) << 4;
                //        self.pattern_table_low_byte = self.read(sprite_address as usize);
                //        self.pattern_table_high_byte = self.read(sprite_address as usize + 8);
                //    }
                //}
            }
            321..=336 => {
                // Background fetches for the next scanline
                if self.cycle % 8 == 0 {
                    self.nametable_byte = self.nametable_byte();
                    self.attribute_byte = self.read(self.attribute_address());

                    let pattern_table_address = self.pattern_table_address();
                    self.pattern_table_low_byte = self.read(pattern_table_address);
                    self.pattern_table_high_byte = self.read(pattern_table_address + 8);
                }
            }
            337..=340 => {
                // 2 nametables bytes should be fetched here but they do nothing
            }
            _ => unreachable!("ppu cycle should never be {}", self.cycle),
        }
    }

    fn render_pixel(&mut self) {
        let palette_index = (self.attribute_low_shift_register & 1) | ((self.attribute_low_shift_register & 1) << 1);
        let palette = self.get_palette(palette_index);
        let color_index = (self.pattern_low_shift_register & 1) | ((self.pattern_high_shift_register & 1) << 1);
        let color = SYSTEM_PALLETE[palette[color_index as usize] as usize];
        self.frame.set_pixel(self.cycle, self.scanline, color);
    }

    fn get_palette(&mut self, palette_index: u8) -> [u8; 4] {
        let mut palette: [u8; 4] = [0; 4];
        let background_color = self.read(0x3F00);
        palette[0] = background_color;
        for i in 0..=2 {
            let color = self.read(0x3F01 + 4 * (palette_index as u16) + (i as u16));
            palette[i + 1] = color;
        }
        palette
    }

    fn update_shift_registers(&mut self) {
        self.pattern_low_shift_register |= (self.pattern_table_low_byte as u16) << 8;
        self.pattern_high_shift_register |= (self.pattern_table_high_byte as u16) << 8;

        let low_attribute_bit_index = (self.v & 1) * 2 + ((self.v & (0b1_00000)) >> 5) * 4;
        let high_attribute_bit_index = low_attribute_bit_index + 1;
        //println!("{low_attribute_bit_index}");
        self.attribute_low_bit_latch = (self.attribute_byte & (1 << low_attribute_bit_index)) >> low_attribute_bit_index;
        self.attribute_high_bit_latch = (self.attribute_byte & (1 << high_attribute_bit_index)) >> high_attribute_bit_index;
    }

    fn shift_shift_registers(&mut self) {
        self.pattern_low_shift_register >>= 1;
        self.pattern_high_shift_register >>= 1;

        self.attribute_low_shift_register >>= 1;
        self.attribute_high_shift_register >>= 1;

        // A 1 is shifted into the pattern shift registers, but often gets overwritten by the
        // pattern table byte (this may be unecessary)
        self.pattern_low_shift_register = (self.pattern_low_shift_register & !(1 << 7)) | (1 << 7);
        self.pattern_high_shift_register = (self.pattern_high_shift_register & !(1 << 7)) | (1 << 7);

        // The bit stored in the attribute latches get shifted, and the latch is updated every 8
        // cycles. This makes sure that all each subsequent 8 pixels use the same palette
        //self.attribute_low_shift_register = (self.attribute_low_shift_register & !(1 << 7)) | self.attribute_low_bit_latch << 7;
        //self.attribute_high_shift_register = (self.attribute_high_shift_register & !(1 << 7)) | self.attribute_high_bit_latch << 7;
    }

    fn sprite_evaluation(&mut self) {
        if self.cycle == 65 {
            // Reset flags and addresses at the beginning of evaluation
            self.overflow_detection = false;
            self.oam_address_overflow = false;
            self.sec_oam_address_overflow = false;
            self.sec_oam_address = 0;
        }

        if self.cycle % 2 != 0 {
            // On odd cycles, read data from OAM
            self.oam_data = self.oam[self.oam_address as usize];
            return;
        }

        // Store original OAM data for overflow checking
        let orig_oam_data = self.oam_data;

        // On even cycles, write data into secondary OAM or read from secondary OAM in case of overflow
        if !(self.oam_address_overflow || self.sec_oam_address_overflow) {
            self.sec_oam[self.sec_oam_address] = self.oam_data;
        } else {
            self.oam_data = self.sec_oam[self.sec_oam_address];
        }

        if self.copy_sprite_signal > 0 {
            // Currently copying data for a sprite
            self.copy_sprite_signal -= 1;
            self.move_to_next_oam_byte();
            return;
        }

        // Check if the current sprite is in range
        let sprite_height = if self.control_register.contains(ControlFlags::SpriteSize) {
            16
        } else {
            8
        };
        let in_range = (self.scanline as i16 - orig_oam_data as i16) < sprite_height as i16;

        // At cycle 66, check sprite zero
        //if self.cycle == 66 {
        //    self.s0_on_next_scanline = in_range;
        //}

        if in_range && !(self.oam_address_overflow || self.sec_oam_address_overflow) {
            // In-range sprite found, copy it
            self.copy_sprite_signal = 3;
            self.move_to_next_oam_byte();
            return;
        }

        // Handle cases when sprite is not in range or overflow occurs
        if !self.overflow_detection {
            // Clear low bits and increment high bits
            self.oam_address = (self.oam_address + 4) & 0xFC;
            if self.oam_address == 0 {
                self.oam_address_overflow = true;
            }
        } else if in_range && !self.oam_address_overflow {
            // Set sprite overflow flag
            self.status_register.set(StatusFlags::SpriteOverflow, true);
            self.overflow_detection = false;
        } else {
            // Increment with glitch after exactly eight sprites are found
            self.oam_address = ((self.oam_address + 4) & 0xFC) | ((self.oam_address + 1) & 3);
            if (self.oam_address & 0xFC) == 0 {
                self.oam_address_overflow = true;
            }
        }
    }
    fn move_to_next_oam_byte(&mut self) {
        self.oam_address = (self.oam_address + 1) & 0xFF;
        self.sec_oam_address = (self.sec_oam_address + 1) & 0x1F;

        if self.oam_address == 0 {
            self.oam_address_overflow = true;
        }

        if self.sec_oam_address == 0 {
            self.sec_oam_address_overflow = true;
            // If sec_oam_addr becomes zero, eight sprites have been found, and we
            // enter overflow glitch mode
            self.overflow_detection = true;
        }
    }

    // See here for detail on memory mapping https://www.nesdev.org/wiki/PPU_memory_map
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x0FFF => self.chr_rom[address as usize],
            0x1000..=0x1FFF => self.chr_rom[address as usize],
            0x2000..=0x23FF => self.vram[address as usize - 0x2000], // Nametable 1
            0x2400..=0x2FFF => self.vram[self.nametable_mirroring_address(address) as usize - 0x2000], // Nametable 2 through 4 can be mirrored
            0x3000..=0x3EFF => self.vram[self.nametable_mirroring_address(address & 0x2FFF) as usize - 0x2000],
            0x3F00..=0x3F1F => self.palette_ram[address as usize - 0x3F00],
            0x3F20..=0x3FFF => self.palette_ram[(address as usize & 0x3F1F) - 0x3F00], // Mirror of 0x3F00 to 0x3F1F
            _ => panic!("Address {:X} is outside the ppus memory", address),
        }
    }
    pub fn write(&mut self, address: u16, value: u8) {
        // TODO: I'm pretty sure only vram can be written to so this method might need to change
        match address {
            0x0000..=0x0FFF => self.chr_rom[address as usize] = value,
            0x1000..=0x1FFF => self.chr_rom[address as usize] = value,
            0x2000..=0x23FF => self.vram[address as usize] = value, // Nametable 1
            0x2400..=0x2FFF => self.vram[self.nametable_mirroring_address(address) as usize - 0x2000] = value, // Nametable 2 through 4 can be mirrored
            0x3000..=0x3EFF => self.vram[self.nametable_mirroring_address(address & 0x2FFF) as usize - 0x2000] = value,
            0x3F00..=0x3F1F => self.palette_ram[address as usize] = value,
            0x3F20..=0x3FFF => self.palette_ram[address as usize & 0x3F1F] = value, // Mirror of 0x3F00 to 0x3F1F
            _ => panic!("Address {:X} is outside the ppus memory", address),
        }
    }

    fn nametable_mirroring_address(&self, address: u16) -> u16 {
        //println!("{:X}", address);
        match self.screen_mirroring {
            Mirroring::FourScreen => address,
            Mirroring::SingleScreen => address & 0x23FF,
            Mirroring::Horizontal => match address {
                0x2400..=0x27FF | 0x2C00..=0x2FFF => address - 0x400,
                _ => address,
            },
            Mirroring::Vertical => match address {
                0x2800..=0x2FFF => address - 0x800,
                _ => address,
            },
        }
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct StatusFlags: u8 {
        const PpuOpenBus= 0b0001_1111;
        const SpriteOverflow = 0b0010_0000;
        const SpriteZeroHit = 0b0100_0000;
        const VerticalBlankStarted = 0b1000_0000;
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct ControlFlags: u8 {
        const BaseNametableMask = 0b00000011;
        const VramAddressIncrement = 0b00000100;
        const SpritePatternTableAddress = 0b00001000;
        const BackgroundPatternTableAddress = 0b00010000;
        const SpriteSize = 0b00100000;
        const PpuMasterSlaveSelect = 0b01000000;
        const GenerateNmi = 0b10000000;
    }
}

impl ControlFlags {
    // Method to extract the VRAM address increment value
    pub fn vram_address_increment(&self) -> u8 {
        match *self {
            Self::VramAddressIncrement => 32,
            _ => 1,
        }
    }
}
