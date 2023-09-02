use crate::mapper::{Mapper, NromMapper};
use crate::rom::Rom;

const RAM_MIRRORS_END: u16 = 0x1FFF;
const RAM_MIRROR_MASK: u16 = 0x0800 - 1;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

const RAM_SIZE: usize = 2048;

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn write(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16);
}

pub struct MemoryBus {
    vram: [u8; RAM_SIZE],
    mapper: Box<dyn Mapper>,
}

impl MemoryBus {
    pub fn new(rom: Rom) -> Self {
        let mapper: Box<dyn Mapper> = match rom.mapper {
            0 => Box::new(NromMapper::new(rom)), // Use NROM mapper for mapper number 0
            // Add cases for other mappers as needed
            _ => unimplemented!("Mapper {} not implemented", rom.mapper),
        };

        MemoryBus { vram: [0; RAM_SIZE], mapper }
    }
}

impl Bus for MemoryBus {
    fn read(&self, address: u16) -> u8 {
        // First, try to read from RAM
        if address <= RAM_MIRRORS_END {
            let mirror_down_address = address & RAM_MIRROR_MASK; // Same as module 0x800
            self.vram[mirror_down_address as usize]
        } else {
            // If not in RAM range, delegate to the mapper
            self.mapper.read(address)
        }
    }
    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    fn write(&mut self, address: u16, value: u8) {
        // First, try to write to RAM
        if address <= RAM_MIRRORS_END {
            let mirror_down_address = address & RAM_MIRROR_MASK; // Same as module 0x800
            self.vram[mirror_down_address as usize] = value;
        } else {
            // If not in RAM range, delegate to the mapper
            self.mapper.write(address, value);
        }
    }
    fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.write(address, low_byte);
        self.write(address.wrapping_add(1), high_byte);
    }
}
