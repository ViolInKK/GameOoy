use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::databus::DataBus;

const CPU_FREQUENCY: u32 = 4_213_440;
const FPS: u32 = 60;

const TIMA_ADDRESS: u16 = 0xFF05;
const TMA_ADDRESS:  u16 = 0xFF06;
const TAC_ADDRESS:  u16 = 0xFF07;
const IE_ADDRESS:   u16 = 0xFFFF;
const IF_ADDRESS:   u16 = 0xFF0F;


pub struct GameBoy<'a>{
    game_rom_path: String,

    cycles_this_frame: u32,

    divide_counter: u32,
    tima_counter: u32,
    clock_frequency: u32,

    /*
        1 = NOT pressed
        0 = pressed

        bit0: UP
        bit1: RIGHT
        bit2: DOWN
        bit3: LEFT
        bit4: A
        bit5: B
        bit6: SELECT
        bit7: START
    */
    joypad_state: u8,

    cpu: Cpu,
    pub ppu: Ppu<'a>,
    databus: Rc<RefCell<DataBus>>,
}

impl<'a> GameBoy<'a> {
    pub fn new(canvas: &mut Canvas<Window>, game_rom_path: String) -> GameBoy {
        let joypad_state = 0xFF;
        let databus: Rc<RefCell<DataBus>> = Rc::new(RefCell::new(DataBus::new()));
        GameBoy {
            game_rom_path,

            cycles_this_frame: 0,

            divide_counter: 0,
            tima_counter: 0,
            clock_frequency: 256,

            joypad_state,

            databus: Rc::clone(&databus),
            cpu: Cpu::new(Rc::clone(&databus)),
            ppu: Ppu::new(Rc::clone(&databus), canvas),
        }
    }

    pub fn update(&mut self) {
        let MAXCYCLES: u32 = CPU_FREQUENCY / FPS; // 70224 cpu cycles per frame
        while self.cycles_this_frame < MAXCYCLES {
            if !self.cpu.is_halted {
                let cycles = self.exec_next_instruction() as u32;
                self.cycles_this_frame += cycles;
                self.update_timers(cycles);
                self.ppu.update_graphics(cycles);
                self.do_interrupts();
                if self.cpu.pc == 0x0100 {
                    self.overwrite_boot_rom();
                }
            }
            else {
                self.do_interrupts();
            }
        }
        self.ppu.present();
        self.cycles_this_frame = 0;
    }

    fn request_interupt(&mut self, interupt_id: u8) {
        let IF = self.databus.borrow().read_memory(0xFF0F);
        let updated_IF = IF | (1 << interupt_id);
        self.databus.borrow_mut().write_memory(updated_IF, 0xFF0F);
    }

    fn update_div_timer(&mut self, cycles: u32) {
        self.divide_counter += cycles;
        if self.divide_counter >= 255 {
            self.databus.borrow_mut().increment_div_timer();
            self.divide_counter = 0;
        }
    }

    fn update_clock_frequency(&mut self) {
        let TAC = self.databus.borrow().read_memory(TAC_ADDRESS);
        let CLOCK_SPEED = TAC & 0x03;

        match CLOCK_SPEED {
            0 => {
                self.clock_frequency = 256;
            }

            1 => {
                self.clock_frequency = 4;
            }

            2 => {
                self.clock_frequency = 16;
            }
            
            3 => {
                self.clock_frequency = 64;
            }

            _ => {
                panic!("UNREACHABLE. Non existing clock speed.");
            }
        }
    }

    fn update_timers(&mut self, cycles: u32) {
        self.update_div_timer(cycles);

        let mut databus_borrow = self.databus.borrow_mut();

        let TAC = databus_borrow.read_memory(TAC_ADDRESS);
        let TIMA_ENABLED: bool = ((TAC >> 2) & 1) != 0;

        let TMA = databus_borrow.read_memory(TMA_ADDRESS);

        if TIMA_ENABLED {
            self.tima_counter += cycles;
            if self.tima_counter >= self.clock_frequency {
                let current_TIMA = databus_borrow.read_memory(TIMA_ADDRESS);
                let result = current_TIMA.overflowing_add(1);
                if result.1 {
                    databus_borrow.write_memory(TMA, TIMA_ADDRESS);
                    drop(databus_borrow);
                    self.request_interupt(2);
                }
                else {
                    databus_borrow.write_memory(result.0, TIMA_ADDRESS);
                    drop(databus_borrow);
                }
                self.tima_counter = 0;
                self.update_clock_frequency();
            }
        }
    }

