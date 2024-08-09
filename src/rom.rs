#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    SingleScreen,
    FourScreen,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub prg_ram: [u8; 8192],
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(bytes: &[u8]) -> Result<Rom, String> {
        if bytes[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
            return Err("File is not in iNES file format".to_string());
        }
        if bytes[7] & 0b1100 == 0b1000 {
            return Err("NES 2.0 format is not supported".to_string());
        }

        let has_trainer = bytes[6] & 0b100 != 0;

        let mirroring = if bytes[6] & 0b1000 != 0 {
            Mirroring::FourScreen
        } else if bytes[6] & 0b1 == 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let prg_rom_size = bytes[4] as usize * 16384;
        let chr_rom_size = bytes[5] as usize * 8192;

        let prg_rom_start = (16 + if has_trainer { 512 } else { 0 }) as usize;
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let mapper_number = (bytes[7] & 0b1111_0000) | bytes[6] >> 4;

        Ok(Self {
            prg_rom: bytes[prg_rom_start..prg_rom_start + prg_rom_size].to_vec(),
            chr_rom: bytes[chr_rom_start..chr_rom_start + chr_rom_size].to_vec(),
            prg_ram: [0; 8192],
            mapper: mapper_number,
            screen_mirroring: mirroring,
        })
    }
}

impl Default for Rom {
    fn default() -> Self {
        let mut bytes = vec![0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01];
        bytes.resize(24592, 0x00);
        Self::new(&bytes).expect("Failed to create default rom")
    }
}
