use raw_tty::GuardMode;

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory_bus::Bus};
    use std::fs;

    #[test]
    fn test_official_instructions() {
        let test_bin_path = "tests/bin_files/6502_functional_test.bin";
        let pc_start = 0x0400;
        // break at 40915
        let num_steps = 40330;

        let rom = fs::read(test_bin_path).expect("Invalid file");
        let mut cpu = Cpu::new();
        cpu.write_bytes(0, rom.as_slice());
        cpu.pc = pc_start;

        for _ in 0..num_steps {
            cpu.step(true);
        }
    }
}
