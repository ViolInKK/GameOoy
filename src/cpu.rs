use std::{collections::HashMap, rc::Rc};
use lazy_static::lazy_static;

use crate::databus::DataBus;

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
    PREFIX,
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

pub enum ConditionCode {
    Z,
    NZ,
    C,
    NC,
    NCC,
}

pub enum Operand {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    HL,
    BC,
    DE,
    SP,
    SP_plus_e8,

    at_memory_C,
    at_memory_DE,
    at_memory_BC,
    at_memory_HL,
    at_memory_HLI,
    at_memory_HLD,
    at_memory_a16,
    at_memory_a8,

    n8,
    n16,
    a16,
    e8,
    u3,

    Z,
    NZ,
    CY,
    NCY,
    NCC,

    vec(u8),

    none,
}

pub struct Instruction {
    mnemonic: Mnemonic,
    pub length: u8,
    cycles: u8,
    operands: Option<[Operand; 2]>,
}

impl Instruction {
   pub fn new(mnemonic: Mnemonic, length: u8, cycles: u8, operands: Option<[Operand; 2]>) -> Instruction {
        Instruction{
            mnemonic,
            length,
            cycles,
            operands,
        }
    }
}

const PREFIX: u8 = 0xCB;
lazy_static! {
    static ref INSTRUCTIONS_MAP: HashMap<u8, Instruction> = HashMap::from([
        (0x00,Instruction::new(Mnemonic::NOP, 1, 4,  None)),
        (0x01,Instruction::new(Mnemonic::LD,  3, 12, Some([Operand::BC,            Operand::n16]))),
        (0x02,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_BC,  Operand::A]))),
        (0x03,Instruction::new(Mnemonic::INC, 1, 8,  Some([Operand::BC,            Operand::none]))),
        (0x04,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::B,             Operand::none]))),
        (0x05,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::B,             Operand::none]))),
        (0x06,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::B,             Operand::n8]))),
        (0x07,Instruction::new(Mnemonic::RLCA,1, 4,  None)),
        (0x08,Instruction::new(Mnemonic::LD,  3, 20, Some([Operand::at_memory_a16, Operand::SP]))),
        (0x09,Instruction::new(Mnemonic::ADD, 1, 8,  Some([Operand::HL,            Operand::BC]))),
        (0x0A,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::A,             Operand::at_memory_BC]))),
        (0x0B,Instruction::new(Mnemonic::DEC, 1, 8,  Some([Operand::BC,            Operand::none]))),
        (0x0C,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::C,             Operand::none]))),
        (0x0D,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::C,             Operand::none]))),
        (0x0E,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::C,             Operand::n8]))),
        (0x0F,Instruction::new(Mnemonic::RRCA,1, 4,  None)),

        (0x10,Instruction::new(Mnemonic::STOP,2, 4,  Some([Operand::n8,            Operand::none]))),
        (0x11,Instruction::new(Mnemonic::LD,  3, 12, Some([Operand::DE,            Operand::n16]))),
        (0x12,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_DE,  Operand::A]))),
        (0x13,Instruction::new(Mnemonic::INC, 1, 8,  Some([Operand::DE,            Operand::none]))),
        (0x14,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::D,             Operand::none]))),
        (0x15,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::D,             Operand::none]))),
        (0x16,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::D,             Operand::n8]))),
        (0x17,Instruction::new(Mnemonic::RLA, 1, 4,  None)),
        (0x18,Instruction::new(Mnemonic::JR,  2, 12, Some([Operand::e8,            Operand::none]))),
        (0x19,Instruction::new(Mnemonic::ADD, 1, 8,  Some([Operand::HL,            Operand::DE]))),
        (0x1A,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::A,             Operand::at_memory_DE]))),
        (0x1B,Instruction::new(Mnemonic::DEC, 1, 8,  Some([Operand::DE,            Operand::none]))),
        (0x1C,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::E,             Operand::none]))),
        (0x1D,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::E,             Operand::none]))),
        (0x1E,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::E,             Operand::n8]))),
        (0x1F,Instruction::new(Mnemonic::RRA, 1, 4,  None)),

        (0x20,Instruction::new(Mnemonic::JR,  2, 12, Some([Operand::NZ,            Operand::e8]))), //todo 12/8
        (0x21,Instruction::new(Mnemonic::LD,  3, 12, Some([Operand::HL,            Operand::n16]))),
        (0x22,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HLI, Operand::A]))),
        (0x23,Instruction::new(Mnemonic::INC, 1, 8,  Some([Operand::HL,            Operand::none]))),
        (0x24,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::H,             Operand::none]))),
        (0x25,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::H,             Operand::none]))),
        (0x26,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::H,             Operand::n8]))),
        (0x27,Instruction::new(Mnemonic::DAA, 1, 4,  None)),
        (0x28,Instruction::new(Mnemonic::JR,  2, 12, Some([Operand::Z,             Operand::e8]))), //todo 12/8
        (0x29,Instruction::new(Mnemonic::ADD, 1, 8,  Some([Operand::HL,            Operand::HL]))),
        (0x2A,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::A,             Operand::at_memory_HLI]))),
        (0x2B,Instruction::new(Mnemonic::DEC, 1, 8,  Some([Operand::HL,            Operand::none]))),
        (0x2C,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::L,             Operand::none]))),
        (0x2D,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::L,             Operand::none]))),
        (0x2E,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::L,             Operand::n8]))),
        (0x2F,Instruction::new(Mnemonic::CPL, 1, 4,  None)),

        (0x30,Instruction::new(Mnemonic::JR,  2, 12, Some([Operand::NCY,           Operand::e8]))),//todo: 12/8
        (0x31,Instruction::new(Mnemonic::LD,  3, 12, Some([Operand::SP,            Operand::n16]))),
        (0x32,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HLD, Operand::A]))),
        (0x33,Instruction::new(Mnemonic::INC, 1, 8,  Some([Operand::SP,            Operand::none]))),
        (0x34,Instruction::new(Mnemonic::INC, 1, 12, Some([Operand::at_memory_HL,  Operand::none]))),
        (0x35,Instruction::new(Mnemonic::DEC, 1, 12, Some([Operand::at_memory_HL,  Operand::none]))),
        (0x36,Instruction::new(Mnemonic::LD,  2, 12, Some([Operand::at_memory_HL,  Operand::n8]))),
        (0x37,Instruction::new(Mnemonic::SCF, 1, 4,  None)),
        (0x38,Instruction::new(Mnemonic::JR,  2, 12, Some([Operand::C,             Operand::e8]))),//todo 12/8
        (0x39,Instruction::new(Mnemonic::ADD, 1, 8,  Some([Operand::HL,            Operand::SP]))),
        (0x3A,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::A,             Operand::at_memory_HLD]))),
        (0x3B,Instruction::new(Mnemonic::DEC, 1, 8,  Some([Operand::SP,            Operand::none]))),
        (0x3C,Instruction::new(Mnemonic::INC, 1, 4,  Some([Operand::A,             Operand::none]))),
        (0x3D,Instruction::new(Mnemonic::DEC, 1, 4,  Some([Operand::A,             Operand::none]))),
        (0x3E,Instruction::new(Mnemonic::LD,  2, 8,  Some([Operand::A,             Operand::n8]))),
        (0x3F,Instruction::new(Mnemonic::CCF, 1, 4,  None)),

        (0x40,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::B]))),
        (0x41,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::C]))),
        (0x42,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::D]))),
        (0x43,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::E]))),
        (0x44,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::H]))),
        (0x45,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::L]))),
        (0x46,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::B,             Operand::at_memory_HL]))),
        (0x47,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::B,             Operand::A]))),
        (0x48,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::B]))),
        (0x49,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::C]))),
        (0x4A,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::D]))),
        (0x4B,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::E]))),
        (0x4C,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::H]))),
        (0x4D,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::L]))),
        (0x4E,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::C,             Operand::at_memory_HL]))),
        (0x4F,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::C,             Operand::A]))),

        (0x50,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::B]))),
        (0x51,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::C]))),
        (0x52,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::D]))),
        (0x53,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::E]))),
        (0x54,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::H]))),
        (0x55,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::L]))),
        (0x56,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::D,             Operand::at_memory_HL]))),
        (0x57,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::D,             Operand::A]))),
        (0x58,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::B]))),
        (0x59,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::C]))),
        (0x5A,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::D]))),
        (0x5B,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::E]))),
        (0x5C,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::H]))),
        (0x5D,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::L]))),
        (0x5E,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::E,             Operand::at_memory_HL]))),
        (0x5F,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::E,             Operand::A]))),

        (0x60,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::B]))),
        (0x61,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::C]))),
        (0x62,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::D]))),
        (0x63,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::E]))),
        (0x64,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::H]))),
        (0x65,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::L]))),
        (0x66,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::H,             Operand::at_memory_HL]))),
        (0x67,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::H,             Operand::A]))),
        (0x68,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::B]))),
        (0x69,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::C]))),
        (0x6A,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::D]))),
        (0x6B,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::E]))),
        (0x6C,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::H]))),
        (0x6D,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::L]))),
        (0x6E,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::L,             Operand::at_memory_HL]))),
        (0x6F,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::L,             Operand::A]))),

        (0x70,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::B]))),
        (0x71,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::C]))),
        (0x72,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::D]))),
        (0x73,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::E]))),
        (0x74,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::H]))),
        (0x75,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::L]))),
        (0x76,Instruction::new(Mnemonic::HALT,1, 4,  None)),
        (0x77,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_HL,  Operand::A]))),
        (0x78,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::B]))),
        (0x79,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::C]))),
        (0x7A,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::D]))),
        (0x7B,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::E]))),
        (0x7C,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::H]))),
        (0x7D,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::L]))),
        (0x7E,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0x7F,Instruction::new(Mnemonic::LD,  1, 4,  Some([Operand::A,             Operand::A]))),

        (0x80,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::B]))),
        (0x81,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::C]))),
        (0x82,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::D]))),
        (0x83,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::E]))),
        (0x84,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::H]))),
        (0x85,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::L]))),
        (0x86,Instruction::new(Mnemonic::ADD, 1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0x87,Instruction::new(Mnemonic::ADD, 1, 4,  Some([Operand::A,             Operand::A]))),
        (0x88,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::B]))),
        (0x89,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::C]))),
        (0x8A,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::D]))),
        (0x8B,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::E]))),
        (0x8C,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::H]))),
        (0x8D,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::L]))),
        (0x8E,Instruction::new(Mnemonic::ADC, 1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0x8F,Instruction::new(Mnemonic::ADC, 1, 4,  Some([Operand::A,             Operand::A]))),

        (0x90,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::B]))),
        (0x91,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::C]))),
        (0x92,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::D]))),
        (0x93,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::E]))),
        (0x94,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::H]))),
        (0x95,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::L]))),
        (0x96,Instruction::new(Mnemonic::SUB, 1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0x97,Instruction::new(Mnemonic::SUB, 1, 4,  Some([Operand::A,             Operand::A]))),
        (0x98,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::B]))),
        (0x99,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::C]))),
        (0x9A,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::D]))),
        (0x9B,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::E]))),
        (0x9C,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::H]))),
        (0x9D,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::L]))),
        (0x9E,Instruction::new(Mnemonic::SBC, 1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0x9F,Instruction::new(Mnemonic::SBC, 1, 4,  Some([Operand::A,             Operand::A]))),

        (0xA0,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::B]))),
        (0xA1,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::C]))),
        (0xA2,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::D]))),
        (0xA3,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::E]))),
        (0xA4,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::H]))),
        (0xA5,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::L]))),
        (0xA6,Instruction::new(Mnemonic::AND, 1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0xA7,Instruction::new(Mnemonic::AND, 1, 4,  Some([Operand::A,             Operand::A]))),
        (0xA8,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::B]))),
        (0xA9,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::C]))),
        (0xAA,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::D]))),
        (0xAB,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::E]))),
        (0xAC,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::H]))),
        (0xAD,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::L]))),
        (0xAE,Instruction::new(Mnemonic::XOR, 1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0xAF,Instruction::new(Mnemonic::XOR, 1, 4,  Some([Operand::A,             Operand::A]))),

        (0xB0,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::B]))),
        (0xB1,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::C]))),
        (0xB2,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::D]))),
        (0xB3,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::E]))),
        (0xB4,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::H]))),
        (0xB5,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::L]))),
        (0xB6,Instruction::new(Mnemonic::OR,  1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0xB7,Instruction::new(Mnemonic::OR,  1, 4,  Some([Operand::A,             Operand::A]))),
        (0xB8,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::B]))),
        (0xB9,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::C]))),
        (0xBA,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::D]))),
        (0xBB,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::E]))),
        (0xBC,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::H]))),
        (0xBD,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::L]))),
        (0xBE,Instruction::new(Mnemonic::CP,  1, 8,  Some([Operand::A,             Operand::at_memory_HL]))),
        (0xBF,Instruction::new(Mnemonic::CP,  1, 4,  Some([Operand::A,             Operand::A]))),

        (0xC0,Instruction::new(Mnemonic::RET, 1, 20, Some([Operand::NZ,            Operand::none]))),//todo 20/8
        (0xC1,Instruction::new(Mnemonic::POP, 1, 12, Some([Operand::BC,            Operand::none]))),
        (0xC2,Instruction::new(Mnemonic::JP,  3, 16, Some([Operand::NZ,            Operand::a16]))),//todo 16/12
        (0xC3,Instruction::new(Mnemonic::JP,  3, 16, Some([Operand::a16,           Operand::none]))),
        (0xC4,Instruction::new(Mnemonic::CALL,3, 24, Some([Operand::NZ,            Operand::a16]))),//todo 24/12
        (0xC5,Instruction::new(Mnemonic::PUSH,1, 16, Some([Operand::BC,            Operand::none]))),
        (0xC6,Instruction::new(Mnemonic::ADD, 2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xC7,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x00),     Operand::none]))),
        (0xC8,Instruction::new(Mnemonic::RET, 1, 20, Some([Operand::Z,             Operand::none]))),//todo 20/8
        (0xC9,Instruction::new(Mnemonic::RET, 1, 16, None)),
        (0xCA,Instruction::new(Mnemonic::JP,  3, 16, Some([Operand::Z,             Operand::a16]))),//todo 16/12
        (0xCC,Instruction::new(Mnemonic::CALL,3, 24, Some([Operand::Z,             Operand::a16]))),//todo 24/12
        (0xCD,Instruction::new(Mnemonic::CALL,3, 24, Some([Operand::a16,           Operand::none]))),
        (0xCE,Instruction::new(Mnemonic::ADC, 2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xCF,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x08),     Operand::none]))),

        (0xD0,Instruction::new(Mnemonic::RET, 1, 20, Some([Operand::NCY,           Operand::none]))),//todo 20/8
        (0xD1,Instruction::new(Mnemonic::POP, 1, 12, Some([Operand::DE,            Operand::none]))),
        (0xD2,Instruction::new(Mnemonic::JP,  3, 16, Some([Operand::NCY,           Operand::a16]))),//todo 16/12
        (0xD4,Instruction::new(Mnemonic::CALL,3, 24, Some([Operand::NCY,           Operand::a16]))),//todo 24/12
        (0xD5,Instruction::new(Mnemonic::PUSH,1, 16, Some([Operand::DE,            Operand::none]))),
        (0xD6,Instruction::new(Mnemonic::SUB, 2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xD7,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x10),     Operand::none]))),
        (0xD8,Instruction::new(Mnemonic::RET, 1, 20, Some([Operand::C,             Operand::none]))),//todo 20/8
        (0xD9,Instruction::new(Mnemonic::RETI,1, 16, None)),
        (0xDA,Instruction::new(Mnemonic::JP,  3, 16, Some([Operand::C,             Operand::a16]))),//todo 16/12
        (0xDC,Instruction::new(Mnemonic::CALL,3, 24, Some([Operand::C,             Operand::a16]))),//todo 24/12
        (0xDE,Instruction::new(Mnemonic::SBC, 2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xDF,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x18),     Operand::none]))),

        (0xE0,Instruction::new(Mnemonic::LDH, 2, 12, Some([Operand::at_memory_a8,  Operand::A]))),
        (0xE1,Instruction::new(Mnemonic::POP, 1, 12, Some([Operand::HL,            Operand::none]))),
        (0xE2,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::at_memory_C,   Operand::A]))),
        (0xE5,Instruction::new(Mnemonic::PUSH,1, 16, Some([Operand::HL,            Operand::none]))),
        (0xE6,Instruction::new(Mnemonic::AND, 2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xE7,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x20),     Operand::none]))),
        (0xE8,Instruction::new(Mnemonic::ADD, 2, 16, Some([Operand::SP,            Operand::e8]))),
        (0xE9,Instruction::new(Mnemonic::JP,  1, 4,  Some([Operand::HL,            Operand::none]))),
        (0xEA,Instruction::new(Mnemonic::LD,  3, 16, Some([Operand::at_memory_a16, Operand::A]))),
        (0xEE,Instruction::new(Mnemonic::XOR, 2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xEF,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x28),     Operand::none]))),

        (0xF0,Instruction::new(Mnemonic::LDH, 2, 12, Some([Operand::A,             Operand::at_memory_a8]))),
        (0xF1,Instruction::new(Mnemonic::POP, 1, 12, Some([Operand::AF,            Operand::none]))),
        (0xF2,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::A,             Operand::at_memory_C]))),
        (0xF3,Instruction::new(Mnemonic::DI,  1, 4,  None)),
        (0xF5,Instruction::new(Mnemonic::PUSH,1, 16, Some([Operand::AF,            Operand::none]))),
        (0xF6,Instruction::new(Mnemonic::OR,  2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xF7,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x30),     Operand::none]))),
        (0xF8,Instruction::new(Mnemonic::LD,  2, 12, Some([Operand::HL,            Operand::SP_plus_e8]))),
        (0xF9,Instruction::new(Mnemonic::LD,  1, 8,  Some([Operand::SP,            Operand::HL]))),
        (0xFA,Instruction::new(Mnemonic::LD,  3, 16, Some([Operand::A,             Operand::at_memory_a16]))),
        (0xFB,Instruction::new(Mnemonic::EI,  1, 4,  None)),
        (0xFE,Instruction::new(Mnemonic::CP,  2, 8,  Some([Operand::A,             Operand::n8]))),
        (0xFF,Instruction::new(Mnemonic::RST, 1, 16, Some([Operand::vec(0x38),     Operand::none]))),

        (0xCB,Instruction::new(Mnemonic::PREFIX,1,4, None))

    ]);
}

