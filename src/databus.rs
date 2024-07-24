pub struct DataBus{
    memory: [u8; 65535],
}
impl DataBus {
    pub fn new() -> DataBus {
        let mut memory = [0; 65535];
        memory[0x0100] = 0x99;

        memory[0x0101] = 0x7F;
        memory[0x0102] = 0x44;

        memory[0x4455] = 0xBB;
        memory[0x4456] = 0xAA;

        memory[0xfffc] = 0xE8;
        memory[0xfffd] = 0xF9;

        memory[0x014d] = 0x23;

        memory[0x8000] = 0xFF;
        memory[0x8001] = 0x00;
        memory[0x8002] = 0x7E;
        memory[0x8003] = 0xFF;
        memory[0x8004] = 0x85;
        memory[0x8005] = 0x81;
        memory[0x8006] = 0x89;
        memory[0x8007] = 0x83;
        memory[0x8008] = 0x93;
        memory[0x8009] = 0x85;
        memory[0x800A] = 0xA5;
        memory[0x800B] = 0x8B;
        memory[0x800C] = 0xC9;
        memory[0x800D] = 0x97;
        memory[0x800E] = 0x7E;
        memory[0x800F] = 0xFF;
        DataBus {
            memory,
        }
    }

    pub fn read_memory(&self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    pub fn write_memory(&mut self, data: u8, addr: u16){
        self.memory[addr as usize] = data;
    }
}
