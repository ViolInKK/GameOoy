use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::apu::Apu;
use crate::databus::DataBus;

pub struct GameBoy{
    cpu: Cpu,
    memory: Box<Memory>,
    ppu: Ppu,
    apu: Apu,
    databus: DataBus
}

impl GameBoy {
    pub fn new() -> GameBoy {
        GameBoy {
            cpu: Cpu::new(),
            memory: Box::new(Memory::new()),
            ppu: Ppu,
            apu: Apu,
            databus: DataBus,
        }
    }

    pub fn exec_cycle(&mut self){
        //fetch instruction
        //Decode instruction into commands
        //execute command
        let _instruction_byte = self.databus.read_memory(&self.memory, 0x0000);
    }
}

