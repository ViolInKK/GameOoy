//#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]

mod gameboy;
mod cpu;
mod ppu;
mod memory;
mod apu;
mod databus;

use crate::gameboy::GameBoy;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {

    let gameboy = GameBoy::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running: bool = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();
    }
}
