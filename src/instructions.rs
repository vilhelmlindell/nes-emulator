use crate::cpu::AddressingMode;
use crate::cpu::Cpu;
use once_cell::sync::Lazy;

const MAX_OPCODES: usize = 256;

#[derive(Clone)]
pub struct OpCode {
    pub instruction: fn(&mut Cpu, &AddressingMode),
    pub name: &'static str,
    pub addressing_mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl OpCode {
    pub fn new(
        instruction: fn(&mut Cpu, &AddressingMode),
        name: &'static str,
        addressing_mode: AddressingMode,
        bytes: u8,
        cycles: u8,
    ) -> Self {
        Self {
            instruction,
            name,
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
                          name: &'static str, // Name is the third argument now
                          addressing_mode: AddressingMode,
                          bytes: u8,
                          cycles: u8| {
        opcodes[opcode as usize] = Some(OpCode::new(
            instruction,
            name,
            addressing_mode,
            bytes,
            cycles,
        ));
    };
    // ADC instruction
    add_opcode(0x69, Cpu::adc, "ADC", AddressingMode::Immediate, 2, 2);
    add_opcode(0x65, Cpu::adc, "ADC", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x75, Cpu::adc, "ADC", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x6D, Cpu::adc, "ADC", AddressingMode::Absolute, 3, 4);
    add_opcode(0x7D, Cpu::adc, "ADC", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x79, Cpu::adc, "ADC", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x61, Cpu::adc, "ADC", AddressingMode::IndirectX, 2, 6);
    add_opcode(0x71, Cpu::adc, "ADC", AddressingMode::IndirectY, 2, 5);

    // AND instruction
    add_opcode(0x29, Cpu::and, "AND", AddressingMode::Immediate, 2, 2);
    add_opcode(0x25, Cpu::and, "AND", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x35, Cpu::and, "AND", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x2D, Cpu::and, "AND", AddressingMode::Absolute, 3, 4);
    add_opcode(0x3D, Cpu::and, "AND", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x39, Cpu::and, "AND", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x21, Cpu::and, "AND", AddressingMode::IndirectX, 2, 6);
    add_opcode(0x31, Cpu::and, "AND", AddressingMode::IndirectY, 2, 5);

    // ASL instruction
    add_opcode(0x0A, Cpu::asl, "ASL", AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x06, Cpu::asl, "ASL", AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x16, Cpu::asl, "ASL", AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x0E, Cpu::asl, "ASL", AddressingMode::Absolute, 3, 6);
    add_opcode(0x1E, Cpu::asl, "ASL", AddressingMode::AbsoluteX, 3, 7);

    // BCC instruction
    add_opcode(0x90, Cpu::bcc, "BCC", AddressingMode::Relative, 2, 2);

    // BCS instruction
    add_opcode(0xB0, Cpu::bcs, "BCS", AddressingMode::Relative, 2, 2);

    // BEQ instruction
    add_opcode(0xF0, Cpu::beq, "BEQ", AddressingMode::Relative, 2, 2);

    // BIT instruction
    add_opcode(0x24, Cpu::bit, "BIT", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x2C, Cpu::bit, "BIT", AddressingMode::Absolute, 3, 4);

    // BMI instruction
    add_opcode(0x30, Cpu::bmi, "BMI", AddressingMode::Relative, 2, 2);
    // BNE instruction
    add_opcode(0xD0, Cpu::bne, "BNE", AddressingMode::Relative, 2, 2);
    // BPL instruction
    add_opcode(0x10, Cpu::bpl, "BPL", AddressingMode::Relative, 2, 2);
    // BRK instruction
    add_opcode(0x00, Cpu::brk, "BRK", AddressingMode::NoneAddressing, 1, 7);
    // BVC instruction
    add_opcode(0x50, Cpu::bvc, "BVC", AddressingMode::Relative, 2, 2);

    // BVS instruction
    add_opcode(0x70, Cpu::bvs, "BVS", AddressingMode::Relative, 2, 2);
    // CLC instruction
    add_opcode(0x18, Cpu::clc, "CLC", AddressingMode::NoneAddressing, 1, 2);
    // CLD instruction
    add_opcode(0xD8, Cpu::cld, "CLD", AddressingMode::NoneAddressing, 1, 2);
    // CLI instruction
    add_opcode(0x58, Cpu::cli, "CLI", AddressingMode::NoneAddressing, 1, 2);
    // CLV instruction
    add_opcode(0xB8, Cpu::clv, "CLV", AddressingMode::NoneAddressing, 1, 2);
    // CMP instruction
    add_opcode(0xC9, Cpu::cmp, "CMP", AddressingMode::Immediate, 2, 2);
    add_opcode(0xC5, Cpu::cmp, "CMP", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xD5, Cpu::cmp, "CMP", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xCD, Cpu::cmp, "CMP", AddressingMode::Absolute, 3, 4);
    add_opcode(0xDD, Cpu::cmp, "CMP", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xD9, Cpu::cmp, "CMP", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xC1, Cpu::cmp, "CMP", AddressingMode::IndirectX, 2, 6);
    add_opcode(0xD1, Cpu::cmp, "CMP", AddressingMode::IndirectY, 2, 5);
    // CPX instruction
    add_opcode(0xE0, Cpu::cpx, "CPX", AddressingMode::Immediate, 2, 2);
    add_opcode(0xE4, Cpu::cpx, "CPX", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xEC, Cpu::cpx, "CPX", AddressingMode::Absolute, 3, 4);
    // CPY instruction
    add_opcode(0xC0, Cpu::cpy, "CPY", AddressingMode::Immediate, 2, 2);
    add_opcode(0xC4, Cpu::cpy, "CPY", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xCC, Cpu::cpy, "CPY", AddressingMode::Absolute, 3, 4);
    // DEC instruction
    add_opcode(0xC6, Cpu::dec, "DEC", AddressingMode::ZeroPage, 2, 5);
    add_opcode(0xD6, Cpu::dec, "DEC", AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0xCE, Cpu::dec, "DEC", AddressingMode::Absolute, 3, 6);
    add_opcode(0xDE, Cpu::dec, "DEC", AddressingMode::AbsoluteX, 3, 7);
    // DEX instruction
    add_opcode(0xCA, Cpu::dex, "DEX", AddressingMode::NoneAddressing, 1, 2);
    // DEY instruction
    add_opcode(0x88, Cpu::dey, "DEY", AddressingMode::NoneAddressing, 1, 2);
    // EOR instruction
    add_opcode(0x49, Cpu::eor, "EOR", AddressingMode::Immediate, 2, 2);
    add_opcode(0x45, Cpu::eor, "EOR", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x55, Cpu::eor, "EOR", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x4D, Cpu::eor, "EOR", AddressingMode::Absolute, 3, 4);
    add_opcode(0x5D, Cpu::eor, "EOR", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x59, Cpu::eor, "EOR", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x41, Cpu::eor, "EOR", AddressingMode::IndirectX, 2, 6);
    add_opcode(0x51, Cpu::eor, "EOR", AddressingMode::IndirectY, 2, 5);
    // INC instruction
    add_opcode(0xE6, Cpu::inc, "INC", AddressingMode::ZeroPage, 2, 5);
    add_opcode(0xF6, Cpu::inc, "INC", AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0xEE, Cpu::inc, "INC", AddressingMode::Absolute, 3, 6);
    add_opcode(0xFE, Cpu::inc, "INC", AddressingMode::AbsoluteX, 3, 7);
    // INX instruction
    add_opcode(0xE8, Cpu::inx, "INX", AddressingMode::NoneAddressing, 1, 2);
    // INY instruction
    add_opcode(0xC8, Cpu::iny, "INY", AddressingMode::NoneAddressing, 1, 2);
    // JMP instruction
    add_opcode(0x4C, Cpu::jmp, "JMP", AddressingMode::Absolute, 3, 3);
    add_opcode(0x6C, Cpu::jmp, "JMP", AddressingMode::NoneAddressing, 3, 5);
    // JSR instruction
    add_opcode(0x20, Cpu::jsr, "JSR", AddressingMode::Absolute, 3, 6);
    // LDA instruction
    add_opcode(0xA9, Cpu::lda, "LDA", AddressingMode::Immediate, 2, 2);
    add_opcode(0xA5, Cpu::lda, "LDA", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB5, Cpu::lda, "LDA", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xAD, Cpu::lda, "LDA", AddressingMode::Absolute, 3, 4);
    add_opcode(0xBD, Cpu::lda, "LDA", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xB9, Cpu::lda, "LDA", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xA1, Cpu::lda, "LDA", AddressingMode::IndirectX, 2, 6);
    add_opcode(0xB1, Cpu::lda, "LDA", AddressingMode::IndirectY, 2, 5);
    // LDX instruction
    add_opcode(0xA2, Cpu::ldx, "LDX", AddressingMode::Immediate, 2, 2);
    add_opcode(0xA6, Cpu::ldx, "LDX", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB6, Cpu::ldx, "LDX", AddressingMode::ZeroPageY, 2, 4);
    add_opcode(0xAE, Cpu::ldx, "LDX", AddressingMode::Absolute, 3, 4);
    add_opcode(0xBE, Cpu::ldx, "LDX", AddressingMode::AbsoluteY, 3, 4);
    // LDY instruction
    add_opcode(0xA0, Cpu::ldy, "LDY", AddressingMode::Immediate, 2, 2);
    add_opcode(0xA4, Cpu::ldy, "LDY", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB4, Cpu::ldy, "LDY", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xAC, Cpu::ldy, "LDY", AddressingMode::Absolute, 3, 4);
    add_opcode(0xBC, Cpu::ldy, "LDY", AddressingMode::AbsoluteX, 3, 4);
    // LSR instruction
    add_opcode(0x4A, Cpu::lsr, "LSR", AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x46, Cpu::lsr, "LSR", AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x56, Cpu::lsr, "LSR", AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x4E, Cpu::lsr, "LSR", AddressingMode::Absolute, 3, 6);
    add_opcode(0x5E, Cpu::lsr, "LSR", AddressingMode::AbsoluteX, 3, 7);
    // NOP instruction
    add_opcode(0xEA, Cpu::nop, "NOP", AddressingMode::NoneAddressing, 1, 2);
    // ORA instruction
    add_opcode(0x09, Cpu::ora, "ORA", AddressingMode::Immediate, 2, 2);
    add_opcode(0x05, Cpu::ora, "ORA", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x15, Cpu::ora, "ORA", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x0D, Cpu::ora, "ORA", AddressingMode::Absolute, 3, 4);
    add_opcode(0x1D, Cpu::ora, "ORA", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x19, Cpu::ora, "ORA", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x01, Cpu::ora, "ORA", AddressingMode::IndirectX, 2, 6);
    add_opcode(0x11, Cpu::ora, "ORA", AddressingMode::IndirectY, 2, 5);
    // PHA instruction
    add_opcode(0x48, Cpu::pha, "PHA", AddressingMode::NoneAddressing, 1, 3);
    // PHP instruction
    add_opcode(0x08, Cpu::php, "PHP", AddressingMode::NoneAddressing, 1, 3);
    // PLA instruction
    add_opcode(0x68, Cpu::pla, "PLA", AddressingMode::NoneAddressing, 1, 4);
    // PLP instruction
    add_opcode(0x28, Cpu::plp, "PLP", AddressingMode::NoneAddressing, 1, 4);
    // ROL instruction
    add_opcode(0x2A, Cpu::rol, "ROL", AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x26, Cpu::rol, "ROL", AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x36, Cpu::rol, "ROL", AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x2E, Cpu::rol, "ROL", AddressingMode::Absolute, 3, 6);
    add_opcode(0x3E, Cpu::rol, "ROL", AddressingMode::AbsoluteX, 3, 7);
    // ROR instruction
    add_opcode(0x6A, Cpu::ror, "ROR", AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x66, Cpu::ror, "ROR", AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x76, Cpu::ror, "ROR", AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x6E, Cpu::ror, "ROR", AddressingMode::Absolute, 3, 6);
    add_opcode(0x7E, Cpu::ror, "ROR", AddressingMode::AbsoluteX, 3, 7);
    // RTI instruction
    add_opcode(0x40, Cpu::rti, "RTI", AddressingMode::NoneAddressing, 1, 6);
    // RTS instruction
    add_opcode(0x60, Cpu::rts, "RTS", AddressingMode::NoneAddressing, 1, 6);
    // SBC instruction
    add_opcode(0xE9, Cpu::sbc, "SBC", AddressingMode::Immediate, 2, 2);
    add_opcode(0xE5, Cpu::sbc, "SBC", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xF5, Cpu::sbc, "SBC", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xED, Cpu::sbc, "SBC", AddressingMode::Absolute, 3, 4);
    add_opcode(0xFD, Cpu::sbc, "SBC", AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xF9, Cpu::sbc, "SBC", AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xE1, Cpu::sbc, "SBC", AddressingMode::IndirectX, 2, 6);
    add_opcode(0xF1, Cpu::sbc, "SBC", AddressingMode::IndirectY, 2, 5);
    // SEC instruction
    add_opcode(0x38, Cpu::sec, "SEC", AddressingMode::NoneAddressing, 1, 2);
    // SED instruction
    add_opcode(0xF8, Cpu::sed, "SED", AddressingMode::NoneAddressing, 1, 2);
    // SEI instruction
    add_opcode(0x78, Cpu::sei, "SEI", AddressingMode::NoneAddressing, 1, 2);
    // STA instruction
    add_opcode(0x85, Cpu::sta, "STA", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x95, Cpu::sta, "STA", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x8D, Cpu::sta, "STA", AddressingMode::Absolute, 3, 4);
    add_opcode(0x9D, Cpu::sta, "STA", AddressingMode::AbsoluteX, 3, 5);
    add_opcode(0x99, Cpu::sta, "STA", AddressingMode::AbsoluteY, 3, 5);
    add_opcode(0x81, Cpu::sta, "STA", AddressingMode::IndirectX, 2, 6);
    add_opcode(0x91, Cpu::sta, "STA", AddressingMode::IndirectY, 2, 6);
    // STX instruction
    add_opcode(0x86, Cpu::stx, "STX", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x96, Cpu::stx, "STX", AddressingMode::ZeroPageY, 2, 4);
    add_opcode(0x8E, Cpu::stx, "STX", AddressingMode::Absolute, 3, 4);
    // STY instruction
    add_opcode(0x84, Cpu::sty, "STY", AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x94, Cpu::sty, "STY", AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x8C, Cpu::sty, "STY", AddressingMode::Absolute, 3, 4);
    // TAX instruction
    add_opcode(0xAA, Cpu::tax, "TAX", AddressingMode::NoneAddressing, 1, 2);
    // TAY instruction
    add_opcode(0xA8, Cpu::tay, "TAY", AddressingMode::NoneAddressing, 1, 2);
    // TSX instruction
    add_opcode(0xBA, Cpu::tsx, "TSX", AddressingMode::NoneAddressing, 1, 2);
    // TXA instruction
    add_opcode(0x8A, Cpu::txa, "TXA", AddressingMode::NoneAddressing, 1, 2);
    // TXS instruction
    add_opcode(0x9A, Cpu::txs, "TXS", AddressingMode::NoneAddressing, 1, 2);
    // TYA instruction
    add_opcode(0x98, Cpu::tya, "TYA", AddressingMode::NoneAddressing, 1, 2);
    opcodes
});
