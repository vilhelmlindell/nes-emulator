#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory_bus::MemoryBus, rom::Rom};
    use std::fs;

    //#[test]
    //fn test_nestest() {
    //    let test_bin_path = "nestest.nes";

    //    let rom = fs::read(test_bin_path).expect("Invalid file");
    //    let mut cpu = Cpu::new(MemoryBus::new(Rom::new(rom.as_slice()).unwrap()));
    //    cpu.pc = 0xC000;

    //    for i in 0..500 {
    //        cpu.step();
    //    }
    //}
}
