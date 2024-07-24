#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]

mod gameboy;
mod cpu;
mod cpu_instructions;
mod ppu;
mod apu;
mod databus;

use crate::gameboy::GameBoy;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub const DEBUG: bool = true;

fn main() {

    let SCREEN_WIDTH = 160;
    let SCREEN_HEIGHT = 144;
    let SCREEN_SCALE = 5;

    //let test = crate::cpu_instructions::BUH.get(&0x01);

    //println!("{:?}", test.unwrap().mnemonic);

   // let file = std::fs::read("../roms/Pokemon - Red Version (USA, Europe) (SGB Enhanced).gb").unwrap();
   // for (index, byte) in file.iter().enumerate() {
   //     println!("INDEX:{:#x} BYTE:{:#x}", index, byte);
   // }

    let mut gameboy = GameBoy::new();
    gameboy.exec_cycle();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    gameboy.ppu.draw_sprite(&mut canvas, 0);

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
    }
}
