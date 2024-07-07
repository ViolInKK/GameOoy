pub struct Memory {
    pub ram: [u8; 8192],
    pub v_ram: [u8; 8192],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 8192],
            v_ram: [0; 8192],
        }
    }
}

