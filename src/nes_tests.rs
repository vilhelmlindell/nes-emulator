#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory_bus::Bus};
    use std::fs;
    use std::io::Write;
    use std::process::{Command, Stdio};

    fn process_input(input: String, mut process: &mut std::process::Child) -> String {
        if let Some(ref mut stdin) = process.stdin {
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        }
        let output = process
            .wait_with_output()
            .expect("Failed to wait for output")
            .stdout;
        std::str::from_utf8(&output)
            .expect("Failed to convert output to string")
            .to_string()
    }
    #[test]
    fn test_official_instructions() {
        let test_bin_path = "tests/bin_files/6502_functional_test.bin";
        let pc_start = 0x0400;

        let mut command = Command::new("go6502");
        command.stdin(Stdio::piped());
        let mut process = command.spawn().expect("Failed to start go6502");

        process_input(format!("load {} $0000", test_bin_path), &mut process);
        process_input(format!("reg PC {:X}", pc_start), &mut process);

        let rom = fs::read(test_bin_path).expect("Invalid file");
        let mut cpu = Cpu::new();
        cpu.write_bytes(0, rom.as_slice());
        cpu.pc = pc_start;
    }
}
