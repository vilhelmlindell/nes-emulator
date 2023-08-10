use crate::cpu::AddressingMode;
use crate::cpu::Cpu;
use once_cell::sync::Lazy;
use std::collections::HashMap;

const MAX_OPCODES: usize = 256;

#[derive(Clone)]
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

pub static CPU_OPCODES: Lazy<[Option<OpCode>; MAX_OPCODES]> = Lazy::new(|| {
    let mut opcodes: [Option<OpCode>; MAX_OPCODES] = std::array::from_fn(|_| None);

    let mut add_opcode = |opcode: u8,
                          instruction: fn(&mut Cpu, &AddressingMode),
                          addressing_mode: AddressingMode,
                          bytes: u8,
                          cycles: u8| {
        opcodes[opcode as usize] = Some(OpCode::new(instruction, addressing_mode, bytes, cycles));
    };

    // ADC instruction
    add_opcode(0x69, Cpu::adc, AddressingMode::Immediate, 2, 2);
    add_opcode(0x65, Cpu::adc, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x75, Cpu::adc, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x6D, Cpu::adc, AddressingMode::Absolute, 3, 4);
    add_opcode(0x7D, Cpu::adc, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x79, Cpu::adc, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x61, Cpu::adc, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x71, Cpu::adc, AddressingMode::IndirectY, 2, 5);

    // AND instruction
    add_opcode(0x29, Cpu::and, AddressingMode::Immediate, 2, 2);
    add_opcode(0x25, Cpu::and, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x35, Cpu::and, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x2D, Cpu::and, AddressingMode::Absolute, 3, 4);
    add_opcode(0x3D, Cpu::and, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x39, Cpu::and, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x21, Cpu::and, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x31, Cpu::and, AddressingMode::IndirectY, 2, 5);

    // ASL instruction
    add_opcode(0x0A, Cpu::asl, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x06, Cpu::asl, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x16, Cpu::asl, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x0E, Cpu::asl, AddressingMode::Absolute, 3, 6);
    add_opcode(0x1E, Cpu::asl, AddressingMode::AbsoluteX, 3, 7);

    // BCC instruction
    add_opcode(0x90, Cpu::bcc, AddressingMode::NoneAddressing, 2, 2);

    // BEQ instruction
    add_opcode(0xF0, Cpu::beq, AddressingMode::NoneAddressing, 2, 2);

    // BIT instruction
    add_opcode(0x24, Cpu::bit, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x2C, Cpu::bit, AddressingMode::Absolute, 3, 4);

    // BMI instruction
    add_opcode(0x30, Cpu::bmi, AddressingMode::NoneAddressing, 2, 2);
    // BNE instruction
    add_opcode(0xD0, Cpu::bne, AddressingMode::NoneAddressing, 2, 2);
    // BPL instruction
    add_opcode(0x10, Cpu::bpl, AddressingMode::NoneAddressing, 2, 2);
    // BRK instruction
    add_opcode(0x00, Cpu::brk, AddressingMode::NoneAddressing, 1, 7);
    // BVC instruction
    add_opcode(0x50, Cpu::bvc, AddressingMode::NoneAddressing, 2, 2);

    // BVS instruction
    add_opcode(0x70, Cpu::bvs, AddressingMode::NoneAddressing, 2, 2);
    // CLC instruction
    add_opcode(0x18, Cpu::clc, AddressingMode::NoneAddressing, 1, 2);
    // CLD instruction
    add_opcode(0xD8, Cpu::cld, AddressingMode::NoneAddressing, 1, 2);
    // CLI instruction
    add_opcode(0x58, Cpu::cli, AddressingMode::NoneAddressing, 1, 2);
    // CLV instruction
    add_opcode(0xB8, Cpu::clv, AddressingMode::NoneAddressing, 1, 2);
    // CMP instruction
    add_opcode(0xC9, Cpu::cmp, AddressingMode::Immediate, 2, 2);
    add_opcode(0xC5, Cpu::cmp, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xD5, Cpu::cmp, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xCD, Cpu::cmp, AddressingMode::Absolute, 3, 4);
    add_opcode(0xDD, Cpu::cmp, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xD9, Cpu::cmp, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xC1, Cpu::cmp, AddressingMode::IndirectX, 2, 6);
    add_opcode(0xD1, Cpu::cmp, AddressingMode::IndirectY, 2, 5);
    // CPX instruction
    add_opcode(0xE0, Cpu::cpx, AddressingMode::Immediate, 2, 2);
    add_opcode(0xE4, Cpu::cpx, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xEC, Cpu::cpx, AddressingMode::Absolute, 3, 4);
    // CPY instruction
    add_opcode(0xC0, Cpu::cpy, AddressingMode::Immediate, 2, 2);
    add_opcode(0xC4, Cpu::cpy, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xCC, Cpu::cpy, AddressingMode::Absolute, 3, 4);
    // DEC instruction
    add_opcode(0xC6, Cpu::dec, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0xD6, Cpu::dec, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0xCE, Cpu::dec, AddressingMode::Absolute, 3, 6);
    add_opcode(0xDE, Cpu::dec, AddressingMode::AbsoluteX, 3, 7);
    // DEX instruction
    add_opcode(0xCA, Cpu::dex, AddressingMode::NoneAddressing, 1, 2);
    // DEY instruction
    add_opcode(0x88, Cpu::dey, AddressingMode::NoneAddressing, 1, 2);
    // EOR instruction
    add_opcode(0x49, Cpu::eor, AddressingMode::Immediate, 2, 2);
    add_opcode(0x45, Cpu::eor, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x55, Cpu::eor, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x4D, Cpu::eor, AddressingMode::Absolute, 3, 4);
    add_opcode(0x5D, Cpu::eor, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x59, Cpu::eor, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x41, Cpu::eor, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x51, Cpu::eor, AddressingMode::IndirectY, 2, 5);
    // INC instruction
    add_opcode(0xE6, Cpu::inc, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0xF6, Cpu::inc, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0xEE, Cpu::inc, AddressingMode::Absolute, 3, 6);
    add_opcode(0xFE, Cpu::inc, AddressingMode::AbsoluteX, 3, 7);
    // INX instruction
    add_opcode(0xE8, Cpu::inx, AddressingMode::NoneAddressing, 1, 2);
    // INY instruction
    add_opcode(0xC8, Cpu::iny, AddressingMode::NoneAddressing, 1, 2);
    // JMP instruction
    add_opcode(0x4C, Cpu::jmp, AddressingMode::Absolute, 3, 3);
    add_opcode(0x6C, Cpu::jmp, AddressingMode::Indirect, 3, 5);
    // JSR instruction
    add_opcode(0x20, Cpu::jsr, AddressingMode::Absolute, 3, 6);
    // LDA instruction
    add_opcode(0xA9, Cpu::lda, AddressingMode::Immediate, 2, 2);
    add_opcode(0xA5, Cpu::lda, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB5, Cpu::lda, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xAD, Cpu::lda, AddressingMode::Absolute, 3, 4);
    add_opcode(0xBD, Cpu::lda, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xB9, Cpu::lda, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xA1, Cpu::lda, AddressingMode::IndirectX, 2, 6);
    add_opcode(0xB1, Cpu::lda, AddressingMode::IndirectY, 2, 5);
    // LDX instruction
    add_opcode(0xA2, Cpu::ldx, AddressingMode::Immediate, 2, 2);
    add_opcode(0xA6, Cpu::ldx, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB6, Cpu::ldx, AddressingMode::ZeroPageY, 2, 4);
    add_opcode(0xAE, Cpu::ldx, AddressingMode::Absolute, 3, 4);
    add_opcode(0xBE, Cpu::ldx, AddressingMode::AbsoluteY, 3, 4);
    // LDY instruction
    add_opcode(0xA0, Cpu::ldy, AddressingMode::Immediate, 2, 2);
    add_opcode(0xA4, Cpu::ldy, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB4, Cpu::ldy, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xAC, Cpu::ldy, AddressingMode::Absolute, 3, 4);
    add_opcode(0xBC, Cpu::ldy, AddressingMode::AbsoluteX, 3, 4);
    // LSR instruction
    add_opcode(0x4A, Cpu::lsr, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x46, Cpu::lsr, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x56, Cpu::lsr, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x4E, Cpu::lsr, AddressingMode::Absolute, 3, 6);
    add_opcode(0x5E, Cpu::lsr, AddressingMode::AbsoluteX, 3, 7);
    // NOP instruction
    add_opcode(0xEA, Cpu::nop, AddressingMode::NoneAddressing, 1, 2);
    // ORA instruction
    add_opcode(0x09, Cpu::ora, AddressingMode::Immediate, 2, 2);
    add_opcode(0x05, Cpu::ora, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x15, Cpu::ora, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x0D, Cpu::ora, AddressingMode::Absolute, 3, 4);
    add_opcode(0x1D, Cpu::ora, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x19, Cpu::ora, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x01, Cpu::ora, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x11, Cpu::ora, AddressingMode::IndirectY, 2, 5);
    // PHA instruction
    add_opcode(0x48, Cpu::pha, AddressingMode::NoneAddressing, 1, 3);
    // PHP instruction
    add_opcode(0x08, Cpu::php, AddressingMode::NoneAddressing, 1, 3);
    // PLA instruction
    add_opcode(0x68, Cpu::pla, AddressingMode::NoneAddressing, 1, 4);
    // PLP instruction
    add_opcode(0x28, Cpu::plp, AddressingMode::NoneAddressing, 1, 4);
    // ROL instruction
    add_opcode(0x2A, Cpu::rol, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x26, Cpu::rol, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x36, Cpu::rol, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x2E, Cpu::rol, AddressingMode::Absolute, 3, 6);
    add_opcode(0x3E, Cpu::rol, AddressingMode::AbsoluteX, 3, 7);
    // ROR instruction
    add_opcode(0x6A, Cpu::ror, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x66, Cpu::ror, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x76, Cpu::ror, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x6E, Cpu::ror, AddressingMode::Absolute, 3, 6);
    add_opcode(0x7E, Cpu::ror, AddressingMode::AbsoluteX, 3, 7);
    // RTI instruction
    add_opcode(0x40, Cpu::rti, AddressingMode::NoneAddressing, 1, 6);
    // RTS instruction
    add_opcode(0x60, Cpu::rts, AddressingMode::NoneAddressing, 1, 6);
    // SBC instruction
    add_opcode(0xE9, Cpu::sbc, AddressingMode::Immediate, 2, 2);
    add_opcode(0xE5, Cpu::sbc, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xF5, Cpu::sbc, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xED, Cpu::sbc, AddressingMode::Absolute, 3, 4);
    add_opcode(0xFD, Cpu::sbc, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xF9, Cpu::sbc, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xE1, Cpu::sbc, AddressingMode::IndirectX, 2, 6);
    add_opcode(0xF1, Cpu::sbc, AddressingMode::IndirectY, 2, 5);
    // SEC instruction
    add_opcode(0x38, Cpu::sec, AddressingMode::NoneAddressing, 1, 2);
    // SED instruction
    add_opcode(0xF8, Cpu::sed, AddressingMode::NoneAddressing, 1, 2);
    // SEI instruction
    add_opcode(0x78, Cpu::sei, AddressingMode::NoneAddressing, 1, 2);
    // STA instruction
    add_opcode(0x85, Cpu::sta, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x95, Cpu::sta, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x8D, Cpu::sta, AddressingMode::Absolute, 3, 4);
    add_opcode(0x9D, Cpu::sta, AddressingMode::AbsoluteX, 3, 5);
    add_opcode(0x99, Cpu::sta, AddressingMode::AbsoluteY, 3, 5);
    add_opcode(0x81, Cpu::sta, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x91, Cpu::sta, AddressingMode::IndirectY, 2, 6);
    // STX instruction
    add_opcode(0x86, Cpu::stx, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x96, Cpu::stx, AddressingMode::ZeroPageY, 2, 4);
    add_opcode(0x8E, Cpu::stx, AddressingMode::Absolute, 3, 4);
    // STY instruction
    add_opcode(0x84, Cpu::sty, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x94, Cpu::sty, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x8C, Cpu::sty, AddressingMode::Absolute, 3, 4);
    // TAX instruction
    add_opcode(0xAA, Cpu::tax, AddressingMode::NoneAddressing, 1, 2);
    // TAY instruction
    add_opcode(0xA8, Cpu::tay, AddressingMode::NoneAddressing, 1, 2);
    // TSX instruction
    add_opcode(0xBA, Cpu::tsx, AddressingMode::NoneAddressing, 1, 2);
    // TXA instruction
    add_opcode(0x8A, Cpu::txa, AddressingMode::NoneAddressing, 1, 2);
    // TXS instruction
    add_opcode(0x9A, Cpu::txs, AddressingMode::NoneAddressing, 1, 2);
    // TYA instruction
    add_opcode(0x98, Cpu::tya, AddressingMode::NoneAddressing, 1, 2);
    opcodes
});
