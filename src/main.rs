#![feature(core_intrinsics)]
#![feature(const_mut_refs)]

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

use cpu::Cpu;
use memory_bus::MemoryBus;
use rom::Rom;

use crate::memory_bus::Bus;

mod cpu;
mod instructions;
mod mapper;
mod memory_bus;
mod nes_tests;
mod opcodes;
mod rom;

fn main() -> io::Result<()> {
    // Access the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a file path as an argument
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Get the file path from the command-line argument
    let file_path = &args[1];

    // Open the file for reading
    let mut file = File::open(file_path)?;

    // Read the file's contents as raw bytes
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Create a file to write the trace
    let mut trace_file = File::create("trace.txt")?; // Adjust the file name and path as needed

    // Run the emulator
    let mut cpu = Cpu::new(MemoryBus::new(Rom::new(&bytes).expect("Failed to create rom")));
    cpu.pc = 0xC000;

    let max_steps = 2000;

    for _ in 0..max_steps {
        let instruction = cpu.fetch();
        let trace = cpu.execution_trace(&instruction);

        // Write the trace to the file
        trace_file.write_all(trace.as_bytes())?;
        trace_file.write_all(b"\n")?; // Add a newline after writing the trace

        cpu.execute(&instruction);
    }

    Ok(())
}
