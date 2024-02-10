pub trait Mapper {
    fn read(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }
    fn write(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.write(address, low_byte);
        self.write(address.wrapping_add(1), high_byte);
    }
}

pub struct NromMapper {
    prg_rom: Vec<u8>,
    prg_ram: [u8; 8192],
}

impl NromMapper {
    pub fn new(prg_rom: Vec<u8>, prg_ram: [u8; 8192]) -> Self {
        NromMapper { prg_rom, prg_ram }
    }
}

impl Mapper for NromMapper {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x4020..=0x7FFF => self.prg_ram[(address - 0x4020) as usize],
            0x8000..=0xFFFF => {
                // In NROM, the PRG ROM is directly accessible from CPU address space
                let prg_rom_address = (address as usize - 0x8000) % 16384;
                self.prg_rom[prg_rom_address]
            }
            _ => {
                unreachable!("Mapper should not handle this address");
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        // Excess ram is writable to
        match address {
            0x4020..=0x7FFF => {
                self.prg_ram[(address - 0x4020) as usize] = value;
            }
            0x8000..=0xFFFF => {
                panic!("Cartridge ROM space is not writable to with the NromMapper");
            }
            _ => {
                unreachable!("Mapper should not handle this address");
            }
        }
    }
}
