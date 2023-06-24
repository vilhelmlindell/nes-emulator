use std::intrinsics::wrapping_add;

#[derive(Debug, PartialEq)]
enum StatusFlag {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCommand,
    Unused,
    Overflow,
    Negative,
}

pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

const CPU_OPCODES: Vec<Instruction> = vec![];

impl OpCode {
    pub const fn new(
        opcode: u8,
        instruction: fn(&mut Cpu),
        addressing_mode: AddressingMode,
        bytes: u8,
        cycles: u8,
    ) -> Self {
        Self {
            opcode,
            instruction,
            addressing_mode,
            bytes,
            cycles,
        }
    }
}

pub struct Instruction {
    instruction: fn(&mut Cpu),
    addressing_mode: AddressingMode,
    bytes: u8,
    cycles: u8,
}

pub struct Cpu {
    pc: u16,               // Program counter
    sp: u8,                // Stack pointer
    a: u8,                 // Accumulator
    x: u8,                 // X register
    y: u8,                 // Y register
    status: u8,            // Status register
    memory: [u8; 0x10000], // Memory
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0xFD,
            a: 0,
            x: 0,
            y: 0,
            status: 0x24,
            memory: [0; 0x10000],
        }
    }
    fn reset(&mut self) {
        self.pc = self.read_word(0xFFFC);
        self.sp = 0xFD;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = 0x24;
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_byte(self.pc) as usize;
            self.pc += 1;
            let instruction = CPU_OPCODES[opcode];
            instruction();
        }
    }
    fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn read_word(&self, addr: u16) -> u16 {
        let high = self.read_byte(addr) as u16;
        let low = self.read_byte(addr + 1) as u16;
        (high << 8) | low
    }
    fn write_byte(&mut self) {}
    fn write_word(&mut self) {}
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.read_byte(self.pc) as u16,
            AddressingMode::Absolute => self.read_word(self.pc),
            AddressingMode::ZeroPageX => {
                let addr = self.read_byte(self.pc);
                wrapping_add(addr, self.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = self.read_byte(self.pc);
                wrapping_add(addr, self.y) as u16
            }
            AddressingMode::AbsoluteX => {
                let addr = self.read_word(self.pc);
                wrapping_add(addr, self.x as u16)
            }
            AddressingMode::AbsoluteY => {
                let addr = self.read_word(self.pc);
                wrapping_add(addr, self.y as u16)
            }
            AddressingMode::IndirectX => {
                let addr = self.read_byte(self.pc);
                let addr = wrapping_add(addr, self.x) as u16;
                let low = self.read_byte(addr) as u16;
                let high = self.read_byte(wrapping_add(addr, 1)) as u16;
                (high << 8) | low
            }
            AddressingMode::IndirectY => {
                let addr = self.read_byte(self.pc) as u16;
                let low = self.read_byte(addr);
                let high = self.read_byte(wrapping_add(addr, 1));
                let addr = (high << 8) | low;
                wrapping_add(addr, self.y) as u16
            }
            AddressingMode::NoneAddressing => panic!("Mode doesn't support addresses"),
        }
    }

    // Instructions
    pub fn adc(&self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        // Overflow
        if self.a.checked_add(operand) == None {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cpu_reset() {
        let mut cpu = Cpu::new();
        cpu.reset();
        assert_eq!(cpu.pc, 0xFFFC);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.status, 0x24);
    }
    #[test]
    fn test_cpu_adc() {
        let mut cpu = Cpu::new();
        cpu.a = 0x01;
        cpu.adc(0x02);
        assert_eq!(cpu.a, 0x03);
        assert_eq!(cpu.status & FLAG_CARRY, 0);
        assert_eq!(cpu.status & FLAG_ZERO, 0);
        assert_eq!(cpu.status & FLAG_NEGATIVE, 0);
        cpu.a = 0xFF;
        cpu.adc(0x01);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.status & FLAG_CARRY, FLAG_CARRY);
        assert_eq!(cpu.status & FLAG_ZERO, FLAG_ZERO);
        assert_eq!(cpu.status & FLAG_NEGATIVE, 0);
    }
}
