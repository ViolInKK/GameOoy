use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::apu::Apu;
use crate::databus::DataBus;

pub struct GameBoy{
    cycles: u32,

    cpu: Cpu,
    pub ppu: Ppu,
    apu: Apu,
    databus: Rc<RefCell<DataBus>>,
}

impl GameBoy {
    pub fn new() -> GameBoy {
        let databus: Rc<RefCell<DataBus>> = Rc::new(RefCell::new(DataBus::new()));
        GameBoy {
            cycles: 0,

            databus: Rc::clone(&databus),
            cpu: Cpu::new(Rc::clone(&databus)),
            ppu: Ppu::new(Rc::clone(&databus)),
            apu: Apu,
        }
    }

    pub fn update(&mut self) {
        //let MAXCYCLES = 69905;
        //let cycles_this_update = 0;

        //while(cycles_this_update < MAXCYCLES)
        //{
        //    let cycles = ExecuteNextOpcode();
        //    cycles_this_update += cycles;
        //    updateTimers(cycles);
        //    updateGraphics(cycles);
        //    DoInterupts();
        //}

        let MAXCYCLES: u32 = 69905;
        let mut cycles_this_update = 0;
        while(cycles_this_update < MAXCYCLES)
        {
            cycles_this_update += self.exec_next_instruction() as u32;
        }
    }

    fn exec_next_instruction(&mut self) -> u8 {
        let instruction_byte = self.databus.borrow().read_memory(self.cpu.pc);
        self.cpu.exec_instruction(instruction_byte)
    }
}

