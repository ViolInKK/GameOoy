use std::{cell::RefCell, rc::Rc};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::databus::DataBus;

pub struct Ppu{
    colors: [Color; 4],

    databus: Rc<RefCell<DataBus>>,
}

impl Ppu {
    pub fn new(databus: Rc<RefCell<DataBus>>) -> Ppu {
        Ppu{
            colors: [
                     Color::RGB(0xFC, 0xEE, 0xE8),
                     Color::RGB(0xDC, 0x95, 0xA7),
                     Color::RGB(0x71, 0x5A, 0x9B),
                     Color::RGB(0x10, 0x10, 0x18),
            ],
                     //https://pixelcod.itch.io/ghost
            databus,
        }
    }

    fn get_sprite(&self, sprite_id: u8) -> [[u8; 8]; 8] {
        let mut sprite: [[u8; 8]; 8] = [[0b00; 8]; 8];
        for i in 0..16 {
            let byte = self.databus.borrow().read_memory(0x8000 + sprite_id as u16 * 16 + i);
            for j in 0..8 {
                if i % 2 == 0 {
                    sprite[j][(i/2) as usize] |= (byte & (0xFF >> j)) >> (7 - j); 
                }
                else {
                    sprite[j][(i/2) as usize] |= ((byte & (0xFF >> j)) >> (7 - j)) << 1;
                }
            }
        }
        sprite
    }

    pub fn draw_sprite(&self, canvas: &mut Canvas<Window>,sprite_id: u8) {
        let mut sprite_data: [[u8; 8]; 8] = self.get_sprite(sprite_id);
        for i in 0..8 {
            for j in 0..8 {
                canvas.set_draw_color(self.colors[sprite_data[i][j] as usize]);
                let _ = canvas.fill_rect(Rect::new((i*20) as i32, (j*20) as i32, 20, 20));
            }
        }
        canvas.present();
    }
}
