use crate::memory::Memory;

pub struct DataBus;
impl DataBus {
    pub fn read_memory(&self, memory: &Memory, addr: u16) -> u8{
        memory.ram[addr as usize]
    }

    pub fn write_memory(&self, memory: &mut Memory, addr: u16){

    }
}
