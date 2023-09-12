use crate::memory_bus::VRAM_SIZE; // Import VRAM_SIZE from your module

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    registers: AddressRegisters,
}

pub struct AddressRegisters {
    upper_addr: u8, // Upper byte of the PPU address
    lower_addr: u8, // Lower byte of the PPU address
}

impl AddressRegisters {
    pub fn new() -> Self {
        AddressRegisters {
            upper_addr: 0,
            lower_addr: 0,
        }
    }

    pub fn write_upper_byte(&mut self, byte: u8) {
        // Write the upper byte of the PPU address
        self.upper_addr = byte;
    }

    pub fn write_lower_byte(&mut self, byte: u8) {
        // Write the lower byte of the PPU address
        self.lower_addr = byte;
    }

    pub fn as_word(&self) -> u16 {
        // Combine the upper and lower bytes to get the 16-bit address
        u16::from(self.upper_addr) << 8 | u16::from(self.lower_addr)
    }

    pub fn reset_address(&mut self) {
        self.upper_addr = 0;
        self.lower_addr = 0;
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            registers: AddressRegisters::new(),
        }
    }

    pub fn read_ppudata(&mut self) -> u8 {
        let data = self.ppudata_register.read();
        self.ppudata_register.write(self.read_vram(self.registers.get_address()));
        if self.dpcm_sample_in_progress {
            self.registers.increment_address(32); // Skip an extra byte
        } else {
            self.registers.increment_address(1);
        }
        data
    }

    pub fn write_ppudata(&mut self, data: u8) {
        self.write_vram(self.registers.get_address(), data);
        if self.dpcm_sample_in_progress {
            self.registers.increment_address(32); // Skip an extra byte
        } else {
            self.registers.increment_address(1);
        }
    }

    // Add other PPU methods as needed
}
