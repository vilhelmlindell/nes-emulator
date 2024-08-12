use bitflags::Flags;

use crate::mapper::Mapper;
use crate::memory_bus::MemoryBus;
use crate::opcodes::{Instruction, CPU_OPCODES};
use crate::ppu::{ControlFlags, StatusFlags};
use bitflags::bitflags;
use std::intrinsics::wrapping_add;

pub const RESET_VECTOR: u16 = 0xFFFC;
const SP_START: u8 = 0xFD;
const STATUS_DEFAULT: u8 = 0b0010_0100;
const NMI_ADDRESS: u16 = 0xFFFA;

pub struct Cpu {
    pub pc: u16,                 // Program counter
    pub sp: u8,                  // Stack pointer
    pub a: u8,                   // Accumulator
    pub x: u8,                   // X register
    pub y: u8,                   // Y register
    pub status: ProcessorStatus, // Status register
    pub cycles: u32,
    pub memory_bus: MemoryBus, // Memory
}

impl Cpu {
    pub fn new(mut memory_bus: MemoryBus) -> Cpu {
        Cpu {
            pc: memory_bus.read_word(RESET_VECTOR),
            sp: SP_START,
            a: 0,
            x: 0,
            y: 0,
            status: ProcessorStatus::from_bits_truncate(STATUS_DEFAULT),
            cycles: 0,
            memory_bus,
        }
    }
    pub fn reset(&mut self) {
        self.pc = self.memory_bus.read_word(RESET_VECTOR);
        self.sp = SP_START;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = ProcessorStatus::from_bits_truncate(STATUS_DEFAULT);
        self.cycles = 0;
    }
    pub fn instruction_cycle(&mut self) {
        if self.memory_bus.ppu.control_register.contains(ControlFlags::GenerateNmi) {
            self.memory_bus.ppu.control_register.set(ControlFlags::GenerateNmi, false);
            self.interrupt_nmi();
        }

        let instruction = self.fetch();
        let cycles = self.execute(&instruction);
        self.cycles += cycles as u32;
        for _ in 0..cycles * 3 {
            self.memory_bus.ppu.step();
        }
    }
    pub fn fetch(&self) -> Instruction {
        let opcode = self.memory_bus.debug_read(self.pc) as usize;
        CPU_OPCODES[opcode].clone().unwrap_or_else(|| panic!("Invalid opcode: {:X}", opcode))
    }
    pub fn execute(&mut self, instruction: &Instruction) -> u8 {
        self.pc += 1;
        (instruction.function)(self, instruction.addressing_mode);
        instruction.cycles
    }
    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.execute(&instruction);
        }
    }
    pub fn execution_trace(&self, instruction: &Instruction) -> String {
        let mut output = format!("{:04X}  ", self.pc);
        for i in 0..3 {
            if i < instruction.bytes {
                output.push_str(&format!("{:02X} ", self.memory_bus.debug_read(self.pc.wrapping_add(i as u16))));
            } else {
                output.push_str("   ");
            }
        }
        output.push(' ');
        output.push_str(&format!("{} ", instruction.name));
        for _ in 0..=27 {
            output.push(' ');
        }
        output.push_str(&format!("A:{:02X} X:{:02X} Y:{:02X} ", self.a, self.x, self.y));

        let letters = "CZID--VN";
        let mut result = String::new();

        for i in 0..8 {
            if i == 5 {
                continue;
            }
            let mask = 1 << i;
            if self.status.bits() & mask != 0 {
                result.insert(0, letters.chars().nth(i).unwrap());
            } else {
                result.insert(0, '-');
            }
        }

        output.push_str(&format!("P:[{}] ", result));
        //output.push_str(&format!("P:{:02X} ", self.status));
        output.push_str(&format!("SP:{:02X} ", self.sp));
        output.push_str(&format!("CYC:{}", self.cycles));

        output
    }

    // Stack operations
    pub fn push(&mut self, value: u8) {
        self.memory_bus.write(0x0100 + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_word(&mut self, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.push(high_byte);
        self.push(low_byte);
    }

    pub fn pull(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory_bus.read(0x0100 + self.sp as u16)
    }

    pub fn pull_word(&mut self) -> u16 {
        let low_byte = self.pull() as u16;
        let high_byte = self.pull() as u16;

        (high_byte << 8) | low_byte
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.set(ProcessorStatus::Zero, true);
        } else {
            self.status.set(ProcessorStatus::Zero, false);
        }

        if result & 0b1000_0000 != 0 {
            self.status.set(ProcessorStatus::Negative, true);
        } else {
            self.status.set(ProcessorStatus::Negative, false);
        }
    }
    pub fn operand_address(&mut self, mode: AddressingMode) -> u16 {
        let address = match mode {
            AddressingMode::Immediate | AddressingMode::Relative => self.pc,
            AddressingMode::ZeroPage => self.memory_bus.read(self.pc) as u16,
            AddressingMode::Absolute => self.memory_bus.read_word(self.pc),
            AddressingMode::ZeroPageX => {
                let addr = self.memory_bus.read(self.pc);
                wrapping_add(addr, self.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = self.memory_bus.read(self.pc);
                wrapping_add(addr, self.y) as u16
            }
            AddressingMode::AbsoluteX => {
                let addr = self.memory_bus.read_word(self.pc);
                wrapping_add(addr, self.x as u16)
            }
            AddressingMode::AbsoluteY => {
                let addr = self.memory_bus.read_word(self.pc);
                wrapping_add(addr, self.y as u16)
            }
            AddressingMode::IndirectX => {
                let zero_page_addr = self.memory_bus.read(self.pc);
                let addr_low = self.memory_bus.read(zero_page_addr.wrapping_add(self.x) as u16) as u16;
                let addr_high = self.memory_bus.read(zero_page_addr.wrapping_add(self.x.wrapping_add(1)) as u16) as u16;

                (addr_high << 8) | addr_low
            }
            AddressingMode::IndirectY => {
                let zero_page_addr = self.memory_bus.read(self.pc);
                let addr_low = self.memory_bus.read(zero_page_addr as u16) as u16;
                let addr_high = self.memory_bus.read(zero_page_addr.wrapping_add(1) as u16) as u16;
                let base_addr = (addr_high << 8) | addr_low;

                base_addr.wrapping_add(self.y as u16)
            }
            AddressingMode::Indirect => {
                let addr = self.memory_bus.read_word(self.pc);
                let addr_low = self.memory_bus.read(addr) as u16;
                let addr_high_location = (addr & 0xFF == 0xFF).then_some(addr & 0xFF00).unwrap_or_else(|| addr.wrapping_add(1));
                let addr_high = self.memory_bus.read(addr_high_location) as u16;
                (addr_high << 8) | addr_low
            }
            AddressingMode::NoneAddressing => 0,
        };
        self.pc = self.pc.wrapping_add(mode.byte_count());
        address
    }
    pub fn count_branch_cycles(&mut self, old_pc: u16, mode: AddressingMode) {
        if mode != AddressingMode::Relative {
            return;
        }
        self.cycles += 1;
        if (old_pc & 0xFF00) != (self.pc & 0xFF00) {
            self.cycles += 1;
        }
    }
    pub fn count_page_crossing_cycles(&mut self, old_pc: u16, operand_address: u16, mode: AddressingMode) {
        let address: u16 = match mode {
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => self.memory_bus.read_word(old_pc),
            AddressingMode::IndirectY => {
                let zero_page_addr = self.memory_bus.read(old_pc) as u16;
                let address_low = self.memory_bus.read(zero_page_addr) as u16;
                let address_high = self.memory_bus.read(zero_page_addr.wrapping_add(1)) as u16;
                (address_high << 8) | address_low
            }
            _ => {
                return;
            }
        };

        if (address & 0xFF00) != (operand_address & 0xFF00) {
            self.cycles += 1;
        }
    }
    fn interrupt_nmi(&mut self) {
        self.push_word(self.pc);
        let mut flag = self.status.clone();
        flag.set(ProcessorStatus::Break, true);
        flag.set(ProcessorStatus::Break1, true);

        self.push(flag.bits());
        self.status.set(ProcessorStatus::InterruptDisable, true);

        self.cycles += 2;
        self.pc = self.memory_bus.read_word(NMI_ADDRESS);
    }
}

bitflags! {
    #[derive(Debug, PartialEq, Clone)]
    pub struct ProcessorStatus: u8 {
        const Carry = 0b0000_0001;
        const Zero = 0b0000_0010;
        const InterruptDisable = 0b0000_0100;
        const DecimalMode = 0b0000_1000;
        const Break = 0b0001_0000;
        const Break1 = 0b0010_0000;
        const Overflow = 0b0100_0000;
        const Negative = 0b1000_0000;
    }
}

#[derive(PartialEq, Clone, Copy)]
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
    Indirect,
    NoneAddressing,
}

impl AddressingMode {
    pub fn byte_count(&self) -> u16 {
        match self {
            Self::Immediate | Self::ZeroPage | Self::ZeroPageX | Self::ZeroPageY | Self::IndirectX | Self::IndirectY | Self::Relative => 1,
            Self::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY | AddressingMode::Indirect => 2,
            Self::NoneAddressing => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rom::Rom;

    use super::*;

    #[test]
    fn test_stack() {
        let mut cpu = Cpu::new(MemoryBus::new(Rom::default()));

        let pushed_byte = 164;
        cpu.push(pushed_byte);

        assert_eq!(cpu.pull(), pushed_byte);

        let pushed_word = 35934;
        cpu.push_word(pushed_word);

        assert_eq!(cpu.pull_word(), pushed_word);
    }
}
