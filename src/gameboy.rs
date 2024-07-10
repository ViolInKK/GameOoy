use std::rc::Rc;

use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::apu::Apu;
use crate::databus::DataBus;

pub struct GameBoy{
    cycles: u32,

    cpu: Cpu,
    ppu: Ppu,
    apu: Apu,
    databus: Rc<DataBus>,
}

impl GameBoy {
    pub fn new() -> GameBoy {
        let databus: Rc<DataBus> = Rc::new(DataBus::new());
        GameBoy {
            cycles: 0,

            databus: Rc::clone(&databus),
            cpu: Cpu::new(Rc::clone(&databus)),
            ppu: Ppu,
            apu: Apu,
        }
    }

    pub fn exec_cycle(&mut self){
        //fetch instruction
        //Decode instruction into commands
        //execute command
 
        let instruction_byte = self.databus.read_memory(self.cpu.pc);
        self.cpu.exec_instruction(instruction_byte);
        self.cpu.pc += 1;

    }
}

