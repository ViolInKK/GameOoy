pub struct Memory {
    pub entire_memory: [u8; 65535],
    ram: [u8; 8192],
    v_ram: [u8; 8192],
}

impl Memory {
    pub fn new() -> Memory {
        let mut entire_memory = [0; 65535];
        entire_memory[0x0100] = 0x00;
        Memory {
            entire_memory,
            ram: [0; 8192],
            v_ram: [0; 8192],
        }
    }
}

