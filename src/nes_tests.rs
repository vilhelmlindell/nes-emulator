#[cfg(test)]
mod tests {
    use crate::{
        cpu::Cpu,
        memory_bus::{Bus, MemoryBus},
        rom::Rom,
    };
    use std::fs;

    #[test]
    fn test_nestest() {
        let test_bin_path = "nestest.nes";
        let pc_start = 0xC000;
        let num_steps = 10;

        let rom = fs::read(test_bin_path).expect("Invalid file");
        let mut cpu = Cpu::new(MemoryBus::new(Rom::new(rom.as_slice()).unwrap()));
        cpu.pc = pc_start;

        for _ in 0..num_steps {
            cpu.step(true);
        }
    }
}
