#![feature(core_intrinsics)]
#![feature(const_mut_refs)]

use cpu::Cpu;
use memory_bus::MemoryBus;
use rom::Rom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
mod cpu;
mod frame;
mod instructions;
mod mapper;
mod memory_bus;
mod nes_tests;
mod opcodes;
mod ppu;
mod rom;

fn main() -> io::Result<()> {
    // Access the command-line arguments
    //let args: Vec<String> = env::args().collect();

    //// Check if the user provided a file path as an argument
    //if args.len() != 2 {
    //    eprintln!("Usage: {} <file_path>", args[0]);
    //    std::process::exit(1);
    //}

    //// Get the file path from the command-line argument
    //let file_path = &args[1];

    //let mut file = File::open(file_path)?;

    //let mut bytes = Vec::new();
    //file.read_to_end(&mut bytes)?;

    //let mut trace_file = File::create("trace.txt")?;

    //let mut cpu = Cpu::new(MemoryBus::new(Rom::new(&bytes).expect("Failed to create rom")));
    //cpu.pc = 0xC000;

    //let max_steps = 7000;

    //for _ in 0..max_steps {
    //    let instruction = cpu.fetch();
    //    let trace = cpu.execution_trace(&instruction);

    //    trace_file.write_all(trace.as_bytes())?;
    //    trace_file.write_all(b"\n")?; // Add a newline after writing the trace

    //    cpu.execute(&instruction);
    //}

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
