use bitflags::{bitflags, Flags};

use crate::{mapper::Mapper, rom::Mirroring};

const VRAM_SIZE: usize = 2048;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    vram_address: VramAddress,
    control: ControlFlags,
    chr_rom: Vec<u8>,
    screen_mirroring: Mirroring,

    cycle: u32,
    scanline: u32,
    frame: Frame,
    frame: u32,
    //v: u16,
    //t: u16,
    //x: u8,
    x_scroll: u8,
    y_scroll: u8,
    w: bool,
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
            frame: 0,
        }
    }

    pub fn read_data(&self) -> u8 {
        let address = self.vram_address.address() as usize;
        // Read data from the VRAM buffer at the current VRAM address
        self.vram[address]
    }

    pub fn write_data(&mut self, value: u8) {
        let address = self.vram_address.address() as usize;
        self.vram[address] = value;
        self.vram_address.increment(self.control.vram_address_increment());
    }

    pub fn write_control(&mut self, value: u8) {
        self.control = ControlFlags::from_bits_truncate(value);
    }

    pub fn write_scroll(&mut self, value: u8) {
        if self.w {
            // First write (vertical scroll value)
            self.t = (self.t & 0x73FF) | ((value as u16 & 0x07) << 12); // t: ....FED CBA.. .... = d: ..HG FEDC
            self.t = (self.t & 0x0C1F) | ((value as u16 & 0xF8) << 2); // t: .HG. .... .CBA = d: BA98 7654
            self.w = false;
        } else {
            // Second write (horizontal scroll value)
            self.t = (self.t & 0x7FE0) | (value as u16 >> 3); // t: .... .... .HG. FEDC BA98 = d: HGFE DCBA
            self.x = value & 0x07; // x: .... .HG. = d: .... ...HGF
            self.w = true;
        }
    }

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

    pub fn step(&mut self) {
        match self.cycle {
            0 => {}
            1..=256 => {
                let pattern_table_address = self.pattern_table_address() as usize;
                let low_bytes = &self.chr_rom[pattern_table_address..(pattern_table_address + 8)];
                let high_bytes = &self.chr_rom[pattern_table_address..(pattern_table_address + 8)];
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
                self.frame += 1;
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
    pub struct ControlFlags: u8 {
        const BaseNametableMask = 0b00000011;
        const VRAMAddressIncrement = 0b00000100;
        const SpritePatternTableAddress = 0b00001000;
        const BackgroundPatternTableAddress = 0b00010000;
        const SpriteSize = 0b00100000;
        const PPUMasterSlaveSelect = 0b01000000;
        const GenerateNMI = 0b10000000;
    }
}

impl ControlFlags {
    // Method to extract the VRAM address increment value
    pub fn vram_address_increment(&self) -> u8 {
        match *self {
            Self::VRAMAddressIncrement => 32,
            _ => 1,
        }
    }
}
