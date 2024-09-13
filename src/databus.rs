pub struct DataBus{

    pub memory: [u8; 65536],
    pub cartridge_rom: Vec<u8>,
    pub cartridge_ram: Vec<u8>,

    pub ROM_banks_count: u32,
    current_ROM_bank: u8,

    pub joypad_state: u8,

    pub RAM_banks_count: u8,
    current_RAM_bank: u8,
    RAM_banks_enabled: bool,

    banking_mode: u8,

    pub MBC1: bool,
    pub MBC2: bool,
}

impl DataBus {
    pub fn new() -> DataBus {
        let mut memory = [0; 65536];
        let cartridge_rom = Vec::new();
        let cartridge_ram = Vec::new();

        //hardware registers initial values
        memory[0xFF00] = 0xCF;
        memory[0xFF02] = 0x7E;
        memory[0xFF06] = 0xAB;
        memory[0xFF07] = 0xFF;
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
            cartridge_rom,
            cartridge_ram,

            ROM_banks_count: 2,
            current_ROM_bank: 1,

            joypad_state: 0xFF,
            
            RAM_banks_count: 0,
            current_RAM_bank: 0,
            RAM_banks_enabled: false,

            banking_mode: 1,

            MBC1: false,
            MBC2: false,
        }
    }

    pub fn load_boot_rom(&mut self, index: usize, data: u8) {
        self.memory[index] = data;
    }

    pub fn load_rom(&mut self, index: usize, data: u8) {
        if index >= 0x8100 {
            self.cartridge_rom[index - 0x8000] = data;
            return;
        }
        self.memory[index] = data;
    }

    pub fn increment_div_timer(&mut self) {
        self.memory[0xFF04] = self.memory[0xFF04].wrapping_add(1);
    }

    pub fn read_memory(&self, addr: u16) -> u8{
        match addr {
            0x4000..=0x7FFF => {
                //rom banks
                if self.current_ROM_bank == 1 {
                    return self.memory[addr as usize]
                }
                let new_address = (addr - 0x4000) + ((self.current_ROM_bank - 2) as u16 * 0x4000);
                    self.cartridge_rom[new_address as usize]
                }

            0xA000..=0xBFFF => {
                //ram banks
                if self.RAM_banks_enabled {
                    let new_address = (addr - 0xA000) + (self.current_RAM_bank as u16 * 0x2000);
                    self.cartridge_ram[new_address as usize]
                }
                else {
                    0xFF
                }
            }

            0xFF00 => {
                if ((self.memory[addr as usize] >> 4) & 0x03) == 3 {
                    0x0F
                }
                else {
                    let mut res = self.memory[addr as usize];
                    if ((self.memory[addr as usize] >> 4) & 0x01) == 0 {
                        let dpad_joypad = self.joypad_state & 0xF;
                        res &= 0x30;
                        res |= dpad_joypad;
                    }
                    else if ((self.memory[addr as usize] >> 5) & 0x01) == 0 {
                        let buttons_joypad = self.joypad_state >> 4;
                        res &= 0x30;
                        res |= buttons_joypad;
                    }

                    res
                }
            }

            _ => {
                self.memory[addr as usize]
            }
        }
    }

    pub fn write_memory(&mut self, data: u8, addr: u16) {
        match addr {
            0x0000..=0x1FFF => {
                //eanbling or disabling ram banking
                if (self.MBC1 || self.MBC2) && self.RAM_banks_count > 0 {
                    if (self.MBC2) && ((addr & 0x100) > 0) {
                        return;
                    }
                    self.RAM_banks_enabled = data & 0x0F == 0x0A;
                }
            }

            0x2000..=0x3FFF => {
                //switch rom bank
                if self.MBC1 {
                    let lower5 = data & 0x1F;
                    self.current_ROM_bank &= 0xE0;
                    self.current_ROM_bank |= lower5;

                    if self.current_ROM_bank == 0x00 {
                        self.current_ROM_bank = 0x01;
                    }
                }
                if self.MBC2 {
                    self.current_ROM_bank = data & 0x0F;
                    if self.current_ROM_bank == 0 {
                        self.current_ROM_bank = 1;
                    }
                }
            }

            0x4000..= 0x5FFF => {
                //ram bank number
                if self.MBC1 {
                    if self.banking_mode == 0 && self.RAM_banks_count > 0 {
                        self.current_RAM_bank = data & 0x3;
                    }
                    if self.banking_mode == 1 && self.ROM_banks_count > 32 {
                        self.current_ROM_bank &= 0x1F;
                        self.current_ROM_bank |= data & 0xE0;
                        if self.current_ROM_bank == 0 {
                            self.current_ROM_bank = 1;
                        }
                    }
                }
            }

            0x6000..=0x7FFF => {
                //banking mode select
                if self.MBC1 || self.MBC2 {
                    self.banking_mode = data & 0x01;
                }
            }

            0xA000..=0xBFFF => {
                //writing to RAM banks
                if self.RAM_banks_enabled {
                    let new_address = (addr - 0xA000) + (self.current_RAM_bank as u16 * 0x2000);
                    self.cartridge_ram[new_address as usize] = data;
                }
            }

            0xE000..=0xFDFF => {
                self.memory[addr as usize] = data;
                self.write_memory(data, addr-0x2000);
            }

            0xFEA0..=0xFEFF => {
            }

            0xFF00 => {
                let input_mode = (data >> 4) & 0x03;
                self.memory[0xFF00] &= !(0x03 << 4) as u8;
                self.memory[0xFF00] |= input_mode << 4;
            }

            //writes to div timer reset it
            0xFF04 => {
                self.memory[addr as usize] = 0x00;
            }

            0xFF40 => {
                self.memory[addr as usize] = data;
            }

            //DMA transfer
            0xFF46 => {
                let address = (data as u16) << 8;
                for i in 0..0xA0 {
                    self.write_memory(self.read_memory(address + i), 0xFE00 + i);
                }
            }

            _ => {
                self.memory[addr as usize] = data;
            }
        }
    }
}
