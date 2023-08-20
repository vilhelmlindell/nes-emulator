const MEMORY_SIZE: usize = 0x10000;

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn write(&mut self, address: u16, value: u8);
    fn write_word(&mut self, address: u16, value: u16);
    fn write_bytes(&mut self, address: u16, bytes: &[u8]);
}

pub struct MemoryBus {
    memory: [u8; MEMORY_SIZE],
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; MEMORY_SIZE],
        }
    }
}

impl Bus for MemoryBus {
    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address) as u16;
        let high = self.read(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }
    fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.memory[address as usize] = low_byte;
        self.memory[(address.wrapping_add(1)) as usize] = high_byte;
    }
    fn write_bytes(&mut self, address: u16, bytes: &[u8]) {
        self.memory[address as usize..address as usize + bytes.len()].copy_from_slice(bytes);
    }
}
