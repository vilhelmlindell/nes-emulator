use bitflags::bitflags;

use crate::rom::{Mirroring, Rom};

const VRAM_SIZE: usize = 2048;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    vram_address: PpuAddress,
    control: ControlFlags,
    chr_rom: Vec<u8>,
    screen_mirroring: Mirroring,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, screen_mirroring: Mirroring) -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            vram_address: PpuAddress::new(),
            control: ControlFlags::empty(),
            chr_rom,
            screen_mirroring,
        }
    }

    pub fn read_ppudata(&self) -> u8 {
        let address = self.vram_address.get_address() as usize;
        // Read data from the VRAM buffer at the current VRAM address
        self.vram[address]
    }

    pub fn write_ppudata(&mut self, value: u8) {
        let address = self.vram_address.get_address() as usize;
        self.vram[address] = value;
        self.vram_address.increment(self.control.vram_address_increment());
    }

    pub fn write_control(&mut self, value: u8) {
        self.control = ControlFlags::from_bits_truncate(value);
    }
}

pub struct PpuAddress {
    high_byte: u8,
    low_byte: u8,
    is_high_byte: bool,
}

impl PpuAddress {
    pub fn new() -> Self {
        PpuAddress {
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

    pub fn get_address(&self) -> u16 {
        u16::from_be_bytes([self.high_byte, self.low_byte])
    }

    pub fn increment(&mut self, value: u8) {
        let current_address = self.get_address();
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
