use crate::rom::Rom;

pub trait Mapper {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct NromMapper {
    rom: Rom, // Store the ROM data
}

impl NromMapper {
    pub fn new(rom: Rom) -> Self {
        NromMapper { rom }
    }
}

impl Mapper for NromMapper {
    fn read(&self, address: u16) -> u8 {
        // In NROM, the PRG ROM is directly accessible from CPU address space
        if address >= 0x8000 {
            let prg_rom_address = address as usize - 0x8000;
            self.rom.prg_rom[prg_rom_address]
        } else {
            // Handle other cases if needed
            0
        }
    }

    fn write(&mut self, _address: u16, _value: u8) {
        // In NROM, PRG ROM is read-only, so no writes are allowed.
    }
}
