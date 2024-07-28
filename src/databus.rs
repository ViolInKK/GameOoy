pub struct DataBus{
    memory: [u8; 65536],
}

impl DataBus {
    pub fn new() -> DataBus {
        let mut memory = [0; 65536];
        memory[0x0100] = 0xCB;
        memory[0x0101] = 0x33;

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

        //hardware registers initial values
        memory[0xFF00] = 0xCF;
        memory[0xFF02] = 0x7E;
        memory[0xFF04] = 0xAB;
        memory[0xFF07] = 0xF8;
        memory[0xFF0F] = 0xE1;
        memory[0xFF10] = 0x80;
        memory[0xFF11] = 0xBF;
        memory[0xFF12] = 0xF3;
        memory[0xFF13] = 0xF3;
        memory[0xFF14] = 0xBF;
        memory[0xFF16] = 0x3F;
        memory[0xFF18] = 0xFF;
        memory[0xFF19] = 0xBF;
        memory[0xFF1A] = 0x7F;
        memory[0xFF1B] = 0xFF;
        memory[0xFF1C] = 0x9F;
        memory[0xFF1D] = 0xFF;
        memory[0xFF1E] = 0xBF;
        memory[0xFF20] = 0xFF;
        memory[0xFF23] = 0xBF;
        memory[0xFF24] = 0x77;
        memory[0xFF25] = 0xF3;
        memory[0xFF26] = 0xF1;
        memory[0xFF40] = 0x91;
        memory[0xFF41] = 0x85;
        memory[0xFF46] = 0xFF;
        memory[0xFF47] = 0xFC;

        DataBus {
            memory,
        }
    }

    pub fn read_memory(&self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    pub fn write_memory(&mut self, data: u8, addr: u16){
        match addr {
            0x0000..=0x7FFF => {
                println!("ROM");
            }

            0xE000..=0xFDFF => {
                self.memory[addr as usize] = data;
                self.write_memory(data, addr-0x2000);
                println!("ECHO MEMORY LOCATION");
            }

            0xFEA0..=0xFEFF => {
                println!("RESTRICTED AREA. NOT WRITABLE");
            }

            _ => {
                self.memory[addr as usize] = data;
            }
        }
    }
}
