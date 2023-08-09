use std::collections::HashMap;

use crate::cpu::AddressingMode;
use crate::cpu::Cpu;
use once_cell::sync::Lazy;

pub struct OpCode {
    instruction: fn(&mut Cpu, &AddressingMode),
    addressing_mode: AddressingMode,
    bytes: u8,
    cycles: u8,
}

impl OpCode {
    pub fn new(
        instruction: fn(&mut Cpu, &AddressingMode),
        addressing_mode: AddressingMode,
        bytes: u8,
        cycles: u8,
    ) -> Self {
        Self {
            instruction,
            addressing_mode,
            bytes,
            cycles,
        }
    }
}

pub static CPU_OPCODES: Lazy<HashMap<u8, OpCode>> = Lazy::new(|| {
    HashMap::from([
        // ADC instruction
        (0x69, OpCode::new(Cpu::adc, AddressingMode::Immediate, 2, 2)),
        (0x65, OpCode::new(Cpu::adc, AddressingMode::ZeroPage, 2, 3)),
        (0x75, OpCode::new(Cpu::adc, AddressingMode::ZeroPageX, 2, 4)),
        (0x6D, OpCode::new(Cpu::adc, AddressingMode::Absolute, 3, 4)),
        (0x7D, OpCode::new(Cpu::adc, AddressingMode::AbsoluteX, 3, 4)),
        (0x79, OpCode::new(Cpu::adc, AddressingMode::AbsoluteY, 3, 4)),
        (0x61, OpCode::new(Cpu::adc, AddressingMode::IndirectX, 2, 6)),
        (0x71, OpCode::new(Cpu::adc, AddressingMode::IndirectY, 2, 5)),
        // AND instruction
        (0x29, OpCode::new(Cpu::and, AddressingMode::Immediate, 2, 2)),
        (0x25, OpCode::new(Cpu::and, AddressingMode::ZeroPage, 2, 3)),
        (0x35, OpCode::new(Cpu::and, AddressingMode::ZeroPageX, 2, 4)),
        (0x2D, OpCode::new(Cpu::and, AddressingMode::Absolute, 3, 4)),
        (0x3D, OpCode::new(Cpu::and, AddressingMode::AbsoluteX, 3, 4)),
        (0x39, OpCode::new(Cpu::and, AddressingMode::AbsoluteY, 3, 4)),
        (0x21, OpCode::new(Cpu::and, AddressingMode::IndirectX, 2, 6)),
        (0x31, OpCode::new(Cpu::and, AddressingMode::IndirectY, 2, 5)),
        // ASL instruction
        (
            0x0A,
            OpCode::new(Cpu::asl, AddressingMode::NoneAddressing, 1, 2),
        ),
        (0x06, OpCode::new(Cpu::asl, AddressingMode::ZeroPage, 2, 5)),
        (0x16, OpCode::new(Cpu::asl, AddressingMode::ZeroPageX, 2, 6)),
        (0x0E, OpCode::new(Cpu::asl, AddressingMode::Absolute, 3, 6)),
        (0x1E, OpCode::new(Cpu::asl, AddressingMode::AbsoluteX, 3, 7)),
        // BCC instruction
        (
            0x90,
            OpCode::new(Cpu::bcc, AddressingMode::NoneAddressing, 2, 2),
        ),
    ])
});
