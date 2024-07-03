use bitflags::bitflags;

use crate::frame::Frame;
use crate::{mapper::Mapper, rom::Mirroring};

const VRAM_SIZE: usize = 2048;

enum SpriteEvaluationState {
    Copying,
    Evaluation,
    OverflowLogic,
}

type Rgb = (u8, u8, u8);

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    vram_address: VramAddress,
    chr_rom: Vec<u8>,
    screen_mirroring: Mirroring,

    control_register: ControlFlags,
    status_register: StatusFlags,

    cycle: u16,
    scanline: u8,

    nametable_byte: u8,
    attribute_byte: u8,
    pattern_table_low_byte: u8,
    pattern_table_high_byte: u8,

    frame: Frame,
    palettes: [[Rgb; 4]; 8],

    oam: [u8; 256],
    oam_address: u8,
    oam_data: u8,
    sec_oam: [u8; 32], // 32-byte buffer for current sprites on scanline
    sec_oam_address: usize,
    sec_oam_writes_disabled: bool,
    copy_sprite_signal: i32,
    oam_address_overflow: bool,
    sec_oam_address_overflow: bool,
    overflow_detection: bool,
    //v: u16,
    //t: u16,
    //x: u8,
    //x_scroll: u8,
    //y_scroll: u8,
    //w: bool,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, screen_mirroring: Mirroring) -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            vram_address: VramAddress::new(),
            control_register: ControlFlags::empty(),
            chr_rom,
            screen_mirroring,
            cycle: 0,
            scanline: 0,
            nametable_byte: todo!(),
            attribute_byte: todo!(),
            pattern_table_low_byte: todo!(),
            pattern_table_high_byte: todo!(),
            frame: Frame::default(),
            palettes: todo!(),
            oam_data: todo!(),
            sec_oam: todo!(),
            status_register: todo!(),
            oam: todo!(),
            oam_address: todo!(),
            sec_oam_address: todo!(),
            sec_oam_writes_disabled: todo!(),
            copy_sprite_signal: todo!(),
            oam_address_overflow: todo!(),
            sec_oam_address_overflow: todo!(),
            overflow_detection: todo!(),
        }
    }

    pub fn read_ppu_data(&self) -> u8 {
        let address = self.vram_address.address() as usize;
        // Read data from the VRAM buffer at the current VRAM address
        self.vram[address]
    }

    pub fn write_ppu_data(&mut self, value: u8) {
        let address = self.vram_address.address() as usize;
        self.vram[address] = value;
        self.vram_address.increment(self.control_register.vram_address_increment());
    }

    pub fn write_control(&mut self, value: u8) {
        self.control_register = ControlFlags::from_bits_truncate(value);
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

    //pub fn write_scroll(&mut self, value: u8) {
    //    if self.w {
    //        // First write (vertical scroll value)
    //        self.t = (self.t & 0x73FF) | ((value as u16 & 0x07) << 12); // t: ....FED CBA.. .... = d: ..HG FEDC
    //        self.t = (self.t & 0x0C1F) | ((value as u16 & 0xF8) << 2); // t: .HG. .... .CBA = d: BA98 7654
    //        self.w = false;
    //    } else {
    //        // yecond write (horizontal scroll value)
    //        self.t = (self.t & 0x7FE0) | (value as u16 >> 3); // t: .... .... .HG. FEDC BA98 = d: HGFE DCBA
    //        self.x = value & 0x07; // x: .... .HG. = d: .... ...HGF
    //        self.w = true;
    //    }
    //}

    fn nametable_byte(&mut self) -> u8 {
        let address = 0x2000 | (self.vram_address.address() & 0x0FFF);
        self.read(address)
    }

    fn pattern_table_address(&mut self) -> u16 {
        let bit_plane = 0b1000;
        let fine_y = (self.vram_address.address() & 0x7000) >> 12;
        let address = bit_plane | (self.nametable_byte() << 4) as u16 | fine_y as u16;
        address
    }

    fn attribute_table_byte(&mut self) -> u8 {
        let address = 0b10001111000000
            | (self.vram_address.address() & 0b110000000000)
            | (self.vram_address.address() >> 2)
            | (self.vram_address.address() >> 4) & 0b111000;
        self.read(address)
    }
    pub fn write_oam_dma(&mut self, oam_data: &[u8; 256]) {}

    pub fn step(&mut self) {
        self.render();
        self.sprite_evaluation();

        match self.cycle {}

        // Increment the cycle and handle the end of scanlines and frames
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                // At the end of the frame, perform VBlank operations
                self.status_register.insert(StatusFlags::VerticalBlankStarted);
            }
        }
    }

    fn render(&mut self) {
        let x = self.cycle - 1;
        let y = self.scanline;

        match self.cycle {
            0 => {}
            1..=256 => {
                // Background rendering
                if self.cycle % 8 == 0 {
                    self.nametable_byte = self.nametable_byte();
                    self.attribute_byte = self.attribute_table_byte();

                    let pattern_table_address = self.pattern_table_address() as usize;
                    self.pattern_table_low_byte = self.chr_rom[pattern_table_address];
                    self.pattern_table_high_byte = self.chr_rom[pattern_table_address + 8];

                    let tile_n = self.vram_address.address() & 0b1111111111;
                    let tile_x = tile_n % 32;
                    let tile_y = tile_n / 32;
                    let quad_x = tile_x % 2;
                    let quad_y = tile_y % 2;
                    let quad = quad_x | (quad_y << 1);
                    let palette_index = (self.attribute_byte >> quad) & 0b11;
                    let palette = self.palettes[palette_index as usize];

                    for i in 0..8 {
                        let bit = 7 - (self.cycle % 8);
                        let value = ((self.pattern_table_low_byte >> bit) & 1) | (((self.pattern_table_high_byte >> bit) & 1) << 1);
                        let color = palette[value as usize];
                        let pixel_x = x + i;
                        let pixel_y = y;
                        self.frame.set_pixel(pixel_x as usize, pixel_y as usize, color);
                    }
                }
            }
            257..=320 => {
                // Sprite fetches (8 cycles per sprite)
                if self.cycle % 8 == 0 {
                    let sprite_index = (self.cycle - 257) / 8;
                    if sprite_index < 8 {
                        let sprite_address = (self.sec_oam[sprite_index as usize * 4 + 1] as u16) << 4;
                        self.pattern_table_low_byte = self.chr_rom[sprite_address as usize];
                        self.pattern_table_high_byte = self.chr_rom[sprite_address as usize + 8];
                    }
                }
            }
            337..=340 => {
                // Background fetches for the next scanline
                if self.cycle % 8 == 0 {
                    let pattern_table_address = self.pattern_table_address() as usize;
                    self.pattern_table_low_byte = self.chr_rom[pattern_table_address];
                    self.pattern_table_high_byte = self.chr_rom[pattern_table_address + 8];
                }
            }
            _ => {}
        }
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
            self.sec_oam[self.sec_oam_address as usize] = self.oam_data;
        } else {
            self.oam_data = self.sec_oam[self.sec_oam_address as usize];
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
}