pub struct Cpu {
    //registers
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,

    //flags
    z: bool,
    n: bool,
    h: bool,
    c: bool,

    // stack pointer and program counter
    pub sp: u16,
    pub pc: u16,

    databus: Rc<DataBus>,
}

impl Cpu {
    pub fn new(databus: Rc<DataBus>) -> Cpu {
        Cpu {
            A: 0x01,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            H: 0x01,
            L: 0x4D,

            z: true,
            n: false,
            h: false,
            c: false,

            sp: 0xFFFE,
            pc: 0x0100,

            databus,
        }
    }

    pub fn exec_instruction(&mut self, instruction_byte: u8) {
        let instruction = INSTRUCTIONS_MAP.get(&instruction_byte).unwrap();
        if !matches!(instruction.mnemonic, Mnemonic::PREFIX) {
            match instruction.mnemonic {

                Mnemonic::ADC => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("ADC A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("ADC A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("ADC A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("ADC A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("ADC A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("ADC A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("ADC A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("ADC A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("ADC A, n8");
                        }

                        _ => {
                            eprintln!("Non existing ADC instruction.");
                        }
                    }
                }

                Mnemonic::ADD => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::HL, Operand::BC] => {
                            println!("ADD HL, BC");
                        }

                        [Operand::HL, Operand::DE] => {
                            println!("ADD HL, DE");
                        }

                        [Operand::HL, Operand::HL] => {
                            println!("ADD HL, HL");
                        }

                        [Operand::HL, Operand::SP] => {
                            println!("ADD HL, SP");
                        }

                        [Operand::A, Operand::B] => {
                            println!("ADD A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("ADD A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("ADD A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("ADD A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("ADD A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("ADD A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("ADD A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("ADD A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("ADD A, n8");
                        }

                        [Operand::SP, Operand::e8] => {
                            println!("ADD SP, e8");
                        }

                        _ => {
                            eprintln!("Non existing ADD instruction.");
                        }

                    }
                }

                Mnemonic::AND => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("AND A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("AND A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("AND A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("AND A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("AND A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("AND A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("AND A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("AND A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("AND A, n8");
                        }

                        _ => {
                            eprintln!("Non existing AND instruction.");
                        }
                    }
                }

                Mnemonic::BIT => {
                }

                Mnemonic::CALL => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::NZ, Operand::a16] => {
                            println!("CALL NZ, a16");
                        }

                        [Operand::Z, Operand::a16] => {
                            println!("CALL Z, a16");
                        }

                        [Operand::a16, Operand::none] => {
                            println!("CALL a16");
                        }

                        [Operand::NCY, Operand::a16] => {
                            println!("CALL NCY, a16");
                        }

                        [Operand::C, Operand::a16] => {
                            println!("CALL C, a16");
                        }

                        _ => {
                            eprintln!("Non existing CALL instruction.");
                        }


                    }
                }

                Mnemonic::CCF => {
                    println!("CCF");
                }

                Mnemonic::CP => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("CP A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("CP A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("CP A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("CP A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("CP A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("CP A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("CP A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("CP A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("CP A, n8");
                        }

                        _ => {
                            eprintln!("Non existing CP instruction.");
                        }
                    }
                }

                Mnemonic::CPL => {
                    println!("CPL");
                }

                Mnemonic::DAA => {
                    println!("DAA");
                }

                Mnemonic::DEC => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::B, Operand::none] => {
                            println!("DEC B");
                        }

                        [Operand::BC, Operand::none] => {
                            println!("DEC BC");
                        }

                        [Operand::C, Operand::none] => {
                            println!("DEC C");
                        }

                        [Operand::D, Operand::none] => {
                            println!("DEC D");
                        }

                        [Operand::DE, Operand::none] => {
                            println!("DEC DE");
                        }

                        [Operand::E, Operand::none] => {
                            println!("DEC E");
                        }

                        [Operand::H, Operand::none] => {
                            println!("DEC H");
                        }

                        [Operand::HL, Operand::none] => {
                            println!("DEC HL");
                        }

                        [Operand::L, Operand::none] => {
                            println!("DEC L");
                        }

                        [Operand::at_memory_HL, Operand::none] => {
                            println!("DEC at_memory_HL");
                        }

                        [Operand::SP, Operand::none] => {
                            println!("DEC SP");
                        }

                        [Operand::A, Operand::none] => {
                            println!("DEC A");
                        }

                        _ => {
                            eprintln!("Non existing DEC instruction.");
                        }
                    }
                }

                Mnemonic::DI => {
                    println!("DI");
                }

                Mnemonic::EI => {
                    println!("EI");
                }

                Mnemonic::HALT => {
                    println!("HALT");
                }

                Mnemonic::INC => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::BC, Operand::none] => {
                            println!("INC BC");
                        }

