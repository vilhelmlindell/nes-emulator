#![feature(core_intrinsics)]
#![feature(const_mut_refs)]

mod cpu;
mod instructions;
mod mapper;
mod memory_bus;
mod nes_tests;
mod opcodes;
mod rom;

fn main() {
    println!("Hello, world!");
}