impl Mapper for Ppu {
    fn read(&mut self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        todo!()
    }
}

pub struct VramAddress {
    high_byte: u8,
    low_byte: u8,
    is_high_byte: bool,
}

impl VramAddress {
    pub fn new() -> Self {
        VramAddress {
            high_byte: 0,
            low_byte: 0,
            is_high_byte: true,
        }
    }

    pub fn set_byte(&mut self, byte: u8) {
        if self.is_high_byte {
            // Mirror down addresses above 0x3FFF
            self.high_byte = byte & 0x3F;
        } else {
            self.low_byte = byte;
        }
        self.is_high_byte = !self.is_high_byte;
    }

    pub fn address(&self) -> u16 {
        u16::from_be_bytes([self.high_byte, self.low_byte])
    }

    pub fn increment(&mut self, value: u8) {
        let current_address = self.address();
        let new_address = current_address.wrapping_add(value as u16);
        self.high_byte = (new_address >> 8) as u8;
        self.low_byte = new_address as u8;
    }

    pub fn reset_latch(&mut self) {
        self.is_high_byte = true;
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct StatusFlags: u8 {
        const PpuOpenBus= 0b0001_1111;
        const SpriteOverflow = 0b0010_0000;
        const SpriteZeroHit = 0b0100_0000;
        const VerticalBlankStarted = 0b1000_0000;
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
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
