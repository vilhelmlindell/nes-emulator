use crate::cpu::{AddressingMode, Cpu, ProcessorStatus, RESET_VECTOR};
use crate::memory_bus::Bus;

pub trait InstructionSet {
    fn adc(&mut self, mode: &AddressingMode); // Add with Carry
    fn and(&mut self, mode: &AddressingMode); // Logical AND
    fn asl(&mut self, mode: &AddressingMode); // Arithmetic Shift Left
    fn bcc(&mut self, mode: &AddressingMode); // Branch if Carry Clear
    fn bcs(&mut self, mode: &AddressingMode); // Branch if Carry Set
    fn beq(&mut self, mode: &AddressingMode); // Branch if Equal
    fn bit(&mut self, mode: &AddressingMode); // Bit Test
    fn bmi(&mut self, mode: &AddressingMode); // Branch if Minus (Negative)
    fn bne(&mut self, mode: &AddressingMode); // Branch if Not Equal
    fn bpl(&mut self, mode: &AddressingMode); // Branch if Positive
    fn brk(&mut self, mode: &AddressingMode); // Break
    fn bvc(&mut self, mode: &AddressingMode); // Branch if Overflow Clear
    fn bvs(&mut self, mode: &AddressingMode); // Branch if Overflow Set
    fn clc(&mut self, mode: &AddressingMode); // Clear Carry Flag
    fn cld(&mut self, mode: &AddressingMode); // Clear Decimal Mode
    fn cli(&mut self, mode: &AddressingMode); // Clear Interrupt Disable
    fn clv(&mut self, mode: &AddressingMode); // Clear Overflow Flag
    fn cmp(&mut self, mode: &AddressingMode); // Compare
    fn cpx(&mut self, mode: &AddressingMode); // Compare X Register
    fn cpy(&mut self, mode: &AddressingMode); // Compare Y Register
    fn dec(&mut self, mode: &AddressingMode); // Decrement Memory
    fn dex(&mut self, mode: &AddressingMode); // Decrement X Register
    fn dey(&mut self, mode: &AddressingMode); // Decrement Y Register
    fn eor(&mut self, mode: &AddressingMode); // Exclusive OR
    fn inc(&mut self, mode: &AddressingMode); // Increment Memory
    fn inx(&mut self, mode: &AddressingMode); // Increment X Register
    fn iny(&mut self, mode: &AddressingMode); // Increment Y Register
    fn jmp(&mut self, mode: &AddressingMode); // Jump
    fn jsr(&mut self, mode: &AddressingMode); // Jump to Subroutine
    fn lda(&mut self, mode: &AddressingMode); // Load Accumulator
    fn ldx(&mut self, mode: &AddressingMode); // Load X Register
    fn ldy(&mut self, mode: &AddressingMode); // Load Y Register
    fn lsr(&mut self, mode: &AddressingMode); // Logical Shift Right
    fn nop(&mut self, mode: &AddressingMode); // No Operation
    fn ora(&mut self, mode: &AddressingMode); // Logical OR
    fn pha(&mut self, mode: &AddressingMode); // Push Accumulator
    fn php(&mut self, mode: &AddressingMode); // Push Processor Status
    fn pla(&mut self, mode: &AddressingMode); // Pull Accumulator
    fn plp(&mut self, mode: &AddressingMode); // Pull Processor Status
    fn rol(&mut self, mode: &AddressingMode); // Rotate Left
    fn ror(&mut self, mode: &AddressingMode); // Rotate Right
    fn rti(&mut self, mode: &AddressingMode); // Return from Interrupt
    fn rts(&mut self, mode: &AddressingMode); // Return from Subroutine
    fn sbc(&mut self, mode: &AddressingMode); // Subtract with Carry
    fn sec(&mut self, mode: &AddressingMode); // Set Carry Flag
    fn sed(&mut self, mode: &AddressingMode); // Set Decimal Mode
    fn sei(&mut self, mode: &AddressingMode); // Set Interrupt Disable
    fn sta(&mut self, mode: &AddressingMode); // Store Accumulator
    fn stx(&mut self, mode: &AddressingMode); // Store X Register
    fn sty(&mut self, mode: &AddressingMode); // Store Y Register
    fn tax(&mut self, mode: &AddressingMode); // Transfer Accumulator to X
    fn tay(&mut self, mode: &AddressingMode); // Transfer Accumulator to Y
    fn tsx(&mut self, mode: &AddressingMode); // Transfer Stack Pointer to X
    fn txa(&mut self, mode: &AddressingMode); // Transfer X to Accumulator
    fn txs(&mut self, mode: &AddressingMode); // Transfer X to Stack Pointer
    fn tya(&mut self, mode: &AddressingMode); // Transfer Y to Accumulator
}

// Instructions
impl InstructionSet for Cpu {
    fn adc(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);

        let carry_flag = if self.status(ProcessorStatus::Carry) { 1 } else { 0 };

