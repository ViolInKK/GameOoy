use std::{cell::RefCell, rc::Rc};
use crate::cpu_instructions::{cycles_length, Mnemonic, Operand, INSTRUCTIONS_MAP, PREFIXED_INSTRUCTIONS_MAP};

use crate::databus::DataBus;

pub struct Cpu {
    //general registers
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,

    //flags register
    //binary: znhc_0000
    F: u8,

    //interrupt flag 
    IME: bool,

    is_halted: bool,

    // stack pointer and program counter
    pub sp: u16,
    pub pc: u16,

    databus: Rc<RefCell<DataBus>>,
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
            A: {:#04x}
            B: {:#04x}
            C: {:#04x}
            D: {:#04x}
            E: {:#04x}
            H: {:#04x}
            L: {:#04x}

            F: {:#010b}

            sp: {:#06x}
            pc: {:#06x}
            ", self.A, self.B, self.C, self.D, self.E, self.H, self.L, self.F, self.sp, self.pc)
    }
}

impl Cpu {
    pub fn new(databus: Rc<RefCell<DataBus>>) -> Cpu {
        Cpu {
            A: 0x01,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            H: 0x01,
            L: 0x4D,

            F: 0b1011_0000,

            IME: false,

            is_halted: false,

            sp: 0xFFFE,
            pc: 0x0100,

            databus,
        }
    }

    fn flip_z(&mut self){
        self.F ^= 0b1000_0000;
    }

    fn flip_n(&mut self){
        self.F ^= 0b0100_0000;
    }

    fn flip_h(&mut self){
        self.F ^= 0b0010_0000;
    }

    fn flip_c(&mut self){
        self.F ^= 0b0001_0000;
    }

    fn set_z_to(&mut self, value: bool) {
        //clean right most bit; a.k.a. z flag.
        self.F &= !(1 << 7) as u8;
        //set right most bit to passed value.
        self.F |= (value as u8) << 7;
    }

    fn set_n_to(&mut self, value: bool) {
        self.F &= !(1 << 6) as u8;
        self.F |= (value as u8) << 6;
    }

    fn set_h_to(&mut self, value: bool) {
        self.F &= !(1 << 5) as u8;
        self.F |= (value as u8) << 5;
    }

    fn set_c_to(&mut self, value: bool) {
        self.F &= !(1 << 4) as u8;
        self.F |= (value as u8) << 4;
    }

    fn get_z(&self) -> bool{
        if (self.F & 0b1000_0000) > 0 {
            return true
        }
        false
    }

    fn get_n(&self) -> bool{
        if (self.F & 0b0100_0000) > 0 {
            return true
        }
        false
    }
    fn get_h(&self) -> bool{
        if (self.F & 0b0010_0000) > 0 {
            return true
        }
        false
    }
    fn get_c(&self) -> bool{
        if (self.F & 0b0001_0000) > 0 {
            return true
        }
        false
    }

    fn get_HL(&self) -> u16 {
        let mut HL: u16 = 0x0000;
        HL |= self.H as u16;
        HL <<= 8;
        HL |= self.L as u16;
        HL
    }

    fn get_DE(&self) -> u16 {
        let mut DE: u16 = 0x0000;
        DE |= self.D as u16;
        DE <<= 8;
        DE |= self.E as u16;
        DE
    }

    fn get_BC(&self) -> u16 {
        let mut BC: u16 = 0x0000;
        BC |= self.B as u16;
        BC <<= 8;
        BC |= self.C as u16;
        BC
    }

    fn get_AF(&self) -> u16 {
        let mut AF: u16 = 0x0000;
        AF |= self.A as u16;
        AF <<= 8;
        AF |= self.F as u16;
        AF
    }

    fn get_a16(&self) -> u16 {
        let mut a16: u16 = 0x0000;
        let databus_borrow = self.databus.borrow();
        a16 |= databus_borrow.read_memory(self.pc + 2) as u16;
        a16  <<= 8;
        a16 |= databus_borrow.read_memory(self.pc + 1) as u16;
        a16
    }

    fn get_n16(&self) -> u16 {
        let mut n16: u16 = 0x0000;
        n16 |= self.databus.borrow().read_memory(self.pc + 2) as u16;
        n16 <<= 8;
        n16 |= self.databus.borrow().read_memory(self.pc + 1) as u16;
        n16
    }

    fn get_n16_bytes(&self) -> [u8; 2] {
        [
            self.databus.borrow().read_memory(self.pc + 2),
            self.databus.borrow().read_memory(self.pc + 1),
        ]
    }

    fn get_n8(&self) -> u8 {
        self.databus.borrow().read_memory(self.pc + 1)
    }

    fn get_e8(&self) -> i8 {
        self.databus.borrow().read_memory(self.pc + 1) as i8
    }

    fn ADC(&mut self, addition_byte: u8) {
        let result = self.A.overflowing_add(addition_byte + self.get_c() as u8);

        if result.0 == 0 {
            self.set_z_to(true);
        }
        if result.1 {
            self.set_c_to(true);
        }
        if ((self.A & 0x0F) + (addition_byte & 0x0F)) > 0x0F {
            self.set_h_to(true);
        }
        self.set_n_to(false);

        self.A = result.0;
    }

    fn ADD(&mut self, rhs: u8) {
        let result = self.A.overflowing_add(rhs);

        if result.0 == 0 {
            self.set_z_to(true);
        }
        if result.1 {
            self.set_c_to(true);
        }
        if ((self.A & 0x0F) + (rhs & 0x0F)) > 0x0F {
            self.set_h_to(true);
        }
        self.set_n_to(false);

        self.A = result.0;
    }

    fn ADD_16bit(&mut self, rhs: u16) {
        let HL = self.get_HL();
        let result = HL.overflowing_add(rhs);

        if result.1 {
            self.set_c_to(true);
        }
        if ((HL & 0x0FFF) + (rhs & 0x0FFF)) > 0x0FFF {
            self.set_h_to(true);
        }
        self.set_n_to(false);

        self.H = (result.0 >> 8) as u8;
        self.L = (result.0 & 0xFF) as u8;
    }

    fn AND(&mut self, rhs: u8) {
        let result = self.A & rhs;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(true);
        self.set_c_to(false);

        self.A = result;
    }

    fn CALL(&mut self, a16: u16) {
        let mut databus_borrow = self.databus.borrow_mut();

        //Push next instruction address onto stack
        self.sp -= 1;
        databus_borrow.write_memory(((self.pc + 3) >> 8) as u8, self.sp);
        self.sp -= 1;
        databus_borrow.write_memory(((self.pc + 3) & 0xFF) as u8, self.sp);

        //jump to address a16
        self.pc = a16;
    }

    fn CP(&mut self, rhs: u8) {
        if self.A == rhs {
            self.set_z_to(true);
        }
        if self.A < rhs {
            self.set_c_to(true);
        }
        //this may be fucked
        if (self.A & 0x0F) < (rhs & 0x0F) {
            self.set_h_to(true);
        }
        self.set_n_to(true);
    }

    fn DAA(&mut self) {
        // https://blog.ollien.com/posts/gb-daa/
        let mut offset: u8 = 0;
        let is_carry = self.get_c();
        let is_half_carry = self.get_h();
        let is_subtract = self.get_n();

        if is_half_carry || (!is_subtract && (self.A & 0x0F) > 0x09) {
            offset |= 0x06;
        }
        if is_carry || (!is_subtract && self.A > 0x99) {
            offset |= 0x60;
            self.set_c_to(true);
        }

        if is_subtract {
            self.A = self.A.wrapping_sub(offset);
        }
        else {
            self.A = self.A.wrapping_add(offset);
        }

        if self.A == 0 {
            self.set_z_to(true);
        }
        else {
            self.set_z_to(false);
        }
        self.set_h_to(false);
    }

    fn DEC(&mut self, lhs: u8) -> u8 {
        let result = lhs.wrapping_sub(1);

        if result == 0 {
            self.set_z_to(true);
        }
        //this may be fucked
        //maybe (lhs > 0) && (lhs & 0x0F == 0)
        if lhs & 0x0F == 0 {
            self.set_h_to(true);
        }
        self.set_n_to(true);

        result
    }

    fn DEC_16bit(&mut self, lhs: u16) -> u16 {
        lhs.wrapping_sub(1)
    }

    fn INC(&mut self, lhs: u8) -> u8 {
        let result = lhs.wrapping_add(1);

        if result == 0 {
            self.set_z_to(true);
        }
        //this also may be as fucked as ^
        if lhs & 0x0F == 0x0F {
            self.set_h_to(true);
        }
        self.set_n_to(false);

        result
    }

    fn INC_16bit(&mut self, lhs: u16) -> u16 {
        lhs.wrapping_add(1)
    }

    fn OR(&mut self, rhs: u8) -> u8 {
        let result = self.A | rhs;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(false);

        result
    }

    fn POP(&mut self) -> (u8, u8) {
        let databus_borrow = self.databus.borrow();
        let lsb: u8 = databus_borrow.read_memory(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let msb: u8 = databus_borrow.read_memory(self.sp);
        self.sp = self.sp.wrapping_add(1);

        (msb, lsb)
    }

    fn PUSH(&mut self, rhs: u16) {
        let mut databus_borrow = self.databus.borrow_mut();
        self.sp = self.sp.wrapping_sub(1);
        databus_borrow.write_memory((rhs >> 8) as u8, self.sp);
        self.sp = self.sp.wrapping_sub(1);
        databus_borrow.write_memory((rhs & 0xFF) as u8, self.sp);
    }

    fn SBC(&mut self, rhs: u8) -> u8 {
        let result = self.A.overflowing_sub(rhs + self.get_c() as u8);

        if result.0 == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(true);
        if self.A & 0x0F < (rhs + self.get_c() as u8) & 0x0F {
            self.set_h_to(true);
        }
        if result.1 {
            self.set_c_to(true);
        }

        result.0
    }

    fn SUB(&mut self, rhs: u8) -> u8 {
        let result = self.A.overflowing_sub(rhs);

        if result.0 == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(true);
        if self.A & 0x0F < rhs & 0x0F {
            self.set_h_to(true);
        }
        if result.1 {
            self.set_c_to(true);
        }

        result.0
    }

    fn XOR(&mut self, rhs: u8) -> u8 {
        let result = self.A ^ rhs;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(false);

        result
    }

    fn BIT(&mut self, affected_bit: u8, rhs: u8) {
        let result = (rhs >> affected_bit) & 1;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(true);
    }

    fn RES(&mut self, affected_bit: u8, rhs: u8) -> u8 {
        rhs & !(1 << affected_bit)
    }

    fn SET(&mut self, affected_bit: u8, rhs: u8) -> u8 {
        rhs | (1 << affected_bit)
    }

    fn RL(&mut self, rhs: u8) -> u8{
        let msb =  (rhs & 0x80) >> 7;
        let c = self.get_c() as u8;
        let result = (rhs << 1) | c;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(msb != 0);

        result
    }

    fn RLC(&mut self, rhs: u8) -> u8 {
        let msb = (rhs & 0x80) >> 7;
        let result = rhs.rotate_left(1);

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(msb != 0);

        result
    }

    fn RR(&mut self, rhs: u8) -> u8 {
        let lsb =  rhs & 0x1;
        let c = (self.get_c() as u8) << 7;
        let result = (rhs >> 1) | c;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(lsb != 0);

        result
    }

    fn RRC(&mut self, rhs: u8) -> u8 {
        let lsb =  rhs & 0x1;
        let result = rhs.rotate_right(1);

        self.set_z_to(false);
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(lsb != 0);

        result
    }

    fn SLA(&mut self, rhs: u8) -> u8 {
        let msb = (rhs & 0x80) >> 7;
        let result = rhs << 1;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(msb != 0);
        
        result
    }

    fn SRA(&mut self, rhs: u8) -> u8 {
        let lsb =  rhs & 0x1;
        let result = rhs >> 1;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(lsb != 0);

        result
    }

    fn SWAP(&mut self, rhs: u8) -> u8 {
        let msn = (rhs & 0xF0) >> 4;
        let lsn = rhs & 0x0F;
        let result = (lsn << 4) | msn;

        if result == 0 {
            self.set_z_to(true);
        }
        self.set_n_to(false);
        self.set_h_to(false);
        self.set_c_to(false);

        result
    }

    pub fn exec_instruction(&mut self, instruction_byte: u8) -> u8 {
        let instruction = INSTRUCTIONS_MAP.get(&instruction_byte).unwrap();
        let mut jumped: bool = false;
        let mut condition_met: bool = false;
        let mut total_cycles: u8 = 0;

        if crate::DEBUG {
            println!("
            CPU STATE BEFORE:
                {}", self);
            println!("
            INSTRUCTION:
                {}", instruction);
        }

        match instruction.mnemonic {
            Mnemonic::ADC => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.ADC(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.ADC(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.ADC(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.ADC(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.ADC(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.ADC(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let at_memory_HL = self.databus.borrow().read_memory(self.get_HL());
                        self.ADC(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.ADC(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.ADC(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing ADC instruction.");
                    }
                }
            }

            Mnemonic::ADD => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::HL, Operand::BC] => {
                        self.ADD_16bit(self.get_BC());
                    }

                    [Operand::HL, Operand::DE] => {
                        self.ADD_16bit(self.get_DE());
                    }

                    [Operand::HL, Operand::HL] => {
                        self.ADD_16bit(self.get_HL());
                    }

                    [Operand::HL, Operand::SP] => {
                        self.ADD_16bit(self.sp);
                    }

                    [Operand::A, Operand::B] => {
                        self.ADD(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.ADD(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.ADD(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.ADD(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.ADD(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.ADD(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let at_memory_HL = self.databus.borrow().read_memory(self.get_HL());
                        self.ADD(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.ADD(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.ADD(self.get_n8());
                    }

                    [Operand::SP, Operand::e8] => {
                        let e8 = self.get_e8();

                        self.set_z_to(false);
                        self.set_n_to(false);
                        self.sp = self.sp.wrapping_add_signed(e8.into());
                    }

                    _ => {
                        eprintln!("Non existing ADD instruction.");
                    }

                }
            }

            Mnemonic::AND => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.AND(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.AND(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.AND(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.AND(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.AND(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.AND(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let at_memory_HL = self.databus.borrow().read_memory(self.get_HL());
                        self.AND(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.AND(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.AND(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing AND instruction.");
                    }
                }
            }

            Mnemonic::CALL => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::NZ, Operand::a16] => {
                        if !self.get_z() {
                            let a16 = self.get_a16();
                            self.CALL(a16);
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::Z, Operand::a16] => {
                        if self.get_z() {
                            let a16 = self.get_a16();
                            self.CALL(a16);
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::a16, Operand::none] => {
                        let a16 = self.get_a16();
                        self.CALL(a16);
                        jumped = true;
                    }

                    [Operand::NCY, Operand::a16] => {
                        if !self.get_c() {
                            let a16 = self.get_a16();
                            self.CALL(a16);
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::C, Operand::a16] => {
                        if self.get_c() {
                            let a16 = self.get_a16();
                            self.CALL(a16);
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    _ => {
                        eprintln!("Non existing CALL instruction.");
                    }
                }
            }

            Mnemonic::CCF => {
                self.flip_c();
            }

            Mnemonic::CP => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.CP(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.CP(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.CP(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.CP(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.CP(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.CP(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let at_memory_HL = self.databus.borrow().read_memory(self.get_HL());
                        self.CP(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.set_z_to(true);
                        self.set_c_to(false);
                        self.set_h_to(false);
                        self.set_n_to(true);
                    }

                    [Operand::A, Operand::n8] => {
                        self.CP(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing CP instruction.");
                    }
                }
            }

            Mnemonic::CPL => {
                self.A = !self.A;
            }
            Mnemonic::DAA => {
                self.DAA();
            }

            Mnemonic::DEC => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.DEC(self.B);
                    }

                    [Operand::BC, Operand::none] => {
                        let mut BC = self.get_BC();
                        BC = self.DEC_16bit(BC);
                        self.B = (BC >> 8) as u8;
                        self.C = (BC & 0xFF) as u8;
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.DEC(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.DEC(self.D);
                    }

                    [Operand::DE, Operand::none] => {
                        let mut DE = self.get_DE();
                        DE = self.DEC_16bit(DE);
                        self.D = (DE >> 8) as u8;
                        self.E = (DE & 0xFF) as u8;
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.DEC(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.DEC(self.H);
                    }

                    [Operand::HL, Operand::none] => {
                        let mut HL = self.get_HL();
                        HL = self.DEC_16bit(HL);
                        self.H = (HL >> 8) as u8;
                        self.L = (HL & 0xFF) as u8;
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.DEC(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.DEC(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::SP, Operand::none] => {
                        self.sp = self.DEC_16bit(self.sp);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.DEC(self.A);
                    }

                    _ => {
                        eprintln!("Non existing DEC instruction.");
                    }
                }
            }

            Mnemonic::DI => {
                //This is some fucky shit
                let next_instruction = self.databus.borrow().read_memory(self.pc.wrapping_add(1));
                self.exec_instruction(next_instruction);
                self.IME = false;
            }

            Mnemonic::EI => {
                //This is some fucky shit
                let next_instruction = self.databus.borrow().read_memory(self.pc.wrapping_add(1));
                self.exec_instruction(next_instruction);
                self.IME = true;
            }

            Mnemonic::HALT => {
                if self.IME {
                    self.is_halted = true;
                }
                else {
                    let IE = self.databus.borrow().read_memory(0xFFFF);
                    let IF = self.databus.borrow().read_memory(0xFF0F);

                    if (IE & IF) == 0 {

                    }

                }
            }

            Mnemonic::INC => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::BC, Operand::none] => {
                        let mut BC = self.get_BC();
                        BC = self.INC_16bit(BC);
                        self.B = (BC >> 8) as u8;
                        self.C = (BC & 0xFF) as u8;
                    }

                    [Operand::B, Operand::none] => {
                        self.B = self.INC(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.INC(self.C);
                    }

                    [Operand::DE, Operand::none] => {
                        let mut DE = self.get_DE();
                        DE = self.INC_16bit(DE);
                        self.D = (DE >> 8) as u8;
                        self.E = (DE & 0xFF) as u8;
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.INC(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.INC(self.E);
                    }

                    [Operand::HL, Operand::none] => {
                        let mut HL = self.get_HL();
                        HL = self.INC_16bit(HL);
                        self.H = (HL >> 8) as u8;
                        self.L = (HL & 0xFF) as u8;
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.INC(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.INC(self.L);
                    }

                    [Operand::SP, Operand::none] => {
                        self.sp = self.INC_16bit(self.sp);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.INC(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.INC(self.A);
                    }

                    _ => {
                        eprintln!("Non existing INC instruction.");
                    }
                }
            }

            Mnemonic::JP => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::NZ, Operand::a16] => {
                        if !self.get_z() {
                            let a16 = self.get_a16();
                            self.pc = a16;
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::a16, Operand::none] => {
                        let a16 = self.get_a16();
                        self.pc = a16;
                        jumped = true;
                    }

                    [Operand::Z, Operand::a16] => {
                        if self.get_z() {
                            let a16 = self.get_a16();
                            self.pc = a16;
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::NCY, Operand::a16] => {
                        if !self.get_c() {
                            let a16 = self.get_a16();
                            self.pc = a16;
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::C, Operand::a16] => {
                        if self.get_c() {
                            let a16 = self.get_a16();
                            self.pc = a16;
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::HL, Operand::none] => {
                        self.pc = self.get_HL();
                        jumped = true;
                    }

                    _ => {
                        eprintln!("Non existing JP instruction.");
                    }

                }
            }

            Mnemonic::JR => {
                let e8 = self.get_e8();
                match instruction.operands.as_ref().unwrap() {

                    [Operand::e8, Operand::none] => {
                        self.pc = self.pc.wrapping_add_signed(e8.into());
                        jumped = true;
                    }

                    [Operand::NZ, Operand::e8] => {
                        if !self.get_z() {
                            self.pc = self.pc.wrapping_add_signed(e8.into());
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::Z, Operand::e8] => {
                        if self.get_z() {
                            self.pc = self.pc.wrapping_add_signed(e8.into());
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::NCY, Operand::e8] => {
                        if !self.get_c() {
                            self.pc = self.pc.wrapping_add_signed(e8.into());
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    [Operand::C, Operand::e8] => {
                        if self.get_c() {
                            self.pc = self.pc.wrapping_add_signed(e8.into());
                            jumped = true;
                            condition_met = true;
                        }
                    }

                    _ => {
                        eprintln!("Non existing JR instruction.");
                    }
                }
            }

            Mnemonic::LD => {
                match instruction.operands.as_ref().unwrap() {
                    [Operand::BC, Operand::n16] => {
                        let n16_bytes = self.get_n16_bytes();
                        self.B = n16_bytes[0];
                        self.C = n16_bytes[1];
                    }

                    [Operand::at_memory_BC, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, self.get_BC());
                    }

                    [Operand::B, Operand::n8] => {
                        self.B = self.get_n8();
                    }

                    [Operand::at_memory_a16, Operand::SP] => {
                        let a16 = self.get_a16();
                        let mut databus_borrow = self.databus.borrow_mut();
                        databus_borrow.write_memory((self.sp & 0x00FF) as u8, a16);
                        databus_borrow.write_memory((self.sp >> 8) as u8, a16 + 1);
                    }

                    [Operand::A, Operand::at_memory_BC] => {
                        self.A = self.databus.borrow().read_memory(self.get_BC());
                    }

                    [Operand::C, Operand::n8] => {
                        self.C = self.get_n8();
                    }

                    [Operand::DE, Operand::n16] => {
                        let n16_bytes = self.get_n16_bytes();
                        self.D = n16_bytes[0];
                        self.E = n16_bytes[1];
                    }

                    [Operand::at_memory_DE, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, self.get_DE());
                    }

                    [Operand::D, Operand::n8] => {
                        self.D = self.get_n8();
                    }

                    [Operand::A, Operand::at_memory_DE] => {
                        self.A = self.databus.borrow().read_memory(self.get_DE());
                    }

                    [Operand::E, Operand::n8] => {
                        self.E = self.get_n8();
                    }

                    [Operand::HL, Operand::n16] => {
                        let n16_bytes = self.get_n16_bytes();
                        self.H = n16_bytes[0];
                        self.L = n16_bytes[1];
                    }

                    [Operand::at_memory_HLI, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, self.get_HL());
                    }

                    [Operand::H, Operand::n8] => {
                        self.H = self.get_n8();
                    }

                    [Operand::A, Operand::at_memory_HLI] => {
                        self.A = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::L, Operand::n8] => {
                        self.L = self.get_n8();
                    }

                    [Operand::SP, Operand::n16] => {
                        self.sp = self.get_n16();
                    }

                    [Operand::at_memory_HLD, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::n8] => {
                        let n8 = self.get_n8();
                        let HL = self.get_HL();
                        self.databus.borrow_mut().write_memory(n8, HL);
                    }

                    [Operand::A, Operand::at_memory_HLD] => {
                        self.A = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::A, Operand::n8] => {
                        self.A = self.get_n8();
                    }

                    [Operand::B, Operand::B] => {
                    }

                    [Operand::B, Operand::C] => {
                        self.B = self.C;
                    }

                    [Operand::B, Operand::D] => {
                        self.B = self.D;
                    }

                    [Operand::B, Operand::E] => {
                        self.B = self.E;
                    }

                    [Operand::B, Operand::H] => {
                        self.B = self.H;
                    }

                    [Operand::B, Operand::L] => {
                        self.B = self.L;
                    }

                    [Operand::B, Operand::at_memory_HL] => {
                        self.B = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::B, Operand::A] => {
                        self.B = self.A;
                    }

                    [Operand::C, Operand::B] => {
                        self.C = self.B;
                    }

                    [Operand::C, Operand::C] => {
                    }

                    [Operand::C, Operand::D] => {
                        self.C = self.D;
                    }

                    [Operand::C, Operand::E] => {
                        self.C = self.E;
                    }

                    [Operand::C, Operand::H] => {
                        self.C = self.H;
                    }

                    [Operand::C, Operand::L] => {
                        self.C = self.L;
                    }

                    [Operand::C, Operand::at_memory_HL] => {
                        self.C = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::C, Operand::A] => {
                        self.C = self.A;
                    }

                    [Operand::D, Operand::B] => {
                        self.D = self.B;
                    }

                    [Operand::D, Operand::C] => {
                        self.D = self.C;
                    }

                    [Operand::D, Operand::D] => {
                    }

                    [Operand::D, Operand::E] => {
                        self.D = self.E;
                    }

                    [Operand::D, Operand::H] => {
                        self.D = self.H;
                    }

                    [Operand::D, Operand::L] => {
                        self.D = self.L;
                    }

                    [Operand::D, Operand::at_memory_HL] => {
                        self.D = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::D, Operand::A] => {
                        self.D = self.A;
                    }

                    [Operand::E, Operand::B] => {
                        self.E = self.B;
                    }

                    [Operand::E, Operand::C] => {
                        self.E = self.C;
                    }

                    [Operand::E, Operand::D] => {
                        self.E = self.D;
                    }

                    [Operand::E, Operand::E] => {
                    }

                    [Operand::E, Operand::H] => {
                        self.E = self.H;
                    }

                    [Operand::E, Operand::L] => {
                        self.E = self.L;
                    }

                    [Operand::E, Operand::at_memory_HL] => {
                        self.E = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::E, Operand::A] => {
                        self.E = self.A;
                    }

                    [Operand::H, Operand::B] => {
                        self.H = self.B;
                    }

                    [Operand::H, Operand::C] => {
                        self.H = self.C;
                    }

                    [Operand::H, Operand::D] => {
                        self.H = self.D;
                    }

                    [Operand::H, Operand::E] => {
                        self.H = self.E;
                    }

                    [Operand::H, Operand::H] => {
                    }

                    [Operand::H, Operand::L] => {
                        self.H = self.L;
                    }

                    [Operand::H, Operand::at_memory_HL] => {
                        self.H = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::H, Operand::A] => {
                        self.H = self.A;
                    }

                    [Operand::L, Operand::B] => {
                        self.L = self.B;
                    }

                    [Operand::L, Operand::C] => {
                        self.L = self.C;
                    }

                    [Operand::L, Operand::D] => {
                        self.L = self.D;
                    }

                    [Operand::L, Operand::E] => {
                        self.L = self.E;
                    }

                    [Operand::L, Operand::H] => {
                        self.L = self.H;
                    }

                    [Operand::L, Operand::L] => {
                    }

                    [Operand::L, Operand::at_memory_HL] => {
                        self.L = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::L, Operand::A] => {
                        self.L = self.A;
                    }

                    [Operand::at_memory_HL, Operand::B] => {
                        self.databus.borrow_mut().write_memory(self.B, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::C] => {
                        self.databus.borrow_mut().write_memory(self.C, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::D] => {
                        self.databus.borrow_mut().write_memory(self.D, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::E] => {
                        self.databus.borrow_mut().write_memory(self.E, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::H] => {
                        self.databus.borrow_mut().write_memory(self.H, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::L] => {
                        self.databus.borrow_mut().write_memory(self.L, self.get_HL());
                    }

                    [Operand::at_memory_HL, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, self.get_HL());
                    }

                    [Operand::A, Operand::B] => {
                        self.A = self.B;
                    }

                    [Operand::A, Operand::C] => {
                        self.A = self.C;
                    }

                    [Operand::A, Operand::D] => {
                        self.A = self.D;
                    }

                    [Operand::A, Operand::E] => {
                        self.A = self.E;
                    }

                    [Operand::A, Operand::H] => {
                        self.A = self.H;
                    }

                    [Operand::A, Operand::L] => {
                        self.A = self.L;
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        self.A = self.databus.borrow().read_memory(self.get_HL());
                    }

                    [Operand::A, Operand::A] => {
                    }

                    [Operand::at_memory_C, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, self.C as u16);
                    }

                    [Operand::at_memory_a16, Operand::A] => {
                        let a16 = self.get_a16();
                        self.databus.borrow_mut().write_memory(self.A,a16);
                    }

                    [Operand::A, Operand::at_memory_C] => {
                        self.A = self.databus.borrow().read_memory(self.C as u16);
                    }

                    [Operand::HL, Operand::SP_plus_e8] => {
                        //This is some fucky stuff
                        let e8 = self.get_e8();
                        let SP_plus_e8 = self.sp.overflowing_add_signed(e8.into());

                        if SP_plus_e8.1 {
                            self.set_c_to(true);
                        }
                        if e8 > 0 && ((e8 & 0x0F) as u8 + (self.sp & 0x0F) as u8) >= 0x10 {
                            self.set_h_to(true);
                        }
                        //half borrow. this may be uneeded
                        if e8 < 0 && ((e8.unsigned_abs() & 0x0F) > (self.sp & 0x0F) as u8){
                            self.set_h_to(true);
                        }
                        self.set_z_to(false);
                        self.set_h_to(false);

                        self.H = (SP_plus_e8.0 >> 8) as u8;
                        self.L = (SP_plus_e8.0 & 0xFF) as u8;
                    }

                    [Operand::SP, Operand::HL] => {
                        self.sp = self.get_HL();
                    }

                    [Operand::A, Operand::at_memory_a16] => {
                        self.A = self.databus.borrow().read_memory(self.get_a16());
                    }

                    _ => {
                        eprintln!("Non existing LD instruction.");
                    }
                }
            }

            Mnemonic::LDH => {
                let a8 = self.get_n8();
                match instruction.operands.as_ref().unwrap() {

                    [Operand::at_memory_a8, Operand::A] => {
                        self.databus.borrow_mut().write_memory(self.A, 0xFF00 + a8 as u16);
                    }

                    [Operand::A, Operand::at_memory_a8] => {
                        self.A = self.databus.borrow().read_memory(0xFF00 + a8 as u16);
                    }

                    _ => {
                        eprintln!("Non existing LDH instruction.");
                    }

                }

            }

            Mnemonic::NOP => {
            }

            Mnemonic::OR => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.A = self.OR(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.A = self.OR(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.A = self.OR(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.A = self.OR(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.A = self.OR(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.A = self.OR(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let at_memory_HL = self.databus.borrow().read_memory(self.get_HL());
                        self.A = self.OR(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.A = self.OR(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.A = self.OR(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing OR instruction.");
                    }
                }
            }

            Mnemonic::POP => {
                let (msb, lsb) = self.POP();
                match instruction.operands.as_ref().unwrap() {

                    [Operand::BC, Operand::none] => {
                        self.B = msb;
                        self.C = lsb;
                    }

                    [Operand::DE, Operand::none] => {
                        self.D = msb;
                        self.E = lsb;
                    }

                    [Operand::HL, Operand::none] => {
                        self.H = msb;
                        self.L = lsb;
                    }

                    [Operand::AF, Operand::none] => {
                        self.A = msb;
                        self.F = lsb;
                    }

                    _ => {
                        eprintln!("Non existing POP instruction.");
                    }
                }
            }

            Mnemonic::PREFIX => {
                let instruction_byte = self.databus.borrow().read_memory(self.pc + 1);
                total_cycles += self.exec_prefixed_instruction(instruction_byte);
            }

            Mnemonic::PUSH => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::BC, Operand::none] => {
                        let BC = self.get_BC();
                        self.PUSH(BC);
                    }

                    [Operand::DE, Operand::none] => {
                        let DE = self.get_DE();
                        self.PUSH(DE);
                    }

                    [Operand::HL, Operand::none] => {
                        let HL = self.get_HL();
                        self.PUSH(HL);
                    }

                    [Operand::AF, Operand::none] => {
                        let AF = self.get_AF();
                        self.PUSH(AF);
                    }

                    _ => {
                        eprintln!("Non existing PUSH instruction.");
                    }

                }
            }

            Mnemonic::RET => {
                let mut new_pc: u16 = 0x0000;

                if instruction.operands.is_none() {
                    let (msb, lsb) = self.POP();

                    new_pc |= msb as u16;
                    new_pc <<= 8;
                    new_pc |= lsb as u16;
                    self.pc = new_pc; 
                    jumped = true;
                }
                else {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::NZ, Operand::none] => {
                            if !self.get_z() {
                                let (msb, lsb) = self.POP();

                                new_pc |= msb as u16;
                                new_pc <<= 8;
                                new_pc |= lsb as u16;
                                self.pc = new_pc;
                                jumped = true;
                                condition_met = true;
                            }
                        }

                        [Operand::Z, Operand::none] => {
                            if self.get_z() {
                                let (msb, lsb) = self.POP();

                                new_pc |= msb as u16;
                                new_pc <<= 8;
                                new_pc |= lsb as u16;
                                self.pc = new_pc; 
                                jumped = true;
                                condition_met = true;
                            }
                        }

                        [Operand::NCY, Operand::none] => {
                            if !self.get_c() {
                                let (msb, lsb) = self.POP();

                                new_pc |= msb as u16;
                                new_pc <<= 8;
                                new_pc |= lsb as u16;
                                self.pc = new_pc; 
                                jumped = true;
                                condition_met = true;
                            }
                        }

                        [Operand::C, Operand::none] => {
                            if self.get_c() {
                                let (msb, lsb) = self.POP();

                                new_pc |= msb as u16;
                                new_pc <<= 8;
                                new_pc |= lsb as u16;
                                self.pc = new_pc; 
                                jumped = true;
                                condition_met = true;
                            }
                        }

                        _ => {
                            eprintln!("Non existing RET instruction.");
                        }
                    }
                }
            }

            Mnemonic::RETI => {
                let mut new_pc: u16 = 0x0000;
                let (msb, lsb) = self.POP();

                new_pc |= msb as u16;
                new_pc <<= 8;
                new_pc |= lsb as u16;
                self.pc = new_pc;
                self.IME = true;
                jumped = true;
            }

            //RLA and RLCA, RRA and RRCA implimintations may have to be swapped ??
            Mnemonic::RLA => {
                let msb =  (self.A & 0x80) >> 7;
                let c = self.get_c() as u8;

                self.set_z_to(false);
                self.set_n_to(false);
                self.set_h_to(false);
                self.set_c_to(msb != 0);

                self.A = (self.A << 1) | c;
            }

            Mnemonic::RLCA => {
                let msb = (self.A & 0x80) >> 7;

                self.set_z_to(false);
                self.set_n_to(false);
                self.set_h_to(false);
                self.set_c_to(msb != 0);

                self.A = self.A.rotate_left(1);
            }

            Mnemonic::RRA => {
                let lsb =  self.A & 0x1;
                let c = (self.get_c() as u8) << 7;

                self.set_z_to(false);
                self.set_n_to(false);
                self.set_h_to(false);
                self.set_c_to(lsb != 0);

                self.A = (self.A >> 1) | c;
            }

            Mnemonic::RRCA => {
                let lsb =  self.A & 0x1;

                self.set_z_to(false);
                self.set_n_to(false);
                self.set_h_to(false);
                self.set_c_to(lsb != 0);

                self.A = self.A.rotate_right(1);
            }

            Mnemonic::RST => {
                //https://retrocomputing.stackexchange.com/questions/15116/how-does-the-rst-operation-of-gameboy-sharp-lr35902-work
                let mut databus_borrow = self.databus.borrow_mut();
                self.sp -= 1;
                databus_borrow.write_memory(((self.pc) >> 8) as u8, self.sp);
                self.sp -= 1;
                databus_borrow.write_memory(((self.pc) & 0xFF) as u8, self.sp);
                match instruction.operands.as_ref().unwrap() {

                    [Operand::vec(0x00), Operand::none] => {
                        self.pc = 0x0;
                    }

                    [Operand::vec(0x08), Operand::none] => {
                        self.pc = 0x0008;
                    }

                    [Operand::vec(0x10), Operand::none] => {
                        self.pc = 0x0010;
                    }

                    [Operand::vec(0x18), Operand::none] => {
                        self.pc = 0x0018;
                    }

                    [Operand::vec(0x20), Operand::none] => {
                        self.pc = 0x0020;
                    }

                    [Operand::vec(0x28), Operand::none] => {
                        self.pc = 0x0028;
                    }

                    [Operand::vec(0x30), Operand::none] => {
                        self.pc = 0x0030;
                    }

                    [Operand::vec(0x38), Operand::none] => {
                        self.pc = 0x0038;
                    }

                    _ => {
                        eprintln!("Non existing RST instruction.");
                    }

                }
            }

            Mnemonic::SBC => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.A = self.SBC(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.A = self.SBC(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.A = self.SBC(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.A = self.SBC(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.A = self.SBC(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.A = self.SBC(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let HL = self.get_HL();
                        let at_memory_HL = self.databus.borrow().read_memory(HL);
                        self.A = self.SBC(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.A = self.SBC(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.A = self.SBC(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing SBC instruction.");
                    }
                }
            }

            Mnemonic::SCF => {
                self.set_n_to(false);
                self.set_h_to(false);
                self.set_c_to(true);
            }

            Mnemonic::STOP => {
            }

            Mnemonic::SUB => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.A = self.SUB(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.A = self.SUB(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.A = self.SUB(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.A = self.SUB(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.A = self.SUB(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.A = self.SUB(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let HL = self.get_HL();
                        let at_memory_HL = self.databus.borrow().read_memory(HL);
                        self.A = self.SUB(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.A = self.SUB(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.A = self.SUB(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing SUB instruction.");
                    }
                }
            }

            Mnemonic::XOR => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::A, Operand::B] => {
                        self.A = self.XOR(self.B);
                    }

                    [Operand::A, Operand::C] => {
                        self.A = self.XOR(self.C);
                    }

                    [Operand::A, Operand::D] => {
                        self.A = self.XOR(self.D);
                    }

                    [Operand::A, Operand::E] => {
                        self.A = self.XOR(self.E);
                    }

                    [Operand::A, Operand::H] => {
                        self.A = self.XOR(self.H);
                    }

                    [Operand::A, Operand::L] => {
                        self.A = self.XOR(self.L);
                    }

                    [Operand::A, Operand::at_memory_HL] => {
                        let HL = self.get_HL();
                        let at_memory_HL = self.databus.borrow().read_memory(HL);
                        self.A = self.XOR(at_memory_HL);
                    }

                    [Operand::A, Operand::A] => {
                        self.A = self.XOR(self.A);
                    }

                    [Operand::A, Operand::n8] => {
                        self.A = self.XOR(self.get_n8());
                    }

                    _ => {
                        eprintln!("Non existing XOR instruction.");
                    }
                }
            }

            _ => {
                eprintln!("Non existing Instruction.");
            }
        }

        if crate::DEBUG {
           println!("
           CPU STATE AFTER:
               {}", self);
       }

        if !jumped {
            self.pc += instruction.length as u16;
        }

        match instruction.cycles {
            cycles_length::non_conditional(cycles) => {
                total_cycles += cycles;
            }

            cycles_length::conditional(met_cycles, not_met_cycles) => {
                if condition_met {
                    total_cycles += met_cycles;
                }
                else {
                    total_cycles += not_met_cycles;
                }
            }
        }

        total_cycles
    }

    fn exec_prefixed_instruction(&mut self, instruction_byte: u8) -> u8 {
        let instruction = PREFIXED_INSTRUCTIONS_MAP.get(&instruction_byte).unwrap();

            println!("
            PREFIXED INSTRUCTION:
                {}", instruction);

        let affected_bit: u8 = match instruction.operands.as_ref().unwrap()[0] {
            Operand::bit_zero => {
                Some(0)
            },

            Operand::bit_one => {
                Some(1)
            },

            Operand::bit_two => {
                Some(2)
            },

            Operand::bit_three => {
                Some(3)
            }

            Operand::bit_four => {
                Some(4)
            }

            Operand::bit_five => {
                Some(5)
            }

            Operand::bit_six => {
                Some(6)
            }

            Operand::bit_seven => {
                Some(7)
            }

            _ => {
               None 
            }
        }.unwrap_or_default();

        match instruction.mnemonic {
            Mnemonic::BIT => {
                match instruction.operands.as_ref().unwrap()[1] {

                    Operand::B => {
                        self.BIT(affected_bit, self.B);
                    }

                    Operand::C => {
                        self.BIT(affected_bit, self.C);
                    }

                    Operand::D => {
                        self.BIT(affected_bit, self.D);
                    }

                    Operand::E => {
                        self.BIT(affected_bit, self.E);
                    }

                    Operand::H => {
                        self.BIT(affected_bit, self.H);
                    }

                    Operand::L => {
                        self.BIT(affected_bit, self.L);
                    }

                    Operand::at_memory_HL => {
                        let HL = self.get_HL();
                        let at_memory_HL = self.databus.borrow().read_memory(HL);
                        self.BIT(affected_bit, at_memory_HL);
                    }

                    Operand::A => {
                        self.BIT(affected_bit, self.A);
                    }

                    _ => {
                        eprintln!("Non existing BIT instruction.");
                    }
                }
            }

            Mnemonic::RES => {
                match instruction.operands.as_ref().unwrap()[1] {

                    Operand::B => {
                        self.B = self.RES(affected_bit, self.B);
                    }

                    Operand::C => {
                        self.C = self.RES(affected_bit, self.C);
                    }

                    Operand::D => {
                        self.D = self.RES(affected_bit, self.D);
                    }

                    Operand::E => {
                        self.E = self.RES(affected_bit, self.E);
                    }

                    Operand::H => {
                        self.H = self.RES(affected_bit, self.H);
                    }

                    Operand::L => {
                        self.L = self.RES(affected_bit, self.L);
                    }

                    Operand::at_memory_HL => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.RES(affected_bit, at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    Operand::A => {
                        self.A = self.RES(affected_bit, self.A);
                    }

                    _ => {
                        eprintln!("Non existing RES instruction.");
                    }
                }
            }

            Mnemonic::RL => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.RL(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.RL(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.RL(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.RL(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.RL(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.RL(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.RL(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.RL(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RL instruction.");
                    }
                }
            }

            Mnemonic::RLC => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.RLC(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.RLC(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.RLC(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.RLC(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.RLC(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.RLC(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.RLC(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.RLC(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RLC instruction.");
                    }
                }
            }

            Mnemonic::RR => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.RR(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.RR(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.RR(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.RR(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.RR(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.RR(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.RR(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.RR(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RLC instruction.");
                    }
                }
            }

            Mnemonic::RRC => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.RRC(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.RRC(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.RRC(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.RRC(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.RRC(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.RRC(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.RRC(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.RRC(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RLC instruction.");
                    }
                }
            }

            Mnemonic::SET => {
                match instruction.operands.as_ref().unwrap()[1] {

                    Operand::B => {
                        self.B = self.SET(affected_bit, self.B);
                    }

                    Operand::C => {
                        self.C = self.SET(affected_bit, self.C);
                    }

                    Operand::D => {
                        self.D = self.SET(affected_bit, self.D);
                    }

                    Operand::E => {
                        self.E = self.SET(affected_bit, self.E);
                    }

                    Operand::H => {
                        self.H = self.SET(affected_bit, self.H);
                    }

                    Operand::L => {
                        self.L = self.SET(affected_bit, self.L);
                    }

                    Operand::at_memory_HL => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.SET(affected_bit, at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    Operand::A => {
                        self.A = self.SET(affected_bit, self.A);
                    }

                    _ => {
                        eprintln!("Non existing SET instruction.");
                    }
                }
            }

            Mnemonic::SLA => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.SLA(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.SLA(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.SLA(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.SLA(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.SLA(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.SLA(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.SLA(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.SLA(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RLC instruction.");
                    }
                }
            }

            Mnemonic::SRA => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.SRA(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.SRA(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.SRA(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.SRA(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.SRA(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.SRA(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.SRA(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.SRA(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RLC instruction.");
                    }
                }
            }

            Mnemonic::SWAP => {
                match instruction.operands.as_ref().unwrap() {

                    [Operand::B, Operand::none] => {
                        self.B = self.SWAP(self.B);
                    }

                    [Operand::C, Operand::none] => {
                        self.C = self.SWAP(self.C);
                    }

                    [Operand::D, Operand::none] => {
                        self.D = self.SWAP(self.D);
                    }

                    [Operand::E, Operand::none] => {
                        self.E = self.SWAP(self.E);
                    }

                    [Operand::H, Operand::none] => {
                        self.H = self.SWAP(self.H);
                    }

                    [Operand::L, Operand::none] => {
                        self.L = self.SWAP(self.L);
                    }

                    [Operand::at_memory_HL, Operand::none] => {
                        let HL = self.get_HL();
                        let mut at_memory_HL = self.databus.borrow().read_memory(HL);
                        at_memory_HL = self.SWAP(at_memory_HL);
                        self.databus.borrow_mut().write_memory(at_memory_HL, HL);
                    }

                    [Operand::A, Operand::none] => {
                        self.A = self.SWAP(self.A);
                    }

                    _ => {
                        eprintln!("Non existing RLC instruction.");
                    }
                }
            }

            _ => {
                eprintln!("Non existing prefixed instruction");
            }
        }
        self.pc += instruction.length as u16;
        if let cycles_length::non_conditional(cycles) = instruction.cycles {
            cycles
        }
        else {
            panic!("Conditional prefixed instruction");
        }
    }
}
