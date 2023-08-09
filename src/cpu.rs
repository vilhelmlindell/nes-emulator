use std::intrinsics::wrapping_add;

const MEMORY_SIZE: usize = 0xFFFF;
const PC_START_ADDRESS: u16 = 0xFFFC;
const SP_START_ADDRESS: u8 = 0xFD;

#[derive(Debug, PartialEq)]
enum Status {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    Break,
    Overflow,
    Negative,
}

#[derive(PartialEq)]
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

pub struct Cpu {
    pc: u16,                   // Program counter
    sp: u8,                    // Stack pointer
    a: u8,                     // Accumulator
    x: u8,                     // X register
    y: u8,                     // Y register
    status: u8,                // Status register
    memory: [u8; MEMORY_SIZE], // Memory
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0xFD,
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            memory: [0; MEMORY_SIZE],
        }
    }
    fn reset(&mut self) {
        self.pc = self.read_word(PC_START_ADDRESS);
        self.sp = 0xFD;
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }
    fn run(&mut self) {
        loop {
            //let opcode = self.read_byte(self.pc) as usize;
            //self.pc += 1;
            //let instruction = CPU_OPCODES[opcode];
            //instruction();
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
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.memory[address as usize] = low_byte;
        self.memory[(address + 1) as usize] = high_byte;
    }

    // Stack operations
    fn push(&mut self, value: u8) {
        self.memory[self.sp as usize] = value;
        self.sp = self.sp.wrapping_sub(1);
    }
    fn push_word(&mut self, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.push(high_byte);
        self.push(low_byte);
    }
    fn pull(&mut self) -> u8 {
        let value = self.memory[self.sp as usize];
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
            self.set_status(Status::Carry, true);
        }

        if result & 0b1000_0000 != 0 {
            self.set_status(Status::Negative, true);
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
            AddressingMode::NoneAddressing => {
                panic!("Mode doesn't support addresses")
            }
        }
    }

    // Instructions
    pub fn adc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        // Overflow
        let (sum, overflow) = self.a.overflowing_add(operand);
        self.a = sum;
        if !self.status(Status::Carry) {
            self.a += 1;
        }
        if overflow {
            self.set_status(Status::Carry, true);
        }
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn and(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        self.a &= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn asl(&mut self, mode: &AddressingMode) {
        let operand = if *mode == AddressingMode::NoneAddressing {
            &mut self.a
        } else {
            let operand_address = self.get_operand_address(mode);
            &mut self.memory[operand_address as usize]
        };

        let carry = *operand & 0b1000_0000 != 0;

        *operand = *operand << 1;

        if carry {
            self.set_status(Status::Carry, true);
        }
    }
    pub fn bcc(&mut self, mode: &AddressingMode) {
        let operand = self.memory[self.pc as usize] as i8;

        if !self.status(Status::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bcs(&mut self, mode: &AddressingMode) {
        let operand = self.memory[self.pc as usize] as i8;

        if self.status(Status::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn beq(&mut self, mode: &AddressingMode) {
        let operand = self.memory[self.pc as usize] as i8;

        if self.status(Status::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bit(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[self.memory[operand_address as usize] as usize];
        let result = self.a & operand;

        if 0b0100_0000 & result != 0 {
            self.set_status(Status::Overflow, true);
        }

        self.update_zero_and_negative_flags(result);
    }
    pub fn bmi(&mut self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if self.status(Status::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bne(&mut self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if !self.status(Status::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bpl(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize] as i8;

        if !self.status(Status::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn brk(&mut self, mode: &AddressingMode) {
        self.push_word(self.pc);
        self.push(self.status);
        self.pc = self.read_word(PC_START_ADDRESS);
        self.set_status(Status::Break, true);
    }
    pub fn bvc(&mut self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if !self.status(Status::Overflow) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bvs(&mut self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if self.status(Status::Overflow) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn clc(&mut self, mode: &AddressingMode) {
        self.set_status(Status::Carry, false);
    }
    pub fn cld(&mut self, mode: &AddressingMode) {
        self.set_status(Status::DecimalMode, false);
    }
    pub fn cli(&mut self, mode: &AddressingMode) {
        self.set_status(Status::InterruptDisable, false);
    }
    pub fn cmp(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];
        let result = self.a - operand;

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
    pub fn cpx(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];
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
        let operand = self.memory[operand_address as usize];
        let result = self.y - operand;

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
    pub fn dec(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = &mut self.memory[self.memory[operand_address as usize] as usize];
        *operand = operand.wrapping_sub(1);
        self.update_zero_and_negative_flags(*operand);
    }
    pub fn dex(&mut self, mode: &AddressingMode) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn dey(&mut self, mode: &AddressingMode) {
        self.y = self.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn eor(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];
        self.a ^= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn inc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let mut operand = &mut self.memory[self.memory[operand_address as usize] as usize];
        *operand = operand.wrapping_add(1);
        self.update_zero_and_negative_flags(*operand);
    }
    pub fn inx(&mut self, mode: &AddressingMode) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn iny(&mut self, mode: &AddressingMode) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn jmp(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];
        self.pc = operand as u16;
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
        let operand = self.memory[operand_address as usize];
        self.a = operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn ldx(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];
        self.x = operand;
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn ldy(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];
        self.y = operand;
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn nop(&mut self, mode: &AddressingMode) {}
    pub fn ora(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        self.a |= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn pha(&mut self, mode: &AddressingMode) {
        self.push(self.a);
    }
    pub fn php(&mut self, mode: &AddressingMode) {
        let value = self.pull();
        self.push(self.status);
    }
    pub fn pla(&mut self, mode: &AddressingMode) {
        let value = self.pull();
        self.a = value;
        self.update_zero_and_negative_flags(value);
    }
    pub fn plp(&mut self, mode: &AddressingMode) {
        let value = self.pull();
        self.status = value;
        self.update_zero_and_negative_flags(value);
    }
    pub fn rol(&mut self, mode: &AddressingMode) {
        let mut operand = if *mode == AddressingMode::NoneAddressing {
            &mut self.a
        } else {
            let operand_address = self.get_operand_address(mode);
            &mut self.memory[operand_address as usize]
        };
        let carry = self.status(Status::Carry) as u8;

        let new_carry = *operand & 0b1000_0000 != 0;
        *operand = (*operand << 1) | carry;

        self.set_status(Status::Carry, new_carry);
        self.update_zero_and_negative_flags(*operand);
    }
    pub fn ror(&mut self, mode: &AddressingMode) {
        let mut operand = if *mode == AddressingMode::NoneAddressing {
            &mut self.a
        } else {
            let operand_address = self.get_operand_address(mode);
            &mut self.memory[operand_address as usize]
        };
        let carry = self.status(Status::Carry) as u8;

        let new_carry = *operand & 0b0000_0001 != 0;
        *operand = (*operand >> 1) | (carry << 7);

        self.set_status(Status::Carry, new_carry);
        self.update_zero_and_negative_flags(*operand);
    }
    pub fn rti(&mut self, mode: &AddressingMode) {
        self.status = self.pull();
        self.pc = self.pull_word();
    }
    pub fn rts(&mut self, mode: &AddressingMode) {
        self.pc = self.pull_word().wrapping_sub(1);
    }
    pub fn sbc(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        // Take the two's complement of the operand if the Carry flag is clear
        let operand = if !self.status(Status::Carry) {
            (!operand).wrapping_add(1)
        } else {
            operand
        };

        let result = self.a.wrapping_sub(operand);
        self.set_status(Status::Carry, self.a >= operand);
        self.set_status(
            Status::Overflow,
            (self.a ^ operand) & 0x80 != 0 && (self.a ^ result) & 0x80 != 0,
        );
        self.a = result;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn sec(&mut self, mode: &AddressingMode) {
        self.set_status(Status::Carry, true);
    }
    pub fn sed(&mut self, mode: &AddressingMode) {
        self.set_status(Status::DecimalMode, true);
    }
    pub fn sei(&mut self, mode: &AddressingMode) {
        self.set_status(Status::InterruptDisable, true);
    }
    pub fn sta(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        self.memory[operand_address as usize] = self.a;
    }
    pub fn stx(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        self.memory[operand_address as usize] = self.x;
    }
    pub fn sty(&mut self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        self.memory[operand_address as usize] = self.x;
    }
    pub fn tax(&mut self, mode: &AddressingMode) {
        self.x = self.a;
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn tay(&mut self, mode: &AddressingMode) {
        self.y = self.a;
        self.update_zero_and_negative_flags(self.y);
    }
    pub fn tsx(&mut self, mode: &AddressingMode) {
        self.x = self.sp;
        self.update_zero_and_negative_flags(self.x);
    }
    pub fn txa(&mut self, mode: &AddressingMode) {
        self.a = self.x;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn txs(&mut self, mode: &AddressingMode) {
        self.sp = self.x
    }
    pub fn tya(&mut self, mode: &AddressingMode) {
        self.a = self.y;
        self.update_zero_and_negative_flags(self.a);
    }
}
