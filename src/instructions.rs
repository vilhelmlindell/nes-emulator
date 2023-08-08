use crate::cpu::AddressingMode;
use crate::cpu::Cpu;

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
