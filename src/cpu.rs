use std::intrinsics::wrapping_add;
use std::io;

use crate::instructions::{OpCode, CPU_OPCODES};
use crate::memory_bus::{Bus, MemoryBus};

const PC_START: u16 = 0xFFFC;
const SP_START: u8 = 0xFD;
const PRG_ROM_START: usize = 0x8000;
const PRG_ROM_END: usize = 0x10000;

pub struct Cpu {
    pub pc: u16,    // Program counter
    pub sp: u8,     // Stack pointer
    pub a: u8,      // Accumulator
    pub x: u8,      // X register
    pub y: u8,      // Y register
    pub status: u8, // Status register
    pub cycles: u32,
    pub memory: MemoryBus, // Memory
}

#[derive(Debug, PartialEq)]
enum Status {
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
            Self::Immediate
            | Self::ZeroPage
            | Self::ZeroPageX
            | Self::ZeroPageY
            | Self::IndirectX
            | Self::IndirectY
            | Self::Relative => 1,
            Self::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 2,
            Self::NoneAddressing => 0,
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: SP_START,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            cycles: 0,
            memory: MemoryBus::new(),
        }
    }
    pub fn reset(&mut self) {
        self.pc = self.read_word(PC_START);
        self.sp = SP_START;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.cycles = 0;
    }
    pub fn load(&mut self, address: u16, bytes: &[u8]) {
        //if PRG_ROM_START + bytes.len() >= PRG_ROM_END {
        //    panic!(
        //        "Rom of size {}, could not fit in available size {}",
        //        bytes.len(),
        //        PRG_ROM_END - PRG_ROM_START
        //    );
        //}
        //self.write_bytes(PRG_ROM_START as u16, bytes);
        self.write_bytes(address, bytes);
    }
    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    fn step(&mut self) {
        let opcode_index = self.read(self.pc) as usize;
        let opcode = CPU_OPCODES[opcode_index].clone().expect("Invalid opcode");

        self.print_instruction_state(&opcode);

        self.pc += 1;

        (opcode.instruction)(self, &opcode.addressing_mode);

        self.cycles += opcode.cycles as u32;
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

        print!("{}", result);

        println!();
    }

    // Stack operations
    fn push(&mut self, value: u8) {
        self.write(self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }
    fn push_word(&mut self, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.push(high_byte);
        self.push(low_byte);
    }
    fn pull(&mut self) -> u8 {
        let value = self.read(self.sp as u16);
        self.sp = self.sp.wrapping_add(1);
        value
    }
    fn pull_word(&mut self) -> u16 {
        let low_byte = self.pull() as u16;
        let high_byte = self.pull() as u16;

        (high_byte << 8) | low_byte
    }

    fn status(&self, flag: Status) -> bool {
        let flag_bit = 1 << flag as u8;
        (self.status & flag_bit) != 0
    }
    fn set_status(&mut self, flag: Status, value: bool) {
        let flag_bit = 1 << flag as u8;
        if value {
            self.status |= flag_bit; // Set the flag
        } else {
            self.status &= !flag_bit; // Clear the flag
        }
    }
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_status(Status::Zero, true);
        } else {
            self.set_status(Status::Zero, false);
        }

        if result & 0b1000_0000 != 0 {
            self.set_status(Status::Negative, true);
        } else {
            self.set_status(Status::Negative, false);
        }
    }
    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
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

    // Instructions
    pub fn adc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);

        let carry_flag = if self.status(Status::Carry) { 1 } else { 0 };

        let (sum, carry1) = self.a.overflowing_add(operand);
        let (sum_with_carry, carry2) = sum.overflowing_add(carry_flag);

        let overflow = (self.a ^ sum) & (operand ^ sum_with_carry) & 0x80 != 0;

        self.set_status(Status::Carry, carry1 || carry2);

        self.a = sum_with_carry;

        self.update_zero_and_negative_flags(self.a);

        self.set_status(Status::Overflow, overflow);
    }
    pub fn and(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);

        self.a &= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn asl(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = if *mode == AddressingMode::NoneAddressing {
            self.a
        } else {
            self.read(operand_address)
        };

        let carry = operand & 0b1000_0000 != 0;

        operand <<= 1;

        self.set_status(Status::Carry, carry);

        if *mode == AddressingMode::NoneAddressing {
            self.a = operand;
        } else {
            self.write(operand_address, operand);
        };
    }
    pub fn bcc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(Status::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bcs(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(Status::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn beq(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(Status::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bit(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.a & operand;

        if 0b0100_0000 & result != 0 {
            self.set_status(Status::Overflow, true);
        }

        self.update_zero_and_negative_flags(result);
    }
    pub fn bmi(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(Status::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bne(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(Status::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bpl(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(Status::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn brk(&mut self, _mode: &AddressingMode) {
        self.push_word(self.pc);
        self.push(self.status);
        self.pc = self.read_word(PC_START);
        self.set_status(Status::Break, true);
    }
    pub fn bvc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(Status::Overflow) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bvs(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(Status::Overflow) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn clc(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::Carry, false);
    }
    pub fn cld(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::DecimalMode, false);
    }
    pub fn cli(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::InterruptDisable, false);
    }
    pub fn clv(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::Overflow, false);
    }
    pub fn cmp(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.a.wrapping_sub(operand);

        if result >= 0 {
            self.set_status(Status::Carry, true);
        }
        if result == 0 {
            self.set_status(Status::Zero, true);
        }
        if result & 0b0100_0000 != 0 {
            self.set_status(Status::Negative, true);
        }
    }
    pub fn cpx(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.x - operand;

        if result > 0 {
            self.set_status(Status::Carry, true);
        }
        if result == 0 {
            self.set_status(Status::Zero, true);
        }
        if result & 0b0100_0000 != 0 {
            self.set_status(Status::Negative, true);
        }
    }
    pub fn cpy(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.y.wrapping_sub(operand);

        if result >= 0 {
            self.set_status(Status::Carry, true);
        }
        self.update_zero_and_negative_flags(result);
    }
    pub fn dec(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = self.read(operand_address);
        operand = operand.wrapping_sub(1);
        self.update_zero_and_negative_flags(operand);
        self.write(operand_address, operand);
    }
    pub fn dex(&mut self, _mode: &AddressingMode) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn dey(&mut self, _mode: &AddressingMode) {
        self.y = self.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn eor(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        self.a ^= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn inc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = self.read(operand_address);
        operand = operand.wrapping_add(1);
        self.update_zero_and_negative_flags(operand);
        self.write(operand_address, operand);
    }
    pub fn inx(&mut self, _mode: &AddressingMode) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn iny(&mut self, _mode: &AddressingMode) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn jmp(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        self.pc = address;
    }
    pub fn jsr(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read_word(operand_address);
        let return_address = self.pc - 1;
        self.write_word(self.sp as u16, return_address);
        self.pc = operand;
    }
    pub fn lda(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        self.a = operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn ldx(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        self.x = operand;
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn ldy(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);
        self.y = operand;
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn lsr(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = if *mode == AddressingMode::NoneAddressing {
            self.a
        } else {
            self.read(operand_address)
        };

        self.set_status(Status::Carry, operand & 0x01 != 0);
        operand >>= 1;
        self.update_zero_and_negative_flags(operand);

        if *mode == AddressingMode::NoneAddressing {
            self.a = operand;
        } else {
            self.write(operand_address, operand);
        };
    }
    pub fn nop(&mut self, _mode: &AddressingMode) {}
    pub fn ora(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.read(operand_address);

        self.a |= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn pha(&mut self, _mode: &AddressingMode) {
        self.push(self.a);
    }
    pub fn php(&mut self, _mode: &AddressingMode) {
        self.push(self.status);
    }
    pub fn pla(&mut self, _mode: &AddressingMode) {
        let value = self.pull();
        self.a = value;
        self.update_zero_and_negative_flags(value);
    }
    pub fn plp(&mut self, _mode: &AddressingMode) {
        let value = self.pull();
        self.status = value;
        self.update_zero_and_negative_flags(value);
    }
    pub fn rol(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = match mode {
            AddressingMode::NoneAddressing => self.a,
            _ => self.read(operand_address),
        };

        let carry = u8::from(self.status(Status::Carry));
        let new_carry = operand & 0b1000_0000 != 0;

        operand = (operand << 1) | carry;

        self.set_status(Status::Carry, new_carry);
        self.update_zero_and_negative_flags(operand);

        match mode {
            AddressingMode::NoneAddressing => self.a = operand,
            _ => self.write(operand_address, operand),
        };
    }
    pub fn ror(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = match mode {
            AddressingMode::NoneAddressing => self.a,
            _ => self.read(operand_address),
        };

        let carry = u8::from(self.status(Status::Carry)) << 7;
        let new_carry = operand & 0b0000_0001 != 0;

        operand = (operand << 1) | carry;

        self.set_status(Status::Carry, new_carry);
        self.update_zero_and_negative_flags(operand);

        match mode {
            AddressingMode::NoneAddressing => self.a = operand,
            _ => self.write(operand_address, operand),
        };
    }
    pub fn rti(&mut self, _mode: &AddressingMode) {
        self.status = self.pull();
        self.pc = self.pull_word();
    }
    pub fn rts(&mut self, _mode: &AddressingMode) {
        self.pc = self.pull_word().wrapping_sub(1);
    }
    pub fn sbc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        // Same as adc, except the operand is inverted
        let operand = !self.read(operand_address);

        let carry_flag = if self.status(Status::Carry) { 1 } else { 0 };

        let (sum, carry1) = self.a.overflowing_add(operand);
        let (sum_with_carry, carry2) = sum.overflowing_add(carry_flag);

        let overflow = (self.a ^ sum) & (operand ^ sum_with_carry) & 0x80 != 0;

        self.set_status(Status::Carry, carry1 || carry2);

        self.a = sum_with_carry;

        self.update_zero_and_negative_flags(self.a);

        self.set_status(Status::Overflow, overflow);
    }
    pub fn sec(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::Carry, true);
    }
    pub fn sed(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::DecimalMode, true);
    }
    pub fn sei(&mut self, _mode: &AddressingMode) {
        self.set_status(Status::InterruptDisable, true);
    }
    pub fn sta(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        self.write(operand_address, self.a);
    }
    pub fn stx(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        self.write(operand_address, self.x);
    }
    pub fn sty(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        self.write(operand_address, self.y);
    }
    pub fn tax(&mut self, _mode: &AddressingMode) {
        self.x = self.a;
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn tay(&mut self, _mode: &AddressingMode) {
        self.y = self.a;
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn tsx(&mut self, _mode: &AddressingMode) {
        self.x = self.sp;
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn txa(&mut self, _mode: &AddressingMode) {
        self.a = self.x;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn txs(&mut self, _mode: &AddressingMode) {
        self.sp = self.x
    }
    pub fn tya(&mut self, _mode: &AddressingMode) {
        self.a = self.y;
        self.update_zero_and_negative_flags(self.a);
    }
}

impl Bus for Cpu {
    fn read(&self, address: u16) -> u8 {
        self.memory.read(address)
    }
    fn read_word(&self, address: u16) -> u16 {
        self.memory.read_word(address)
    }
    fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value);
    }
    fn write_word(&mut self, address: u16, value: u16) {
        self.memory.write_word(address, value);
    }
    fn write_bytes(&mut self, address: u16, bytes: &[u8]) {
        self.memory.write_bytes(address, bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
