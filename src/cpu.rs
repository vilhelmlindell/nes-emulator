use std::intrinsics::wrapping_add;

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

impl From<Status> for usize {
    fn from(flag: Status) -> usize {
        match flag {
            Status::Carry => 0,
            Status::Zero => 1,
            Status::InterruptDisable => 2,
            Status::DecimalMode => 3,
            Status::Break => 4,
            Status::Overflow => 5,
            Status::Negative => 6,
        }
    }
}

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

pub struct Cpu {
    pc: u16,               // Program counter
    sp: u8,                // Stack pointer
    a: u8,                 // Accumulator
    x: u8,                 // X register
    y: u8,                 // Y register
    status: u8,            // Status register
    memory: [u8; 0x10000], // Memory
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
            memory: [0; 0x10000],
        }
    }
    fn reset(&mut self) {
        self.pc = self.read_word(0xFFFC);
        self.sp = 0xFD;
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_byte(self.pc) as usize;
            self.pc += 1;
            let instruction = CPU_OPCODES[opcode];
            instruction();
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
    // Function to write a byte to memory at a specified address
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    // Function to write a word (2 bytes) to memory at a specified address
    fn write_word(&mut self, address: u16, value: u16) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;

        self.memory[address as usize] = low_byte;
        self.memory[(address + 1) as usize] = high_byte;
    }

    // Function to get the value of a specific status flag
    fn status(&self, flag: Status) -> bool {
        let flag_bit = 1 << flag as u8;
        (self.status & flag_bit) != 0
    }

    // Function to set the value of a specific status flag
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
            AddressingMode::NoneAddressing => panic!("Mode doesn't support addresses"),
        }
    }

    // Instructions
    pub fn adc(&self, mode: &AddressingMode) {
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
    pub fn and(&self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        self.a &= operand;
        self.update_zero_and_negative_flags(self.a);
    }
    pub fn asl(&self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize];

        if self.a & 0b1000_0000 != 0 {
            self.set_status(Status::Carry, true);
        }

        self.a &= operand;
    }
    pub fn bcc(&self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if !self.status(Status::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bcs(&self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if self.status(Status::Carry) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn beq(&self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if self.status(Status::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bit(&self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[self.memory[operand_address as usize] as usize];
        let result = self.a & operand;

        self.update_zero_and_negative_flags(result);

        if 0b0100_0000 & result != 0 {
            self.set_status(Status::Overflow, true);
        }
    }
    pub fn bmi(&self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if self.status(Status::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bne(&self, mode: &AddressingMode) {
        let operand_address = self.pc;
        let operand = self.memory[operand_address as usize] as i8;

        if !self.status(Status::Zero) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn bpl(&self, mode: &AddressingMode) {
        let operand_address = self.get_operand_address(mode);
        let operand = self.memory[operand_address as usize] as i8;

        if !self.status(Status::Negative) {
            self.pc = self.pc.wrapping_add_signed(operand as i16);
        }
    }
    pub fn brk(&mut self, mode: &AddressingMode) {
        self.write_byte(self.sp as u16, self.pc as u8);
        self.write_byte(self.sp as u16, self.status as u8);
        self.pc = self.read_word(0xFFFE);
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
}
