#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory_bus::Bus};
    use std::fs;

    #[test]
    fn test_official_instructions() {
        let rom = fs::read("tests/bin_files/6502_functional_test.bin").expect("Invalid file");
        let instructions = disasm6502::from_array(&rom).unwrap();
        for i in 380..450 {
            println!("{}", instructions[i]);
        }
        let mut cpu = Cpu::new();
        cpu.write_bytes(0, rom.as_slice());
        cpu.pc = 0x0400;
        cpu.run();
    }
}
