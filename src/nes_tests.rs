#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory_bus::Bus};
    use std::fs;

    #[test]
    fn test_official_instructions() {
        let rom = fs::read("tests/6502_functional_test.bin").expect("Invalid file");
        let mut cpu = Cpu::new();
        cpu.write_bytes(0, rom.as_slice());
        cpu.pc = 0x0400;
        println!("{}", cpu.read(cpu.pc));
        cpu.run();
    }
}
