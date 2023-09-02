use crate::memory_bus::{self, Bus, MemoryBus};
use crate::opcodes::{OpCode, CPU_OPCODES};
use crate::rom::Rom;
use std::intrinsics::wrapping_add;

const SP_START: u8 = 0xFD;
const RESET_VECTOR: u16 = 0xFFFC;

pub struct Cpu {
    pub pc: u16,    // Program counter
    pub sp: u8,     // Stack pointer
    pub a: u8,      // Accumulator
    pub x: u8,      // X register
    pub y: u8,      // Y register
    pub status: u8, // Status register
    pub cycles: u32,
    pub memory_bus: MemoryBus, // Memory
}

impl Cpu {
    pub fn new(memory_bus: MemoryBus) -> Cpu {
        Cpu {
            pc: 0,
            sp: SP_START,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            cycles: 0,
            memory_bus,
        }
    }
    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = SP_START;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.cycles = 0;
    }
    pub fn execute(&mut self, opcode: usize) {
        let opcode = CPU_OPCODES[opcode].clone().expect(&format!("Invalid opcode: {:X}", opcode));

        self.pc += 1;
        (opcode.instruction)(self, &opcode.addressing_mode);
        self.cycles += opcode.cycles as u32;
    }
    pub fn run(&mut self) {
        loop {
            self.step(false);
        }
    }

    pub fn step(&mut self, debug: bool) {
        let opcode_index = self.read(self.pc) as usize;
    }
    fn print_instruction_state(&self, instruction: &OpCode) {
        print!("{:X} ", self.pc);
        for i in 0..instruction.bytes as u16 {
            print!("{:X} ", self.read(self.pc.wrapping_add(i)));
        }
        print!("{} ", instruction.name);
        print!("A={:X} X={:X} Y={:X} ", self.a, self.x, self.y);

        let letters = "CZIDB-VN";
        let mut result = String::new();

        for i in 0..8 {
            let mask = 1 << i;
            if self.status & mask != 0 {
                result.insert(0, letters.chars().nth(i).unwrap());
            } else {
                result.insert(0, '-');
            }
        }

        print!("PS=[{}] ", result);
        print!("SP={:X} ", self.sp);

        println!();
    }

    // Stack operations
    pub fn push(&mut self, value: u8) {
        self.write(self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }
    pub fn push_word(&mut self, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.push(high_byte);
        self.push(low_byte);
    }
    pub fn pull(&mut self) -> u8 {
        let value = self.read(self.sp as u16);
        self.sp = self.sp.wrapping_add(1);
        value
    }
    pub fn pull_word(&mut self) -> u16 {
        let low_byte = self.pull() as u16;
        let high_byte = self.pull() as u16;

        (high_byte << 8) | low_byte
    }

    pub fn status(&self, flag: ProcessorStatus) -> bool {
        let flag_bit = 1 << flag as u8;
        (self.status & flag_bit) != 0
    }
    pub fn set_status(&mut self, flag: ProcessorStatus, value: bool) {
        let flag_bit = 1 << flag as u8;
        if value {
            self.status |= flag_bit; // Set the flag
        } else {
            self.status &= !flag_bit; // Clear the flag
        }
    }
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_status(ProcessorStatus::Zero, true);
        } else {
            self.set_status(ProcessorStatus::Zero, false);
        }

        if result & 0b1000_0000 != 0 {
            self.set_status(ProcessorStatus::Negative, true);
        } else {
            self.set_status(ProcessorStatus::Negative, false);
        }
    }
    pub fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        let address = match mode {
            AddressingMode::Immediate | AddressingMode::Relative => self.pc,
            AddressingMode::ZeroPage => self.read(self.pc) as u16,
            AddressingMode::Absolute => self.read_word(self.pc),
            AddressingMode::ZeroPageX => {
                let addr = self.read(self.pc);
                wrapping_add(addr, self.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = self.read(self.pc);
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
                let addr = self.read(self.pc);
                let addr = wrapping_add(addr, self.x) as u16;
                let low = self.read(addr) as u16;
                let high = self.read(wrapping_add(addr, 1)) as u16;
                (high << 8) | low
            }
            AddressingMode::IndirectY => {
                let addr = self.read(self.pc);
                let addr = wrapping_add(addr, self.y) as u16;
                let low = self.read(addr) as u16;
                let high = self.read(wrapping_add(addr, 1)) as u16;
                (high << 8) | low
            }
            AddressingMode::NoneAddressing => {
                panic!("Mode doesn't support addresses")
            }
        };
        self.pc = self.pc.wrapping_add(mode.byte_count());
        address
    }
}

impl Bus for Cpu {
    fn read(&self, address: u16) -> u8 {
        self.memory_bus.read(address)
    }
    fn read_word(&self, address: u16) -> u16 {
        self.memory_bus.read_word(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory_bus.write(address, value);
    }
    fn write_word(&mut self, address: u16, value: u16) {
        self.memory_bus.write_word(address, value);
    }
}

#[derive(Debug, PartialEq)]
pub enum ProcessorStatus {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    DecimalMode = 3,
    Break = 4,
    Overflow = 6,
    Negative = 7,
}

#[derive(PartialEq, Clone)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

impl AddressingMode {
    pub fn byte_count(&self) -> u16 {
        match self {
            Self::Immediate | Self::ZeroPage | Self::ZeroPageX | Self::ZeroPageY | Self::IndirectX | Self::IndirectY | Self::Relative => 1,
            Self::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 2,
            Self::NoneAddressing => 0,
        }
    }
}
