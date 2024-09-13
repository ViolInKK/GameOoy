#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

mod gameboy;
mod cpu;
mod cpu_instructions;
mod ppu;
mod apu;
mod databus;
mod boot_rom;

use std::env;
use std::fs::metadata;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::gameboy::GameBoy;

pub const DEBUG: bool = false;

pub const SCREEN_WIDTH: u32  = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCREEN_SCALE: u32  = 5;

pub const UP: u8     = 2;
pub const RIGHT: u8  = 0;
pub const DOWN: u8   = 3;
pub const LEFT: u8   = 1;
pub const A: u8      = 4;
pub const B: u8      = 5;
pub const SELECT: u8 = 6;
pub const START: u8  = 7;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Correct usage: gameooy <game rom path>")
    }
    
    if !Path::new(&args[1]).exists() {
        panic!("Non existing faile path.");
    }

    let file_metadata = metadata(Path::new(&args[1])).unwrap();

    if file_metadata.is_dir() {
        panic!("Provided file is a directory.");
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("GameOoy", SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut gameboy = GameBoy::new(&mut canvas, args[1].clone());
    gameboy.load_rom();
    gameboy.load_boot_rom();

    let mut running: bool = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },

                Event::KeyDown { keycode: Some(Keycode::UP), .. } => {
                    gameboy.key_pressed(UP);
                }
                Event::KeyDown { keycode: Some(Keycode::DOWN), .. } => {
                    gameboy.key_pressed(DOWN);
                }
                Event::KeyDown { keycode: Some(Keycode::RIGHT), .. } => {
                    gameboy.key_pressed(RIGHT);
                }
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {
                    gameboy.key_pressed(LEFT);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    gameboy.key_pressed(A);
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    gameboy.key_pressed(B);
                }
                Event::KeyDown { keycode: Some(Keycode::RETURN), .. } => {
                    gameboy.key_pressed(SELECT);
                }
                Event::KeyDown { keycode: Some(Keycode::SPACE), .. } => {
                    gameboy.key_pressed(START);
                }

                ////

                Event::KeyUp { keycode: Some(Keycode::UP), .. } => {
                    gameboy.key_released(UP);
                }
                Event::KeyUp { keycode: Some(Keycode::DOWN), .. } => {
                    gameboy.key_released(DOWN);
                }
                Event::KeyUp { keycode: Some(Keycode::RIGHT), .. } => {
                    gameboy.key_released(RIGHT);
                }
                Event::KeyUp { keycode: Some(Keycode::LEFT), .. } => {
                    gameboy.key_released(LEFT);
                }
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    gameboy.key_released(A);
                }
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    gameboy.key_released(B);
                }
                Event::KeyUp { keycode: Some(Keycode::RETURN), .. } => {
                    gameboy.key_released(SELECT);
                }
                Event::KeyUp { keycode: Some(Keycode::SPACE), .. } => {
                    gameboy.key_released(START);
                }

                _ => {}
            }
        }
        gameboy.update();
    }
}