        let (sum, overflow1) = self.a.overflowing_add(operand);
        let (sum_with_carry, overflow2) = sum.overflowing_add(carry_flag);

        let overflow = (self.a ^ operand) & 0x80 == 0 && (self.a ^ sum_with_carry) & 0x80 != 0;

        self.set_status(ProcessorStatus::Carry, overflow1 || overflow2);

        self.a = sum_with_carry;

        self.update_zero_and_negative_flags(self.a);

        self.set_status(ProcessorStatus::Overflow, overflow);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);

        self.a &= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    fn asl(&mut self, mode: &AddressingMode) {
        let mut operand = if *mode == AddressingMode::NoneAddressing {
            self.a
        } else {
            let operand_address = self.operand_address(mode);
            self.read(operand_address)
        };

        let carry = operand & 0b1000_0000 != 0;

        operand <<= 1;

        self.set_status(ProcessorStatus::Carry, carry);
        self.update_zero_and_negative_flags(operand);

        if *mode == AddressingMode::NoneAddressing {
            self.a = operand;
        } else {
            let operand_address = self.operand_address(mode);
            self.write(operand_address, operand);
        };
    }
    fn bcc(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(ProcessorStatus::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn bcs(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(ProcessorStatus::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn beq(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(ProcessorStatus::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn bit(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.a & operand;

        self.set_status(ProcessorStatus::Zero, result == 0);
        self.set_status(ProcessorStatus::Overflow, operand & 0b0100_0000 != 0);
        self.set_status(ProcessorStatus::Negative, operand & 0b1000_0000 != 0);
    }
    fn bmi(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(ProcessorStatus::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn bne(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(ProcessorStatus::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn bpl(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(ProcessorStatus::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn brk(&mut self, _mode: &AddressingMode) {
        self.push_word(self.pc);
        // https://www.nesdev.org/wiki/Status_flags#The_B_flag
        self.push(self.status | 0b0001_0000);
        self.pc = self.read_word(RESET_VECTOR);
    }
    fn bvc(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if !self.status(ProcessorStatus::Overflow) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn bvs(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address) as i8;

        if self.status(ProcessorStatus::Overflow) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    fn clc(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::Carry, false);
    }
    fn cld(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::DecimalMode, false);
    }
    fn cli(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::InterruptDisable, false);
    }
    fn clv(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::Overflow, false);
    }
    fn cmp(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.a.wrapping_sub(operand);

        // Check if there was no borrow during subtraction
        if self.a >= operand {
            self.set_status(ProcessorStatus::Carry, true);
        } else {
            self.set_status(ProcessorStatus::Carry, false);
        }
        self.update_zero_and_negative_flags(result);
    }
    fn cpx(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.x.wrapping_sub(operand); // Use wrapping_sub to handle underflow

        // Check if there was no borrow during subtraction
        if self.x >= operand {
            self.set_status(ProcessorStatus::Carry, true);
        } else {
            self.set_status(ProcessorStatus::Carry, false);
        }

        self.update_zero_and_negative_flags(result);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        let result = self.y.wrapping_sub(operand); // Use wrapping_sub to handle underflow

        // Check if there was no borrow during subtraction
        if self.y >= operand {
            self.set_status(ProcessorStatus::Carry, true);
        } else {
            self.set_status(ProcessorStatus::Carry, false);
        }

        self.update_zero_and_negative_flags(result);
    }
    fn dec(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let mut operand = self.read(operand_address);
        operand = operand.wrapping_sub(1);
        self.update_zero_and_negative_flags(operand);
        self.write(operand_address, operand);
    }
    fn dex(&mut self, _mode: &AddressingMode) {
        self.x = self.x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.x);
    }
    fn dey(&mut self, _mode: &AddressingMode) {
        self.y = self.y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.y);
    }
    fn eor(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        self.a ^= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    fn inc(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let mut operand = self.read(operand_address);
        operand = operand.wrapping_add(1);
        self.update_zero_and_negative_flags(operand);
        self.write(operand_address, operand);
    }
    fn inx(&mut self, _mode: &AddressingMode) {
        self.x = self.x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.x);
    }
    fn iny(&mut self, _mode: &AddressingMode) {
        self.y = self.y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.y);
    }
    fn jmp(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        self.pc = address;
    }
    fn jsr(&mut self, mode: &AddressingMode) {
        let address = self.operand_address(mode);
        self.push_word(self.pc - 1);
        self.pc = address;
    }
    fn lda(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        self.a = operand;
        self.update_zero_and_negative_flags(self.a);
    }
    fn ldx(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        self.x = operand;
        self.update_zero_and_negative_flags(self.x);
    }
    fn ldy(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);
        self.y = operand;
        self.update_zero_and_negative_flags(self.y);
    }
    fn lsr(&mut self, mode: &AddressingMode) {
        let mut operand = if *mode == AddressingMode::NoneAddressing {
            self.a
        } else {
            let operand_address = self.operand_address(mode);
            self.read(operand_address)
        };

        self.set_status(ProcessorStatus::Carry, operand & 0x01 != 0);
        operand >>= 1;
        self.update_zero_and_negative_flags(operand);

        if *mode == AddressingMode::NoneAddressing {
            self.a = operand;
        } else {
            let operand_address = self.operand_address(mode);
            self.write(operand_address, operand);
        };
    }
    fn nop(&mut self, _mode: &AddressingMode) {}
    fn ora(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = self.read(operand_address);

        self.a |= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    fn pha(&mut self, _mode: &AddressingMode) {
        self.push(self.a);
    }
    fn php(&mut self, _mode: &AddressingMode) {
        // https://www.nesdev.org/wiki/Status_flags#The_B_flag
        self.push(self.status | 0b0001_0000);
    }
    fn pla(&mut self, _mode: &AddressingMode) {
        let value = self.pull();
        self.a = value;
        self.update_zero_and_negative_flags(value);
    }
    fn plp(&mut self, _mode: &AddressingMode) {
        let value = self.pull();
        self.status = value;
    }
    fn rol(&mut self, mode: &AddressingMode) {
        let mut operand = match mode {
            AddressingMode::NoneAddressing => self.a,
            _ => {
                let operand_address = self.operand_address(mode);
                self.read(operand_address)
            }
        };

        let carry = u8::from(self.status(ProcessorStatus::Carry));
        let new_carry = operand & 0b1000_0000 != 0;

        operand = (operand << 1) | carry;

        self.set_status(ProcessorStatus::Carry, new_carry);
        self.update_zero_and_negative_flags(operand);

        match mode {
            AddressingMode::NoneAddressing => self.a = operand,
            _ => {
                let operand_address = self.operand_address(mode);
                self.write(operand_address, operand);
            }
        };
    }
    fn ror(&mut self, mode: &AddressingMode) {
        let mut operand = match mode {
            AddressingMode::NoneAddressing => self.a,
            _ => {
                let operand_address = self.operand_address(mode);
                self.read(operand_address)
            }
        };

        let carry = u8::from(self.status(ProcessorStatus::Carry)) << 7;
        let new_carry = operand & 0b0000_0001 != 0;

        operand = (operand >> 1) | carry;

        self.set_status(ProcessorStatus::Carry, new_carry);
        self.update_zero_and_negative_flags(operand);

        match mode {
            AddressingMode::NoneAddressing => self.a = operand,
            _ => {
                let operand_address = self.operand_address(mode);
                self.write(operand_address, operand);
            }
        };
    }
    fn rti(&mut self, _mode: &AddressingMode) {
        self.status = self.pull();
        self.pc = self.pull_word();
    }
    fn rts(&mut self, _mode: &AddressingMode) {
        self.pc = self.pull_word() + 1;
    }
    fn sbc(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        let operand = !self.read(operand_address);

        let carry_flag = if self.status(ProcessorStatus::Carry) { 1 } else { 0 };

        let (sum, overflow1) = self.a.overflowing_add(operand);
        let (sum_with_carry, overflow2) = sum.overflowing_add(carry_flag);

        let overflow = (self.a ^ operand) & 0x80 == 0 && (self.a ^ sum_with_carry) & 0x80 != 0;

        self.set_status(ProcessorStatus::Carry, overflow1 || overflow2);

        self.a = sum_with_carry;

        self.update_zero_and_negative_flags(self.a);

        self.set_status(ProcessorStatus::Overflow, overflow);
    }
    fn sec(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::Carry, true);
    }
    fn sed(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::DecimalMode, true);
    }
    fn sei(&mut self, _mode: &AddressingMode) {
        self.set_status(ProcessorStatus::InterruptDisable, true);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        self.write(operand_address, self.a);
    }
    fn stx(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        self.write(operand_address, self.x);
    }
    fn sty(&mut self, mode: &AddressingMode) {
        let operand_address = self.operand_address(mode);
        self.write(operand_address, self.y);
    }
    fn tax(&mut self, _mode: &AddressingMode) {
        self.x = self.a;
        self.update_zero_and_negative_flags(self.x);
    }
    fn tay(&mut self, _mode: &AddressingMode) {
        self.y = self.a;
        self.update_zero_and_negative_flags(self.y);
    }
    fn tsx(&mut self, _mode: &AddressingMode) {
        self.x = self.sp;
        self.update_zero_and_negative_flags(self.x);
    }
    fn txa(&mut self, _mode: &AddressingMode) {
        self.a = self.x;
        self.update_zero_and_negative_flags(self.a);
    }
    fn txs(&mut self, _mode: &AddressingMode) {
        self.sp = self.x;
    }
    fn tya(&mut self, _mode: &AddressingMode) {
        self.a = self.y;
        self.update_zero_and_negative_flags(self.a);
    }
}