    fn do_interrupts(&mut self) {
        if self.cpu.IME_enabled {
            let IE = self.databus.borrow().read_memory(IE_ADDRESS);
            if IE > 0 {
                let IF = self.databus.borrow().read_memory(IF_ADDRESS);
                for i in 0..5 {
                    let is_enabled = ((IE >> i) & 0x01) != 0;
                    let is_requested = ((IF >> i) & 0x01) != 0;

                    if is_enabled && is_requested {
                        self.cpu.IME_enabled = false;
                        self.cpu.PUSH(self.cpu.pc);
                        let updated_IF = IF ^ (1 << i);
                        self.databus.borrow_mut().write_memory(updated_IF, IF_ADDRESS);
                        match i {
                            0 => {
                                //VBLANK interrupt
                                self.cpu.pc = 0x0040;
                            }

                            1 => {
                                //LCD interrupt
                                self.cpu.pc = 0x0048;
                            }

                            2 => {
                                //Serial interrupt
                                self.cpu.pc = 0x0050;
                            }

                            3 => {
                                //Joypad interrupt
                                self.cpu.pc = 0x0060;
                            }

                            _ => {
                                panic!("Non existing interrupt.");
                            }
                        }
                        self.cycles_this_frame = self.cycles_this_frame.wrapping_add(5);
                    }
                }
            }
        }
    }

    pub fn load_boot_rom(&mut self) {
        for (index, byte) in crate::boot_rom::BOOT_ROM.iter().enumerate() {
            self.databus.borrow_mut().load_boot_rom(index, *byte);
        }
    }

    fn overwrite_boot_rom(&mut self) {
        let file = std::fs::read(Path::new(&self.game_rom_path)).unwrap();

        for (index, byte) in file.iter().enumerate() {
            if index == 0x0100 {
                break;
            }
            self.databus.borrow_mut().load_rom(index, *byte);
        }
    }

    pub fn load_rom(&mut self) {
        let file = std::fs::read(Path::new(&self.game_rom_path)).unwrap();
        let cartridge_type = file.get(0x0147).unwrap();

        let rom_banks = file.get(0x0148).unwrap();
        let ram_banks = &0x02;

        match cartridge_type {
            0x00 => {
            }

            0x01..=0x03 => {
                self.databus.borrow_mut().MBC1 = true;
            }

            0x05..=0x06 => {
                self.databus.borrow_mut().MBC2 = true;
            }

            0x08..=0x0D => {
            }

            _ => {
                eprintln!("Not supported cartirdge type.");
            }
        }

        match ram_banks {

            0x00..=0x01 => {
            }

            0x02 => {
                self.databus.borrow_mut().RAM_banks_count = 1;
            }

            0x03 => {
                self.databus.borrow_mut().RAM_banks_count = 4;
            }

            0x04 => {
                self.databus.borrow_mut().RAM_banks_count = 16;
            }

            0x05 => {
                self.databus.borrow_mut().RAM_banks_count = 8;
            }

            _ => {
                panic!("Non supported amount of ram banks.");
            }
        }

        if *rom_banks > 0x08 {
            panic!("Non supported amount of rom banks.");
        }
        else {
            self.databus.borrow_mut().ROM_banks_count = u32::pow(2 ,1 + *rom_banks as u32);
        }
        
        let cartridge_ram = vec![0; (self.databus.borrow().RAM_banks_count as u32 * 0x2000) as usize];
        let cartridge_rom = vec![0; (self.databus.borrow().ROM_banks_count * 0x4000) as usize];
        self.databus.borrow_mut().cartridge_ram = cartridge_ram;
        self.databus.borrow_mut().cartridge_rom = cartridge_rom;

        for (index, byte) in file.iter().enumerate() {
            self.databus.borrow_mut().load_rom(index, *byte);
        }
    }

    pub fn key_pressed(&mut self, key_id: u8) {
        let input_modes = (self.databus.borrow().read_memory(0xFF00) >> 4) & 0x03;
        let dpad_mode: bool = (input_modes & 0x01) == 0;
        let buttons_mode: bool = ((input_modes >> 1) & 0x01) == 0;

        let pressed_before: bool = ((self.joypad_state >> key_id) & 0x01) == 0;

        self.joypad_state &= !(1 << key_id);
        self.databus.borrow_mut().joypad_state = self.joypad_state;

        if dpad_mode && !pressed_before && key_id < 4 {
            self.request_interupt(4);
        }

        if buttons_mode && !pressed_before && key_id > 3 {
            self.request_interupt(4);
        }
    }

    pub fn key_released(&mut self, key_id: u8) {
        self.joypad_state |= 1 << key_id;
        self.databus.borrow_mut().joypad_state = self.joypad_state;
    }

    fn exec_next_instruction(&mut self) -> u8 {
        let instruction_byte = self.databus.borrow().read_memory(self.cpu.pc);
        self.cpu.exec_instruction(instruction_byte)
    }
}
