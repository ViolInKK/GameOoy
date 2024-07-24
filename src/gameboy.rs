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

    pub fn exec_cycle(&mut self){
        let instruction_byte = self.databus.borrow().read_memory(self.cpu.pc);
        self.cpu.exec_instruction(instruction_byte);
    }
}

