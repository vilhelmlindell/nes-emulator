use crate::cpu::AddressingMode;
use crate::instructions::InstructionSet;
use once_cell::sync::Lazy;

pub static CPU_OPCODES: Lazy<[Option<Instruction>; MAX_OPCODES]> = Lazy::new(initialize_opcodes);
const MAX_OPCODES: usize = 0xFF;

#[derive(Clone)]
pub struct Instruction {
    pub name: &'static str,
    pub function: fn(&mut (dyn InstructionSet + 'static), AddressingMode),
    pub addressing_mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction {
    fn new(
        name: &'static str,
        function: fn(&mut (dyn InstructionSet + 'static), AddressingMode),
        addressing_mode: AddressingMode,
        bytes: u8,
        cycles: u8,
    ) -> Self {
        Self {
            name,
            function,
            addressing_mode,
            bytes,
            cycles,
        }
    }
}

fn initialize_opcodes() -> [Option<Instruction>; MAX_OPCODES] {
    let mut opcodes: [Option<Instruction>; MAX_OPCODES] = std::array::from_fn(|_| None);

    let mut add_opcode = |opcode: u8,
                          name: &'static str,
                          instruction: fn(&mut (dyn InstructionSet + 'static), AddressingMode),
                          addressing_mode: AddressingMode,
                          bytes: u8,
                          cycles: u8| {
        opcodes[opcode as usize] = Some(Instruction::new(name, instruction, addressing_mode, bytes, cycles));
    };
    // ADC instruction
    add_opcode(0x69, "ADC", InstructionSet::adc, AddressingMode::Immediate, 2, 2);
    add_opcode(0x65, "ADC", InstructionSet::adc, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x75, "ADC", InstructionSet::adc, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x6D, "ADC", InstructionSet::adc, AddressingMode::Absolute, 3, 4);
    add_opcode(0x7D, "ADC", InstructionSet::adc, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x79, "ADC", InstructionSet::adc, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x61, "ADC", InstructionSet::adc, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x71, "ADC", InstructionSet::adc, AddressingMode::IndirectY, 2, 5);

    // AND instruction
    add_opcode(0x29, "AND", InstructionSet::and, AddressingMode::Immediate, 2, 2);
    add_opcode(0x25, "AND", InstructionSet::and, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x35, "AND", InstructionSet::and, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x2D, "AND", InstructionSet::and, AddressingMode::Absolute, 3, 4);
    add_opcode(0x3D, "AND", InstructionSet::and, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x39, "AND", InstructionSet::and, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x21, "AND", InstructionSet::and, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x31, "AND", InstructionSet::and, AddressingMode::IndirectY, 2, 5);

    // ASL instruction
    add_opcode(0x0A, "ASL", InstructionSet::asl, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x06, "ASL", InstructionSet::asl, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x16, "ASL", InstructionSet::asl, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x0E, "ASL", InstructionSet::asl, AddressingMode::Absolute, 3, 6);
    add_opcode(0x1E, "ASL", InstructionSet::asl, AddressingMode::AbsoluteX, 3, 7);
    // BCC instruction
    add_opcode(0x90, "BCC", InstructionSet::bcc, AddressingMode::Relative, 2, 2);
    // BCS instruction
    add_opcode(0xB0, "BCS", InstructionSet::bcs, AddressingMode::Relative, 2, 2);
    // BEQ instruction
    add_opcode(0xF0, "BEQ", InstructionSet::beq, AddressingMode::Relative, 2, 2);
    // BIT instruction
    add_opcode(0x24, "BIT", InstructionSet::bit, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x2C, "BIT", InstructionSet::bit, AddressingMode::Absolute, 3, 4);
    // BMI instruction
    add_opcode(0x30, "BMI", InstructionSet::bmi, AddressingMode::Relative, 2, 2);
    // BNE instruction
    add_opcode(0xD0, "BNE", InstructionSet::bne, AddressingMode::Relative, 2, 2);
    // BPL instruction
    add_opcode(0x10, "BPL", InstructionSet::bpl, AddressingMode::Relative, 2, 2);
    // BRK instruction
    add_opcode(0x00, "BRK", InstructionSet::brk, AddressingMode::NoneAddressing, 1, 7);
    // BVC instruction
    add_opcode(0x50, "BVC", InstructionSet::bvc, AddressingMode::Relative, 2, 2);
    // BVS instruction
    add_opcode(0x70, "BVS", InstructionSet::bvs, AddressingMode::Relative, 2, 2);
    // CLC instruction
    add_opcode(0x18, "CLC", InstructionSet::clc, AddressingMode::NoneAddressing, 1, 2);
    // CLD instruction
    add_opcode(0xD8, "CLD", InstructionSet::cld, AddressingMode::NoneAddressing, 1, 2);
    // CLI instruction
    add_opcode(0x58, "CLI", InstructionSet::cli, AddressingMode::NoneAddressing, 1, 2);
    // CLV instruction
    add_opcode(0xB8, "CLV", InstructionSet::clv, AddressingMode::NoneAddressing, 1, 2);
    // CMP instruction
    add_opcode(0xC9, "CMP", InstructionSet::cmp, AddressingMode::Immediate, 2, 2);
    add_opcode(0xC5, "CMP", InstructionSet::cmp, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xD5, "CMP", InstructionSet::cmp, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xCD, "CMP", InstructionSet::cmp, AddressingMode::Absolute, 3, 4);
    add_opcode(0xDD, "CMP", InstructionSet::cmp, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xD9, "CMP", InstructionSet::cmp, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xC1, "CMP", InstructionSet::cmp, AddressingMode::IndirectX, 2, 6);
    add_opcode(0xD1, "CMP", InstructionSet::cmp, AddressingMode::IndirectY, 2, 5);
    // CPX instruction
    add_opcode(0xE0, "CPX", InstructionSet::cpx, AddressingMode::Immediate, 2, 2);
    add_opcode(0xE4, "CPX", InstructionSet::cpx, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xEC, "CPX", InstructionSet::cpx, AddressingMode::Absolute, 3, 4);
    // CPY instruction
    add_opcode(0xC0, "CPY", InstructionSet::cpy, AddressingMode::Immediate, 2, 2);
    add_opcode(0xC4, "CPY", InstructionSet::cpy, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xCC, "CPY", InstructionSet::cpy, AddressingMode::Absolute, 3, 4);
    // DEC instruction
    add_opcode(0xC6, "DEC", InstructionSet::dec, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0xD6, "DEC", InstructionSet::dec, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0xCE, "DEC", InstructionSet::dec, AddressingMode::Absolute, 3, 6);
    add_opcode(0xDE, "DEC", InstructionSet::dec, AddressingMode::AbsoluteX, 3, 7);
    // DEX instruction
    add_opcode(0xCA, "DEX", InstructionSet::dex, AddressingMode::NoneAddressing, 1, 2);
    // DEY instruction
    add_opcode(0x88, "DEY", InstructionSet::dey, AddressingMode::NoneAddressing, 1, 2);
    // EOR instruction
    add_opcode(0x49, "EOR", InstructionSet::eor, AddressingMode::Immediate, 2, 2);
    add_opcode(0x45, "EOR", InstructionSet::eor, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x55, "EOR", InstructionSet::eor, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x4D, "EOR", InstructionSet::eor, AddressingMode::Absolute, 3, 4);
    add_opcode(0x5D, "EOR", InstructionSet::eor, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x59, "EOR", InstructionSet::eor, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x41, "EOR", InstructionSet::eor, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x51, "EOR", InstructionSet::eor, AddressingMode::IndirectY, 2, 5);
    // INC instruction
    add_opcode(0xE6, "INC", InstructionSet::inc, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0xF6, "INC", InstructionSet::inc, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0xEE, "INC", InstructionSet::inc, AddressingMode::Absolute, 3, 6);
    add_opcode(0xFE, "INC", InstructionSet::inc, AddressingMode::AbsoluteX, 3, 7);
    // INX instruction
    add_opcode(0xE8, "INX", InstructionSet::inx, AddressingMode::NoneAddressing, 1, 2);
    // INY instruction
    add_opcode(0xC8, "INY", InstructionSet::iny, AddressingMode::NoneAddressing, 1, 2);
    // JMP instruction
    add_opcode(0x4C, "JMP", InstructionSet::jmp, AddressingMode::Absolute, 3, 3);
    add_opcode(0x6C, "JMP", InstructionSet::jmp, AddressingMode::Indirect, 3, 5);
    // JSR instruction
    add_opcode(0x20, "JSR", InstructionSet::jsr, AddressingMode::Absolute, 3, 6);
    // LDA instruction
    add_opcode(0xA9, "LDA", InstructionSet::lda, AddressingMode::Immediate, 2, 2);
    add_opcode(0xA5, "LDA", InstructionSet::lda, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB5, "LDA", InstructionSet::lda, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xAD, "LDA", InstructionSet::lda, AddressingMode::Absolute, 3, 4);
    add_opcode(0xBD, "LDA", InstructionSet::lda, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xB9, "LDA", InstructionSet::lda, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xA1, "LDA", InstructionSet::lda, AddressingMode::IndirectX, 2, 6);
    add_opcode(0xB1, "LDA", InstructionSet::lda, AddressingMode::IndirectY, 2, 5);
    // LDX instruction
    add_opcode(0xA2, "LDX", InstructionSet::ldx, AddressingMode::Immediate, 2, 2);
    add_opcode(0xA6, "LDX", InstructionSet::ldx, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB6, "LDX", InstructionSet::ldx, AddressingMode::ZeroPageY, 2, 4);
    add_opcode(0xAE, "LDX", InstructionSet::ldx, AddressingMode::Absolute, 3, 4);
    add_opcode(0xBE, "LDX", InstructionSet::ldx, AddressingMode::AbsoluteY, 3, 4);
    // LDY instruction
    add_opcode(0xA0, "LDY", InstructionSet::ldy, AddressingMode::Immediate, 2, 2);
    add_opcode(0xA4, "LDY", InstructionSet::ldy, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xB4, "LDY", InstructionSet::ldy, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xAC, "LDY", InstructionSet::ldy, AddressingMode::Absolute, 3, 4);
    add_opcode(0xBC, "LDY", InstructionSet::ldy, AddressingMode::AbsoluteX, 3, 4);
    // LSR instruction
    add_opcode(0x4A, "LSR", InstructionSet::lsr, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x46, "LSR", InstructionSet::lsr, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x56, "LSR", InstructionSet::lsr, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x4E, "LSR", InstructionSet::lsr, AddressingMode::Absolute, 3, 6);
    add_opcode(0x5E, "LSR", InstructionSet::lsr, AddressingMode::AbsoluteX, 3, 7);
    // NOP instruction
    add_opcode(0xEA, "NOP", InstructionSet::nop, AddressingMode::NoneAddressing, 1, 2);
    // ORA instruction
    add_opcode(0x09, "ORA", InstructionSet::ora, AddressingMode::Immediate, 2, 2);
    add_opcode(0x05, "ORA", InstructionSet::ora, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x15, "ORA", InstructionSet::ora, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x0D, "ORA", InstructionSet::ora, AddressingMode::Absolute, 3, 4);
    add_opcode(0x1D, "ORA", InstructionSet::ora, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0x19, "ORA", InstructionSet::ora, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0x01, "ORA", InstructionSet::ora, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x11, "ORA", InstructionSet::ora, AddressingMode::IndirectY, 2, 5);
    // PHA instruction
    add_opcode(0x48, "PHA", InstructionSet::pha, AddressingMode::NoneAddressing, 1, 3);
    // PHP instruction
    add_opcode(0x08, "PHP", InstructionSet::php, AddressingMode::NoneAddressing, 1, 3);
    // PLA instruction
    add_opcode(0x68, "PLA", InstructionSet::pla, AddressingMode::NoneAddressing, 1, 4);
    // PLP instruction
    add_opcode(0x28, "PLP", InstructionSet::plp, AddressingMode::NoneAddressing, 1, 4);
    // ROL instruction
    add_opcode(0x2A, "ROL", InstructionSet::rol, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x26, "ROL", InstructionSet::rol, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x36, "ROL", InstructionSet::rol, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x2E, "ROL", InstructionSet::rol, AddressingMode::Absolute, 3, 6);
    add_opcode(0x3E, "ROL", InstructionSet::rol, AddressingMode::AbsoluteX, 3, 7);
    // ROR instruction
    add_opcode(0x6A, "ROR", InstructionSet::ror, AddressingMode::NoneAddressing, 1, 2);
    add_opcode(0x66, "ROR", InstructionSet::ror, AddressingMode::ZeroPage, 2, 5);
    add_opcode(0x76, "ROR", InstructionSet::ror, AddressingMode::ZeroPageX, 2, 6);
    add_opcode(0x6E, "ROR", InstructionSet::ror, AddressingMode::Absolute, 3, 6);
    add_opcode(0x7E, "ROR", InstructionSet::ror, AddressingMode::AbsoluteX, 3, 7);
    // RTI instruction
    add_opcode(0x40, "RTI", InstructionSet::rti, AddressingMode::NoneAddressing, 1, 6);
    // RTS instruction
    add_opcode(0x60, "RTS", InstructionSet::rts, AddressingMode::NoneAddressing, 1, 6);
    // SBC instruction
    add_opcode(0xE9, "SBC", InstructionSet::sbc, AddressingMode::Immediate, 2, 2);
    add_opcode(0xE5, "SBC", InstructionSet::sbc, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0xF5, "SBC", InstructionSet::sbc, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0xED, "SBC", InstructionSet::sbc, AddressingMode::Absolute, 3, 4);
    add_opcode(0xFD, "SBC", InstructionSet::sbc, AddressingMode::AbsoluteX, 3, 4);
    add_opcode(0xF9, "SBC", InstructionSet::sbc, AddressingMode::AbsoluteY, 3, 4);
    add_opcode(0xE1, "SBC", InstructionSet::sbc, AddressingMode::IndirectX, 2, 6);
    add_opcode(0xF1, "SBC", InstructionSet::sbc, AddressingMode::IndirectY, 2, 5);
    // SEC instruction
    add_opcode(0x38, "SEC", InstructionSet::sec, AddressingMode::NoneAddressing, 1, 2);
    // SED instruction
    add_opcode(0xF8, "SED", InstructionSet::sed, AddressingMode::NoneAddressing, 1, 2);
    // SEI instruction
    add_opcode(0x78, "SEI", InstructionSet::sei, AddressingMode::NoneAddressing, 1, 2);
    // STA instruction
    add_opcode(0x85, "STA", InstructionSet::sta, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x95, "STA", InstructionSet::sta, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x8D, "STA", InstructionSet::sta, AddressingMode::Absolute, 3, 4);
    add_opcode(0x9D, "STA", InstructionSet::sta, AddressingMode::AbsoluteX, 3, 5);
    add_opcode(0x99, "STA", InstructionSet::sta, AddressingMode::AbsoluteY, 3, 5);
    add_opcode(0x81, "STA", InstructionSet::sta, AddressingMode::IndirectX, 2, 6);
    add_opcode(0x91, "STA", InstructionSet::sta, AddressingMode::IndirectY, 2, 6);
    // STX instruction
    add_opcode(0x86, "STX", InstructionSet::stx, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x96, "STX", InstructionSet::stx, AddressingMode::ZeroPageY, 2, 4);
    add_opcode(0x8E, "STX", InstructionSet::stx, AddressingMode::Absolute, 3, 4);
    // STY instruction
    add_opcode(0x84, "STY", InstructionSet::sty, AddressingMode::ZeroPage, 2, 3);
    add_opcode(0x94, "STY", InstructionSet::sty, AddressingMode::ZeroPageX, 2, 4);
    add_opcode(0x8C, "STY", InstructionSet::sty, AddressingMode::Absolute, 3, 4);
    // TAX instruction
    add_opcode(0xAA, "TAX", InstructionSet::tax, AddressingMode::NoneAddressing, 1, 2);
    // TAY instruction
    add_opcode(0xA8, "TAY", InstructionSet::tay, AddressingMode::NoneAddressing, 1, 2);
    // TSX instruction
    add_opcode(0xBA, "TSX", InstructionSet::tsx, AddressingMode::NoneAddressing, 1, 2);
    // TXA instruction
    add_opcode(0x8A, "TXA", InstructionSet::txa, AddressingMode::NoneAddressing, 1, 2);
    // TXS instruction
    add_opcode(0x9A, "TXS", InstructionSet::txs, AddressingMode::NoneAddressing, 1, 2);
    // TYA instruction
    add_opcode(0x98, "TYA", InstructionSet::tya, AddressingMode::NoneAddressing, 1, 2);
    opcodes
}
