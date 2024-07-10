use std::rc::Rc;

use crate::memory::Memory;

pub struct DataBus{
    memory: Memory,
}
impl DataBus {
    pub fn new() -> DataBus {
        DataBus {
            memory: Memory::new(),
        }
    }

    pub fn read_memory(&self, addr: u16) -> u8{
        self.memory.entire_memory[addr as usize]
    }

    pub fn write_memory(&self, memory: &mut Memory, addr: u16){

    }
}
