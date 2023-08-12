#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use std::fs;

    #[test]
    fn test_official_instructions() {
        let rom = fs::read("tests/instr_test-v5/official_only.nes").expect("Invalid file");
        println!("{:x?}", rom);
        let mut cpu = Cpu::new();
        cpu.load(&rom[..]);
    }
}
