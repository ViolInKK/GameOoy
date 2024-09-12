use std::{cell::RefCell, rc::Rc};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{databus::DataBus, SCREEN_SCALE};

pub struct Ppu <'a> {
    colors: [Color; 4],

    current_line_cycles: u16,

    canvas: &'a mut Canvas<Window>,

    databus: Rc<RefCell<DataBus>>,
}

impl<'a> Ppu<'a> {
    pub fn new(databus: Rc<RefCell<DataBus>>, canvas: &mut Canvas<Window>) -> Ppu {
        Ppu{
            //https://pixelcod.itch.io/ghost
            colors: [
                     Color::RGB(0xFC, 0xEE, 0xE8),
                     Color::RGB(0xDC, 0x95, 0xA7),
                     Color::RGB(0x71, 0x5A, 0x9B),
                     Color::RGB(0x10, 0x10, 0x18),
            ],

            current_line_cycles: 456,

            canvas,

            databus,
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32) {
        let _ = self.canvas.fill_rect(Rect::new(x * SCREEN_SCALE as i32,y * SCREEN_SCALE as i32 , SCREEN_SCALE, SCREEN_SCALE));
    }

    fn updated_STAT(&mut self) {
        let STAT: u8 = self.databus.borrow().read_memory(0xFF41);
        let LCDC: u8 = self.databus.borrow().read_memory(0xFF40);
        let LYC: u8 = self.databus.borrow().read_memory(0xFF45);

        if ((LCDC >> 7) & 0x01) == 0 {
            self.current_line_cycles = 0;
            self.databus.borrow_mut().write_memory(0, 0xFF44);
            let updated_STAT: u8 = (STAT & 0xFC) | 0x01;
            self.databus.borrow_mut().write_memory(updated_STAT, 0xFF41);
            return;
        }

        let current_mode = STAT & 0x03;
        let new_mode: u8;
        let mut reqint = false;
        let LY = self.databus.borrow().read_memory(0xFF44);

        let mut updated_STAT: u8;

        if LY >= 144 {
            updated_STAT = (STAT & 0xFC) | 0x01;
            new_mode = 1;
            reqint = ((STAT >> 4) & 0x01) != 0;
        }
        else if (0..80).contains(&self.current_line_cycles) {
            updated_STAT = (STAT & 0xFC) | 0x10;
            new_mode = 2;
            reqint = ((STAT >> 5) & 0x01) != 0;
        }

        else if (80..245).contains(&self.current_line_cycles) {
            updated_STAT = (STAT & 0xFC) | 0x11;
            new_mode = 3;
        }
        else {
            updated_STAT = STAT & 0xFC;
            new_mode = 0;
            reqint = ((STAT >> 3) & 0x01) != 0;
        }

        if reqint && (current_mode != new_mode) {
                let IF = self.databus.borrow().read_memory(0xFF0F);
                let updated_IF = IF | 1;
                self.databus.borrow_mut().write_memory(updated_IF, 0xFF0F);
        }

        if LY == LYC {
            if ((STAT >> 6) & 0x01) == 1 {
                let IF = self.databus.borrow().read_memory(0xFF0F);
                let updated_IF = IF | 1;
                self.databus.borrow_mut().write_memory(updated_IF, 0xFF0F);
            }
        }
        else {
            updated_STAT &= !(1 << 2);
        }

        self.databus.borrow_mut().write_memory(updated_STAT, 0xFF41);

    }

    fn render_tiles(&mut self) {
        let databus_borrow = self.databus.borrow();
        let LCDC = databus_borrow.read_memory(0xFF40);

        let tile_data: u16;
        let background_map: u16;
        let mut unsig: bool = true;

        let SCY = databus_borrow.read_memory(0xFF42);
        let SCX = databus_borrow.read_memory(0xFF43);
        let WY = databus_borrow.read_memory(0xFF4A);
        let WX = databus_borrow.read_memory(0xFF4B).wrapping_sub(7);
        let LY = databus_borrow.read_memory(0xFF44);

        let mut using_window: bool = false;

        if (((LCDC >> 5) & 0x01) == 1) && (WY <= LY) {
            using_window = true;

        }

        if ((LCDC >> 4) & 0x01) == 1 {
            tile_data = 0x8000;
        }
        else {
            tile_data = 0x8800;
            unsig = false;
        }

        if !using_window {
            if ((LCDC >> 3) & 0x01) == 1 {
                background_map = 0x9C00;
            }
            else {
                background_map = 0x9800;
            }
        }
        else {
            if ((LCDC >> 6) & 0x01) == 1 {
                background_map = 0x9C00;
            }
            else {
                background_map = 0x9800;
            }
        }

        let tile_row: u16;

        if !using_window {
            tile_row = 32 * (((LY as u16 + SCY as u16) as u16 & 0xFF) / 8);
        }
        else {
            tile_row = 32 * (((LY - WY) as u16 & 0xFF) / 8);
        }

        for pixel in 0..160 {
            let mut tile_col = ((SCX + pixel) as u16 / 8) & 0x1f;

            if using_window && (pixel >= WX) {
                tile_col = ((pixel - WX) as u16 / 8) & 0x1f;
            }

            let tile_address: u16 = background_map + tile_row + tile_col;
            let tile_num = databus_borrow.read_memory(tile_address);

            let tile_location: u16;

            if unsig {
                tile_location = tile_data + (tile_num as u16 * 16);
            }
            else {
                if (tile_num as i8) < 0 {
                    tile_location = tile_data - (tile_num as u16 * 16);
                }
                else {
                    tile_location = tile_data + (tile_num as u16 * 16);
                }
            }

            let line = ((SCY as u16 + LY as u16) % 8) * 2;
            let data1 = databus_borrow.read_memory(tile_location + line as u16);
            let data2 = databus_borrow.read_memory(tile_location + line as u16 + 1);

            let mut colour_bit: i8 = ((pixel + SCX) % 8).try_into().unwrap();
            colour_bit -= 7;
            colour_bit *= -1;

            let data1_bit = (data1 >> colour_bit) & 0x01;
            let data2_bit = (data2 >> colour_bit) & 0x01;
            let final_colour = (data2_bit << 1) | data1_bit;
            self.canvas.set_draw_color(self.colors[final_colour as usize]);
            let _ = self.canvas.fill_rect(Rect::new(pixel as i32 * SCREEN_SCALE as i32,LY as i32 * SCREEN_SCALE as i32 , SCREEN_SCALE, SCREEN_SCALE));
        }
    }

    fn render_sprites(&mut self) {
        let mut use8x16: bool = false;
        let databus_borrow = self.databus.borrow_mut();

        for sprite in 0..40_u16 {
            let index = sprite*4;
            let yPos = databus_borrow.read_memory(0xFE00 + index).wrapping_sub(16);
            let xPos = databus_borrow.read_memory(0xFE00 + index + 1).wrapping_sub(8);
            let tile_location = databus_borrow.read_memory(0xFE00 + index + 2);
            let tile_attributes = databus_borrow.read_memory(0xFE00 + index + 3);

            let xFlip = ((tile_attributes >> 5) & 0x01) == 1;
            let yFlip = ((tile_attributes >> 6) & 0x01) == 1;

            let ysize = 8;
            let LY = databus_borrow.read_memory(0xFF44);

            if (LY >= yPos) && (LY < (yPos + ysize)) {
                let mut line: i16 = (LY as i16 - yPos as i16) * 2;

                if yFlip {
                    line = line.wrapping_sub(ysize as i16);
                    line *= -1;
                }

                let data_address: u16 = (0x8000 + (tile_location as u16 * 16)) + line as u16;
                let data1 = databus_borrow.read_memory(data_address);
                let data2 = databus_borrow.read_memory(data_address + 1);

                for pixel in (0..7_i8).rev() {
                    let mut colour_bit = pixel;
                    if xFlip {
                        colour_bit -= 7;
                        colour_bit *= -1;
                    }

                    let data1_bit = (data1 >> colour_bit) & 0x01;
                    let data2_bit = (data2 >> colour_bit) & 0x01;
                    let final_colour = (data2_bit << 1) | data1_bit;

                    //If color white dont draw; transparent.
                    if final_colour == 0 {
                        continue;
                    }

                    let xPix = (7 - pixel as u8) + xPos;

                    self.canvas.set_draw_color(self.colors[final_colour as usize]);
                    let _ = self.canvas.fill_rect(Rect::new(xPix as i32 * SCREEN_SCALE as i32,LY as i32 * SCREEN_SCALE as i32 , SCREEN_SCALE, SCREEN_SCALE));
                }
            }
        }
    }

    fn draw_scanline(&mut self) {
        let LCDC = self.databus.borrow().read_memory(0xFF40);

        if (LCDC & 0x01) == 1 {
            self.render_tiles();
        }

        if ((LCDC >> 1) & 0x01) == 1 {
            self.render_sprites();
        }
    }

    pub fn update_graphics(&mut self, cycles: u32) {
        //Update status.
        self.updated_STAT();
        let LCDC: u8 = self.databus.borrow().read_memory(0xFF40);
        //If LCD disabled return.
        if ((LCDC >> 7) & 0x01) == 0 {
            return;
        }

        self.current_line_cycles += cycles as u16;

        if self.current_line_cycles >= 456 {
            let LY = self.databus.borrow().read_memory(0xFF44);
            if LY < 144 {
                self.draw_scanline();
            }

            else if LY == 144 {
                //Request interrup.
                let IF = self.databus.borrow().read_memory(0xFF0F);
                let updated_IF = IF | 1;
                self.databus.borrow_mut().write_memory(updated_IF, 0xFF0F);
            }

            else if LY > 153 {
                self.databus.borrow_mut().write_memory(0, 0xFF44);
                return;
            }


            self.current_line_cycles = 0;
            self.databus.borrow_mut().write_memory(LY + 1, 0xFF44);
            //Increment LY. Check if needs to be resetted a.k.a. if LY > 153.
            //Reset self cureent line.
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    fn get_tile_data_low(&self, sprite_id: u8) -> u8 {
        let LY = self.databus.borrow().read_memory(0xFF44);
        self.databus.borrow().read_memory(0x8000 + (sprite_id * 16) as u16 + (2 * (LY % 8)) as u16)
    }

    fn get_tile_data_high(&self, sprite_id: u8) -> u8 {
        let LY = self.databus.borrow().read_memory(0xFF44);
        self.databus.borrow().read_memory(0x8001 + (sprite_id * 16) as u16 + (2 * (LY % 8)) as u16)
    }
}
