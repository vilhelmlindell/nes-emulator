use crate::mapper::{Mapper, NromMapper};
use crate::ppu::Ppu;
use crate::rom::Rom;

const RAM_MIRRORS_END: u16 = 0x1FFF;
const RAM_MIRROR_MASK: u16 = 0x0800 - 1;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn write(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16);
}

pub struct MemoryBus {
    cpu_vram: [u8; 2048],
    ppu: Ppu,
    apu_io_registers: [u8; 0x20],
    mapper: Box<dyn Mapper>,
}

impl MemoryBus {
    pub fn new(rom: Rom) -> Self {
        let mapper: Box<dyn Mapper> = match rom.mapper {
            0 => Box::new(NromMapper::new(rom)), // Use NROM mapper for mapper number 0
            // Add cases for other mappers as needed
            _ => unimplemented!("Mapper {} not implemented", rom.mapper),
        };

        MemoryBus {
            cpu_vram: [0; 2048],
            ppu: Ppu::new(),
            apu_io_registers: [0; 32],
            mapper,
        }
    }
}

impl Bus for MemoryBus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => self.cpu_vram[address as usize],
            0x0800..=0x1FFF => {
                let mirrored_down_address = address & 0x07FF;
                self.read(mirrored_down_address)
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Address {} is a write only PPU register and read", address);
            }
            0x4000..=0x401F => {
                // NES APU and I/O registers and their functionality
                self.apu_io_registers[(address - 0x4000) as usize]
            }
            0x4020..=0xFFFF => {
                // Cartridge space: PRG ROM, PRG RAM, and mapper registers
                self.mapper.read(address)
            }
        }
    }
    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // 2 KB internal RAM mirrored every 0x0800 bytes
                let mirror_down_address = address & RAM_MIRROR_MASK; // Same as module 0x800
                self.cpu_vram[mirror_down_address as usize] = value;
            }
            0x2000 => {}
            0x2008..=0x3FFF => {
                // Mirrors of $2000–$2007 (repeats every 8 bytes)
                let mirror_down_address = (address & 0x2007) % 0x800;
                self.write(mirror_down_address, value);
            }
            0x4000..=0x401F => {
                // NES APU and I/O registers and their functionality
                self.apu_io_registers[(address - 0x4000) as usize] = value;
            }
            0x4020..=0xFFFF => {
                // Cartridge space: PRG ROM, PRG RAM, and mapper registers
                self.mapper.write(address, value);
            }
        }
    }
    fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.write(address, low_byte);
        self.write(address.wrapping_add(1), high_byte);
    }
}