                        [Operand::B, Operand::none] => {
                            println!("INC B");
                        }

                        [Operand::C, Operand::none] => {
                            println!("INC C");
                        }

                        [Operand::DE, Operand::none] => {
                            println!("INC DE");
                        }

                        [Operand::D, Operand::none] => {
                            println!("INC D");
                        }

                        [Operand::E, Operand::none] => {
                            println!("INC E");
                        }

                        [Operand::HL, Operand::none] => {
                            println!("INC HL");
                        }

                        [Operand::H, Operand::none] => {
                            println!("INC H");
                        }

                        [Operand::L, Operand::none] => {
                            println!("INC L");
                        }

                        [Operand::SP, Operand::none] => {
                            println!("INC SP");
                        }

                        [Operand::at_memory_HL, Operand::none] => {
                            println!("INC at_memory_HL");
                        }

                        [Operand::A, Operand::none] => {
                            println!("INC A");
                        }

                        _ => {
                            eprintln!("Non existing INC instruction.");
                        }
                    }
                }

                Mnemonic::JP => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::NZ, Operand::a16] => {
                            println!("JP NZ, a16");
                        }

                        [Operand::a16, Operand::none] => {
                            println!("JP a16");
                        }

                        [Operand::Z, Operand::a16] => {
                            println!("JP Z, a16");
                        }

                        [Operand::NCY, Operand::a16] => {
                            println!("JP NCY, a16");
                        }

                        [Operand::C, Operand::a16] => {
                            println!("JP C, a16");
                        }

                        [Operand::HL, Operand::none] => {
                            println!("JP HL");
                        }

                        _ => {
                            eprintln!("Non existing JP instruction.");
                        }

                    }
                }

                Mnemonic::JR => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::e8, Operand::none] => {
                            println!("JR e8");
                        }

                        [Operand::NZ, Operand::e8] => {
                            println!("JR NZ, e8");
                        }

                        [Operand::Z, Operand::e8] => {
                            println!("JR Z, e8");
                        }

                        [Operand::NCY, Operand::e8] => {
                            println!("JR NCY, e8");
                        }

                        [Operand::C, Operand::e8] => {
                            println!("JR C, e8");
                        }

                        _ => {
                            eprintln!("Non existing JR instruction.");
                        }
                    }
                }

                Mnemonic::LD => {
                    match instruction.operands.as_ref().unwrap() {
                        [Operand::BC, Operand::n16] => {
                            println!("LD BC n16");
                        }

                        [Operand::at_memory_BC, Operand::A] => {
                            println!("LD AT_MEMORY_BC A");
                        }

                        [Operand::B, Operand::n8] => {
                            println!("LD B n8");
                        }

                        [Operand::at_memory_a16, Operand::SP] => {
                            println!("LD [a16] SP");
                        }

                        [Operand::A, Operand::at_memory_BC] => {
                            println!("LD A, [BC]");
                        }

                        [Operand::C, Operand::n8] => {
                            println!("LD C, n8");
                        }

                        [Operand::DE, Operand::n16] => {
                            println!("LD DE, n16");
                        }

                        [Operand::at_memory_DE, Operand::A] => {
                            println!("LD at_memory_DE, A");
                        }

                        [Operand::D, Operand::n8] => {
                            println!("LD D, n8");
                        }

                        [Operand::A, Operand::at_memory_DE] => {
                            println!("LD A, at_memory_DE");
                        }

                        [Operand::E, Operand::n8] => {
                            println!("LD E, n8");
                        }

                        [Operand::HL, Operand::n16] => {
                            println!("LD HL, n16");
                        }

                        [Operand::at_memory_HLI, Operand::A] => {
                            println!("LD at_memory_HLI, A");
                        }

                        [Operand::H, Operand::n8] => {
                            println!("LD H, n8");
                        }

                        [Operand::A, Operand::at_memory_HLI] => {
                            println!("LD A, at_memory_HLI");
                        }

                        [Operand::L, Operand::n8] => {
                            println!("LD L, n8");
                        }

                        [Operand::SP, Operand::n16] => {
                            println!("LD SP, n16");
                        }

                        [Operand::at_memory_HLD, Operand::A] => {
                            println!("LD at_memory_HLD, A");
                        }

                        [Operand::at_memory_HL, Operand::n8] => {
                            println!("LD at_memory_HL, n8");
                        }

                        [Operand::A, Operand::at_memory_HLD] => {
                            println!("LD A, at_memory_HLD");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("LD A, n8");
                        }

                        [Operand::B, Operand::B] => {
                            println!("LD B, B");
                        }

                        [Operand::B, Operand::C] => {
                            println!("LD B, C");
                        }

                        [Operand::B, Operand::D] => {
                            println!("LD B, D");
                        }

                        [Operand::B, Operand::E] => {
                            println!("LD B, E");
                        }

                        [Operand::B, Operand::H] => {
                            println!("LD B, H");
                        }

                        [Operand::B, Operand::L] => {
                            println!("LD B, L");
                        }

                        [Operand::B, Operand::at_memory_HL] => {
                            println!("LD B, at_memory_HL");
                        }

                        [Operand::B, Operand::A] => {
                            println!("LD B, A");
                        }

                        [Operand::C, Operand::B] => {
                            println!("LD C, B");
                        }

                        [Operand::C, Operand::C] => {
                            println!("LD C, C");
                        }

                        [Operand::C, Operand::D] => {
                            println!("LD C, D");
                        }

                        [Operand::C, Operand::E] => {
                            println!("LD C, E");
                        }

                        [Operand::C, Operand::H] => {
                            println!("LD C, H");
                        }

                        [Operand::C, Operand::L] => {
                            println!("LD C, L");
                        }

                        [Operand::C, Operand::at_memory_HL] => {
                            println!("LD C, at_memory_HL");
                        }

                        [Operand::C, Operand::A] => {
                            println!("LD C, A");
                        }

                        [Operand::D, Operand::B] => {
                            println!("LD D, B");
                        }

                        [Operand::D, Operand::C] => {
                            println!("LD D, C");
                        }

                        [Operand::D, Operand::D] => {
                            println!("LD D, D");
                        }

                        [Operand::D, Operand::E] => {
                            println!("LD D, E");
                        }

                        [Operand::D, Operand::H] => {
                            println!("LD D, H");
                        }

                        [Operand::D, Operand::L] => {
                            println!("LD D, L");
                        }

                        [Operand::D, Operand::at_memory_HL] => {
                            println!("LD D, at_memory_HL");
                        }

                        [Operand::D, Operand::A] => {
                            println!("LD D, A");
                        }

                        [Operand::E, Operand::B] => {
                            println!("LD E, B");
                        }

                        [Operand::E, Operand::C] => {
                            println!("LD E, C");
                        }

                        [Operand::E, Operand::D] => {
                            println!("LD E, D");
                        }

                        [Operand::E, Operand::E] => {
                            println!("LD E, E");
                        }

                        [Operand::E, Operand::H] => {
                            println!("LD E, H");
                        }

                        [Operand::E, Operand::L] => {
                            println!("LD E, L");
                        }

                        [Operand::E, Operand::at_memory_HL] => {
                            println!("LD E, at_memory_HL");
                        }

                        [Operand::E, Operand::A] => {
                            println!("LD E, A");
                        }

                        [Operand::H, Operand::B] => {
                            println!("LD H, B");
                        }

                        [Operand::H, Operand::C] => {
                            println!("LD H, C");
                        }

                        [Operand::H, Operand::D] => {
                            println!("LD H, D");
                        }

                        [Operand::H, Operand::E] => {
                            println!("LD H, E");
                        }

                        [Operand::H, Operand::H] => {
                            println!("LD H, H");
                        }

                        [Operand::H, Operand::L] => {
                            println!("LD H, L");
                        }

                        [Operand::H, Operand::at_memory_HL] => {
                            println!("LD H, at_memory_HL");
                        }

                        [Operand::H, Operand::A] => {
                            println!("LD H, A");
                        }

                        [Operand::L, Operand::B] => {
                            println!("LD L, B");
                        }

                        [Operand::L, Operand::C] => {
                            println!("LD L, C");
                        }

                        [Operand::L, Operand::D] => {
                            println!("LD L, D");
                        }

                        [Operand::L, Operand::E] => {
                            println!("LD L, E");
                        }

                        [Operand::L, Operand::H] => {
                            println!("LD L, H");
                        }

                        [Operand::L, Operand::L] => {
                            println!("LD L, L");
                        }

                        [Operand::L, Operand::at_memory_HL] => {
                            println!("LD L, at_memory_HL");
                        }

                        [Operand::L, Operand::A] => {
                            println!("LD L, A");
                        }

                        [Operand::at_memory_HL, Operand::B] => {
                            println!("LD at_memory_HL, B");
                        }

                        [Operand::at_memory_HL, Operand::C] => {
                            println!("LD at_memory_HL, C");
                        }

                        [Operand::at_memory_HL, Operand::D] => {
                            println!("LD at_memory_HL, D");
                        }

                        [Operand::at_memory_HL, Operand::E] => {
                            println!("LD at_memory_HL, E");
                        }

                        [Operand::at_memory_HL, Operand::H] => {
                            println!("LD at_memory_HL, H");
                        }

                        [Operand::at_memory_HL, Operand::L] => {
                            println!("LD at_memory_HL, L");
                        }

                        [Operand::at_memory_HL, Operand::A] => {
                            println!("LD at_memory_HL, A");
                        }

                        [Operand::A, Operand::B] => {
                            println!("LD A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("LD A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("LD A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("LD A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("LD A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("LD A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("LD A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("LD A, A");
                        }

                        [Operand::at_memory_C, Operand::A] => {
                            println!("LD at_memory_C, A");
                        }

                        [Operand::at_memory_a16, Operand::A] => {
                            println!("LD at_memory_a16, A");
                        }

                        [Operand::A, Operand::at_memory_C] => {
                            println!("LD A, at_memory_C");
                        }

                        [Operand::HL, Operand::SP_plus_e8] => {
                            println!("LD HL, SP + e8");
                        }

                        [Operand::SP, Operand::HL] => {
                            println!("LD SP, HL");
                        }

                        [Operand::A, Operand::at_memory_a16] => {
                            println!("LD A, at_memory_a16");
                        }

                        _ => {
                            eprintln!("Non existing LD instruction.");
                        }
                    }
                }

                Mnemonic::LDH => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::at_memory_a8, Operand::A] => {
                            println!("LDH at_memory_a8, A");
                        }

                        [Operand::A, Operand::at_memory_a8] => {
                            println!("LDH A, at_memory_a8");
                        }

                        _ => {
                            eprintln!("Non existing LDH instruction.");
                        }

                    }

                }

                Mnemonic::NOP => {
                    println!("NOP");
                }

                Mnemonic::OR => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("OR A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("OR A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("OR A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("OR A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("OR A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("OR A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("OR A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("OR A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("OR A, n8");
                        }

                        _ => {
                            eprintln!("Non existing OR instruction.");
                        }
                    }
                }

                Mnemonic::POP => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::BC, Operand::none] => {
                            println!("POP BC");
                        }

                        [Operand::DE, Operand::none] => {
                            println!("POP DE");
                        }

                        [Operand::HL, Operand::none] => {
                            println!("POP HL");
                        }

                        [Operand::AF, Operand::none] => {
                            println!("POP AF");
                        }

                        _ => {
                            eprintln!("Non existing POP instruction.");
                        }
                    }
                }

                Mnemonic::PREFIX => {
                    println!("PREFIX");
                }

                Mnemonic::PUSH => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::BC, Operand::none] => {
                            println!("PUSH BC");
                        }

                        [Operand::DE, Operand::none] => {
                            println!("PUSH DE");
                        }

                        [Operand::HL, Operand::none] => {
                            println!("PUSH HL");
                        }

                        [Operand::AF, Operand::none] => {
                            println!("PUSH AF");
                        }

                        _ => {
                            eprintln!("Non existing PUSH instruction.");
                        }

                    }
                }

                Mnemonic::RET => {
                    if instruction.operands.is_none() {
                        println!("RET");
                    }
                    else {
                        match instruction.operands.as_ref().unwrap() {

                            [Operand::NZ, Operand::none] => {
                                println!("RET NZ");
                            }

                            [Operand::Z, Operand::none] => {
                                println!("RET Z");
                            }

                            [Operand::NCY, Operand::none] => {
                                println!("RET NCY");
                            }

                            [Operand::C, Operand::none] => {
                                println!("RET C");
                            }

                            _ => {
                                eprintln!("Non existing RET instruction.");
                            }
                        }
                    }
                }

                Mnemonic::RETI => {
                    println!("RETI");
                }

                Mnemonic::RLA => {
                    println!("RLA");
                }

                Mnemonic::RLCA => {
                    println!("RLCA");
                }

                Mnemonic::RRA => {
                    println!("RRA");
                }

                Mnemonic::RRCA => {
                    println!("RRCA");
                }

                Mnemonic::RST => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::vec(0x00), Operand::none] => {
                            println!("RST $00");
                        }

                        [Operand::vec(0x08), Operand::none] => {
                            println!("RST $08");
                        }

                        [Operand::vec(0x10), Operand::none] => {
                            println!("RST $10");
                        }

                        [Operand::vec(0x18), Operand::none] => {
                            println!("RST $18");
                        }

                        [Operand::vec(0x20), Operand::none] => {
                            println!("RST $20");
                        }

                        [Operand::vec(0x28), Operand::none] => {
                            println!("RST $28");
                        }

                        [Operand::vec(0x30), Operand::none] => {
                            println!("RST $30");
                        }

                        [Operand::vec(0x38), Operand::none] => {
                            println!("RST $38");
                        }

                        _ => {
                            eprintln!("Non existing RST instruction.");
                        }

                    }
                }

                Mnemonic::SBC => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("SBC A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("SBC A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("SBC A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("SBC A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("SBC A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("SBC A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("SBC A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("SBC A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("SBC A, n8");
                        }

                        _ => {
                            eprintln!("Non existing SBC instruction.");
                        }
                    }
                }

                Mnemonic::SCF => {
                    println!("SCF");
                }

                Mnemonic::STOP => {
                    println!("STOP");
                }

                Mnemonic::SUB => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("SUB A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("SUB A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("SUB A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("SUB A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("SUB A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("SUB A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("SUB A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("SUB A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("SUB A, n8");
                        }

                        _ => {
                            eprintln!("Non existing SUB instruction.");
                        }
                    }
                }

                Mnemonic::XOR => {
                    match instruction.operands.as_ref().unwrap() {

                        [Operand::A, Operand::B] => {
                            println!("XOR A, B");
                        }

                        [Operand::A, Operand::C] => {
                            println!("XOR A, C");
                        }

                        [Operand::A, Operand::D] => {
                            println!("XOR A, D");
                        }

                        [Operand::A, Operand::E] => {
                            println!("XOR A, E");
                        }

                        [Operand::A, Operand::H] => {
                            println!("XOR A, H");
                        }

                        [Operand::A, Operand::L] => {
                            println!("XOR A, L");
                        }

                        [Operand::A, Operand::at_memory_HL] => {
                            println!("XOR A, at_memory_HL");
                        }

                        [Operand::A, Operand::A] => {
                            println!("XOR A, A");
                        }

                        [Operand::A, Operand::n8] => {
                            println!("XOR A, n8");
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
        }
       // else {

       // }
    }
}
