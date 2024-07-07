pub enum Mnemonic {
    ADC,
    ADD,
    AND,
    BIT,
    CALL,
    CCF,
    CP,
    CPL,
    DAA,
    DEC,
    DI,
    EI,
    HALT,
    INC,
    JP,
    JR,
    LD,
    LDH,
    NOP,
    OR,
    POP,
    PUSH,
    RES,
    RET,
    RETI,
    RL,
    RLA,
    RLC,
    RLCA,
    RR,
    RRA,
    RRC,
    RRCA,
    RST,
    SBC,
    SCF,
    SET,
    SLA,
    SRA,
    SRL,
    STOP,
    SUB,
    SWAP,
    XOR,
}

pub enum AddressingMode {
    Buh,
}

pub struct Instruction {
    opcode: u8,
    mnemonic: Mnemonic,
    length: u8,
    cycles: u8,
}

impl Instruction {
   pub fn new(opcode: u8, mnemonic: Mnemonic, length: u8, cycles: u8) -> Instruction {
        Instruction{
            opcode,
            mnemonic,
            length,
            cycles,
        }
    }
}

pub struct Cpu {
    cycles: u32,

    //registers
    a: u8,
    bc: u16,
    de: u16,
    hl: u16,

    //flags setter byte
    f: u8,

    //flags
    z: bool,
    n: bool,
    h: bool,
    c: bool,

    // stack pointer and program counter
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            cycles: 0,

            a: 0x01,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,

            f: 0,

            z: true,
            n: false,
            h: false,
            c: false,

            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
    pub fn exec_instruction() {

    }
}
