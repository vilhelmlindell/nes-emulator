#![feature(core_intrinsics)]
#![feature(const_mut_refs)]

use core::time;
use cpu::Cpu;
use egui_macroquad::egui::{self, Color32, ColorImage, Context, Painter, TextureId};
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::*;
use frame::Frame;
use macroquad::ui::Id;
use memory_bus::MemoryBus;
use ppu::Ppu;
use rom::Rom;
use std::env;
use std::fs::File;
use std::io::{self, Read};

mod cpu;
mod frame;
mod instructions;
mod mapper;
mod memory_bus;
mod nes_tests;
mod opcodes;
mod ppu;
mod rom;

const WINDOW_SCALE: usize = 1;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_width: 256 * WINDOW_SCALE as i32,
        window_height: 240 * WINDOW_SCALE as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> io::Result<()> {
    // Access the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a file path as an argument
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Get the file path from the command-line argument
    let file_path = &args[1];

    let mut file = File::open(file_path)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    //let mut trace_file = File::create("trace.txt")?;

    let mut cpu = Cpu::new(MemoryBus::new(Rom::new(&bytes).expect("Failed to create rom")));

    let is_running = true;
    let draw_egui = true;

    clear_background(BLACK);

    loop {
        if !is_running {
            break;
        }

        cpu.instruction_cycle();

        draw_nes_screen(&cpu.memory_bus.ppu.frame.pixels);

        if draw_egui {
            egui_macroquad::ui(|egui_ctx| {
                egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                    let color_image = pattern_table_image(&cpu.memory_bus.ppu, false);
                    let texture_handle = egui_ctx.load_texture("color_image", color_image, egui::TextureOptions::NEAREST);
                    //ui.heading("My egui Application");
                    //ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                    //if ui.button("Increment").clicked() {
                    //}
                    //ui.label(format!("Hello '{name}', age {age}"));
                    ui.image(
                        texture_handle.id(),
                        texture_handle.size_vec2() * egui::Vec2::new(WINDOW_SCALE as f32, WINDOW_SCALE as f32),
                    );
                    ui.heading("My egui Application");
                });
            });

            egui_macroquad::draw();
        }

        //sleep(Duration::SECOND * 3);
        next_frame().await;
    }

    Ok(())
}

fn pattern_table_image(ppu: &Ppu, is_left: bool) -> egui::ColorImage {
    let start_address = if is_left { 0x0000 } else { 0x1000 };
    let mut pattern_table = egui::ColorImage::new([16 * 8, 16 * 8], egui::Color32::BLACK);
    for tile_y in 0..16 {
        for tile_x in 0..16 {
            let tile_address = start_address + tile_y * 16 * 16 + tile_x * 16;
            for y in 0..8 {
                let low_byte = ppu.read(tile_address + y);
                let high_byte = ppu.read(tile_address + y + 8);
                for x in 0..8 {
                    let low_bit = (low_byte & (1 << x)) >> x;
                    let high_bit = (high_byte & (1 << x)) >> x;
                    //let color_index = low_bit + high_bit;
                    let pixel_x = tile_x * 8 + (7 - x);
                    let pixel_y = tile_y * 8 + y;
                    //println!("x: {} y: {}", pixel_x, pixel_y);
                    pattern_table[(pixel_x as usize, pixel_y as usize)] = Color32::from_rgb(low_bit * 255, high_bit * 255, 0);
                }
            }
        }
    }
    pattern_table
}

fn draw_nes_screen(frame: &[[(u8, u8, u8); Frame::HEIGHT]; Frame::WIDTH]) {
    let mut image = macroquad::texture::Image::gen_image_color(Frame::WIDTH as u16, Frame::HEIGHT as u16, BLACK);

    for x in 0..Frame::WIDTH {
        for y in 0..Frame::HEIGHT {
            let color = frame[x][y];
            image.set_pixel(x as u32, y as u32, Color::from_rgba(color.0, color.1, color.2, 255))
        }
    }

    let texture = Texture2D::from_image(&image);
    let draw_params = DrawTextureParams {
        dest_size: Some(Vec2::new((Frame::WIDTH * WINDOW_SCALE) as f32, (Frame::HEIGHT * WINDOW_SCALE) as f32)),
        ..Default::default()
    };
    draw_texture_ex(texture, 0.0, 0.0, WHITE, draw_params);
}
