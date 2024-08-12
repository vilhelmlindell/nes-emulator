#![feature(core_intrinsics)]
#![feature(const_mut_refs)]

use cpu::Cpu;
use egui_macroquad::egui::{self, vec2, Color32, ColorImage, Context, Painter, TextureId};
use egui_macroquad::macroquad;
use egui_macroquad::macroquad::color::{Color, BLACK, WHITE};
use egui_macroquad::macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};
use egui_macroquad::macroquad::window::{clear_background, next_frame, Conf};
use frame::Frame;
use memory_bus::MemoryBus;
use ppu::{ControlFlags, Ppu};
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

const WINDOW_SCALE: usize = 4;

fn window_conf() -> Conf {
    Conf {
        window_title: "Nes Emulator".to_owned(),
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

    let mut cpu = Cpu::new(MemoryBus::new(Rom::new(&bytes).expect("Failed to create rom")));

    let is_running = true;
    let draw_egui = true;
    let mut nametable_index = 0;

    clear_background(BLACK);

    loop {
        if !is_running {
            break;
        }

        //loop {
        //    let prev_scanline = cpu.memory_bus.ppu.scanline;
        //    cpu.instruction_cycle();
        //    if cpu.memory_bus.ppu.scanline < prev_scanline {
        //        break;
        //    }
        //}

        draw_nes_screen(&cpu.memory_bus.ppu.frame.pixels);

        if draw_egui {
            egui_macroquad::ui(|egui_ctx| {
                egui::SidePanel::left("").show(egui_ctx, |ui| {
                    let left_pattern_table = pattern_table_image(&cpu.memory_bus.ppu, false);
                    let right_pattern_table = pattern_table_image(&cpu.memory_bus.ppu, true);
                    let pattern_tables = [left_pattern_table.clone(), right_pattern_table.clone()];
                    let nametable = nametable_image(&cpu.memory_bus.ppu, nametable_index, &pattern_tables);

                    let left_pattern_table_handle = egui_ctx.load_texture("left_pattern_table", left_pattern_table, egui::TextureOptions::NEAREST);
                    let right_pattern_table_handle = egui_ctx.load_texture("right_pattern_table", right_pattern_table, egui::TextureOptions::NEAREST);
                    let nametable_handle = egui_ctx.load_texture("nametable", nametable, egui::TextureOptions::NEAREST);

                    ui.collapsing("Timing", |ui| {
                        ui.label(format!("CPU {}", cpu.cycles));
                        let instruction = cpu.fetch();
                        ui.label(cpu.execution_trace(&instruction));
                        if ui.button("Step Instruction").clicked() {
                            cpu.instruction_cycle();
                        }
                        ui.label(format!("PPU {}, {}", cpu.memory_bus.ppu.scanline, cpu.memory_bus.ppu.cycle,))
                    });
                    ui.collapsing("Pattern Tables", |ui| {
                        ui.horizontal(|ui| {
                            ui.image(left_pattern_table_handle.id(), left_pattern_table_handle.size_vec2());
                            ui.image(right_pattern_table_handle.id(), right_pattern_table_handle.size_vec2());
                        });
                    });
                    ui.collapsing("Nametables", |ui| {
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut nametable_index, 0, "$2000");
                            ui.radio_value(&mut nametable_index, 1, "$2400");
                            ui.radio_value(&mut nametable_index, 2, "$2800");
                            ui.radio_value(&mut nametable_index, 3, "$2C00");
                        });
                        ui.image(nametable_handle.id(), nametable_handle.size_vec2());
                    });
                });
            });

            egui_macroquad::draw();
        }

        next_frame().await;
    }

    Ok(())
}

fn pattern_table_image(ppu: &Ppu, is_right: bool) -> egui::ColorImage {
    let start_address = if is_right { 0x1000 } else { 0x0000 };
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
fn nametable_image(ppu: &Ppu, nametable_index: u16, pattern_tables: &[ColorImage; 2]) -> ColorImage {
    let pattern_table_image = if ppu.control_register.contains(ControlFlags::BackgroundPatternTableAddress) {
        &pattern_tables[1]
    } else {
        &pattern_tables[0]
    };
    let start_address = 0x2000 + nametable_index * 0x400;
    let mut nametable_image = egui::ColorImage::new([32 * 8, 30 * 8], egui::Color32::BLACK);
    for tile_y in 0..30 {
        for tile_x in 0..32 {
            let nametable_address = start_address + tile_y * 16 + tile_x;
            let nametable_byte = ppu.read(nametable_address);
            let pattern_x = (nametable_byte % 16) * 8;
            let pattern_y = (nametable_byte / 16) * 8;
            //println!("{}", nametable_byte);
            for y in 0..8 {
                for x in 0..8 {
                    let pixel_x = pattern_x + x;
                    let pixel_y = pattern_y + y;
                    let nametable_x = tile_x as u8 * 8 + pixel_x;
                    let nametable_y = tile_y as u8 * 8 + pixel_y;
                    nametable_image[(nametable_x as usize, nametable_y as usize)] = pattern_table_image[(pixel_x as usize, pixel_y as usize)];
                }
            }
        }
    }
    nametable_image
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
        dest_size: Some(macroquad::math::Vec2::new(
            (Frame::WIDTH * WINDOW_SCALE) as f32,
            (Frame::HEIGHT * WINDOW_SCALE) as f32,
        )),
        ..Default::default()
    };
    draw_texture_ex(texture, 0.0, 0.0, WHITE, draw_params);
}
