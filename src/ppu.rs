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
    control: ControlFlags,
    status: StatusFlags,
    chr_rom: Vec<u8>,
    screen_mirroring: Mirroring,

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
    secondary_oam: [u8; 32],
    secondary_oam_address: usize,
    sprite_evaluation_state: SpriteEvaluationState,
    cycles_to_wait: u32,
    secondary_oam_writes_disabled: bool,
    n: usize,
    m: usize,
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
            control: ControlFlags::empty(),
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
            secondary_oam: todo!(),
            status: todo!(),
            oam: todo!(),
            oam_address: todo!(),
            secondary_oam_address: todo!(),
            sprite_evaluation_state: todo!(),
            cycles_to_wait: todo!(),
            secondary_oam_writes_disabled: todo!(),
            n: todo!(),
            m: todo!(),
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
        self.vram_address.increment(self.control.vram_address_increment());
    }

    pub fn write_control(&mut self, value: u8) {
        self.control = ControlFlags::from_bits_truncate(value);
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

    fn nametable_byte(&self) -> u8 {
        let address = 0x2000 | (self.vram_address.address() & 0x0FFF);
        self.read(address)
    }

    fn pattern_table_address(&self) -> u16 {
        let bit_plane = 0b1000;
        let fine_y = (self.vram_address.address() & 0x7000) >> 12;
        let address = bit_plane | (self.nametable_byte() << 4) as u16 | fine_y as u16;
        address
    }

    fn attribute_table_byte(&self) -> u8 {
        let address = 0b10001111000000
            | (self.vram_address.address() & 0b110000000000)
            | (self.vram_address.address() >> 2)
            | (self.vram_address.address() >> 4) & 0b111000;
        self.read(address)
    }
    pub fn write_oam_dma(&self, oam_data: &[u8; 256]) {}

    pub fn step(&mut self) {
        let x = self.cycle - 1;
        let y = self.scanline;

        match self.cycle {
            0 => {}
            1..=256 => {
                if self.cycle % 8 == 0 {
                    let pattern_table_address = self.pattern_table_address() as usize;
                    let nametable_byte = self.nametable_byte();
                    let attribute_byte = self.attribute_table_byte();
                    let pattern_table_low_byte = self.chr_rom[pattern_table_address];
                    let pattern_table_high_byte = self.chr_rom[pattern_table_address + 8];
                    let tile_n = self.vram_address.address() & 0b1111111111;

                    let tile_x = tile_n % 32;
                    let tile_y = tile_n / 32;
                    let quad_x = tile_x % 2;
                    let quad_y = tile_y % 2;
                    let quad = quad_x | (quad_y << 1);
                    let palette_index = (attribute_byte >> quad) & 0b11;
                    let palette = self.palettes[palette_index as usize];

                    for i in 0..8 {
                        let value = (pattern_table_low_byte & (1 << x)) | ((pattern_table_high_byte & (1 << x)) << 1);
                        let color = palette[value as usize];
                        let pixel_x = x + i;
                        let pixel_y = y + i as u8;
                        self.frame.set_pixel(pixel_x as usize, pixel_y as usize, color);
                    }
                }
            }
            257..=320 => {}
            337..=340 => {}
        }

        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
            }
        }
    }

    fn sprite_evaluation(&self) {
        if self.cycle == 65 {
            self.secondary_oam_address = 0;
        }

        if self.cycles_to_wait != 0 {
            self.cycles_to_wait -= 1;
            return;
        }

        if self.cycle % 2 != 0 {
            self.oam_data = self.oam[self.oam_address as usize];
            return;
        }

        let sprite_height = if self.control.contains(ControlFlags::SpriteSize) { 16 } else { 8 };

        match self.sprite_evaluation_state {
            SpriteEvaluationState::Copying => {
                let y = self.oam[self.n * 4];

                if !(self.secondary_oam_address == 8 || self.n == 64) {
                    self.secondary_oam[self.secondary_oam_address * 4] = y
                }

                let is_in_range = (y >= self.scanline) && (y + sprite_height) <= self.scanline;

                if !is_in_range {
                    self.sprite_evaluation_state = SpriteEvaluationState::Evaluation;
                }

                if self.cycle % 2 == 0 {
                    self.secondary_oam[self.secondary_oam_address * 4 + self.m] = self.oam[self.n * 4 + self.m]
                }

                // This implementation wont be cycle accurate since it copies before
                //self.secondary_oam[(self.secondary_oam_address * 4 + 1)..=(self.secondary_oam_address * 4 + 3)]
                //    .copy_from_slice(&self.oam[(self.n * 4 + 1)..=(self.n * 4 + 3)]);

                //self.secondary_oam[(self.secondary_oam_address * 4 + 1)..=(self.secondary_oam_address * 4 + 3)]
                //    .copy_from_slice(&self.oam[(self.n * 4 + 1)..=(self.n * 4 + 3)]);
            }
            SpriteEvaluationState::Evaluation => {
                self.cycles_to_wait = 2;

                self.n += 1;

                // All 64 sprites have been evaluated
                if self.n == 64 {
                    self.secondary_oam_writes_disabled = true;
                    self.sprite_evaluation_state = SpriteEvaluationState::Copying;
                }
                // Fewer than 8 sprites have been found
                if self.secondary_oam_address < 8 {
                    self.sprite_evaluation_state = SpriteEvaluationState::Copying;
                }
                // Exactly 8 sprites have been found
                if self.secondary_oam_address == 8 {
                    self.secondary_oam_writes_disabled = true;
                    self.sprite_evaluation_state = SpriteEvaluationState::OverflowLogic;
                }
            }
            SpriteEvaluationState::OverflowLogic => {
                let y = self.oam[self.n * 4 + self.m];

                let is_in_range = (y >= self.scanline) && (y + sprite_height) <= self.scanline;

                if is_in_range {
                    self.cycles_to_wait = 7;

                    self.status.set(StatusFlags::SpriteOverflow, true);
                } else {
                }
            }
        }
    }
}

impl Mapper for Ppu {
    fn read(&self, address: u16) -> u8 {
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
