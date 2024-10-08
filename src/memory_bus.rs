use bitflags::Flags;

use crate::mapper::{Mapper, NromMapper};
use crate::ppu::Ppu;
use crate::rom::Rom;

const RAM_MIRRORS_END: u16 = 0x1FFF;
const RAM_MIRROR_MASK: u16 = 0x0800 - 1;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

const PPUCTRL: u16 = 0x2000;
const PPUMASK: u16 = 0x2001;
const PPUSTATUS: u16 = 0x2002;
const OAMADDR: u16 = 0x2003;
const OAMDATA: u16 = 0x2004;
const PPUSCROLL: u16 = 0x2005;
const PPUADDR: u16 = 0x2006;
const PPUDATA: u16 = 0x2007;
const OAMDMA: u16 = 0x4014;

pub struct MemoryBus {
    pub cpu_vram: [u8; 2048],
    pub ppu: Ppu,
    pub apu_io_registers: [u8; 0x20],
    pub mapper: Box<dyn Mapper>,
}

impl MemoryBus {
    pub fn new(rom: Rom) -> Self {
        let mapper: Box<dyn Mapper> = match rom.mapper {
            0 => Box::new(NromMapper::new(rom.prg_rom, rom.prg_ram)), // Use NROM mapper for mapper number 0
            // Add cases for other mappers as needed
            _ => unimplemented!("Mapper {} not implemented", rom.mapper),
        };

        MemoryBus {
            cpu_vram: [0; 2048],
            ppu: Ppu::new(rom.chr_rom, rom.screen_mirroring),
            apu_io_registers: [0; 32],
            mapper,
        }
    }

    pub fn debug_read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => self.cpu_vram[address as usize],
            0x0800..=0x1FFF => {
                let mirrored_down_address = address & 0x07FF;
                self.debug_read(mirrored_down_address)
            }
            PPUSTATUS => self.ppu.status_register.bits(),
            OAMDATA => self.ppu.oam_data,
            PPUDATA => self.ppu.read(self.ppu.v),
            PPUCTRL | PPUMASK | OAMADDR | PPUSCROLL | PPUADDR | OAMDMA => {
                panic!("Address {} is a write only PPU register and reading from it is not allowed", address);
            }
            0x2008..=0x3FFF => {
                let mirror_down_address = (address & 0x2007) % 0x800;
                self.debug_read(mirror_down_address)
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

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => self.cpu_vram[address as usize],
            0x0800..=0x1FFF => {
                let mirrored_down_address = address & 0x07FF;
                self.read(mirrored_down_address)
            }
            PPUSTATUS => self.ppu.read_status().bits(),
            OAMDATA => self.ppu.read_oam_data(),
            PPUDATA => self.ppu.read_ppudata(),
            PPUCTRL | PPUMASK | OAMADDR | PPUSCROLL | PPUADDR | OAMDMA => {
                panic!("Address {} is a write only PPU register and reading from it is not allowed", address);
            }
            0x2008..=0x3FFF => {
                let mirror_down_address = (address & 0x2007) % 0x800;
                self.read(mirror_down_address)
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

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // 2 KB internal RAM mirrored every 0x0800 bytes
                let mirror_down_address = address & RAM_MIRROR_MASK; // Same as module 0x800
                self.cpu_vram[mirror_down_address as usize] = value;
            }
            PPUCTRL => self.ppu.write_control(value),
            PPUMASK => {}
            PPUSTATUS => {
                panic!("Address {} is a read only PPU register and writing to it is not allowed", address);
            }
            OAMADDR => {}
            OAMDATA => {}
            PPUSCROLL => {
                self.ppu.write_scroll(value);
            }
            PPUADDR => self.ppu.write_ppuaddr(value),
            PPUDATA => {
                self.ppu.write_ppudata(value);
            }
            OAMDMA => {
                let oam_data_slice = &self.cpu_vram[0x0200..0x02FF];
                let oam_data: &[u8; 256] = oam_data_slice.try_into().expect("slice with incorrect length");
                self.ppu.write_oam_dma(oam_data);
            }
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

    pub fn read_word(&mut self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.write(address, low_byte);
        self.write(address.wrapping_add(1), high_byte);
    }
}
