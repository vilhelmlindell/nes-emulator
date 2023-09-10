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
        match address {
            0x4020..=0x7FFF => self.rom.prg_ram[(address - 0x4020) as usize],
            0x8000..=0xFFFF => {
                // In NROM, the PRG ROM is directly accessible from CPU address space
                let prg_rom_address = if self.rom.prg_rom.len() >= 16384 {
                    (address as usize - 0x8000) % 16384
                } else {
                    address as usize - 0x8000
                };
                self.rom.prg_rom[prg_rom_address]
            }
            // Add other address ranges within the cartridge space here if needed
            _ => {
                // Handle other cases if needed
                0
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        // Excess ram is writable to
        if (0x4020..=0x7FFF).contains(&address) {
            self.rom.prg_ram[(address - 0x4020) as usize] = value;
        }
        // In NROM, PRG ROM is read-only, so no writes are allowed.
    }
}
