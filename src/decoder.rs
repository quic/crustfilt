use bitmatch::bitmatch;
use std::fmt::{Debug, Display};

pub trait Insn: Display + Debug {}

pub fn decode(sz: usize, insn: Vec<u16>) -> Result<Box<dyn Insn>, ()> {
    match sz {
        2 => decode_16(insn[0]),
        4 => decode_32(((insn[0] as u32) << 16) | (insn[1] as u32)),
        6 => decode_48(insn[0] as u64 | ((insn[1] as u64) << 16) | ((insn[2] as u64) << 32)),
        8 => decode_64(
            (insn[0] as u64) << 16
                | ((insn[1] as u64) << 0)
                | ((insn[2] as u64) << 48)
                | ((insn[3] as u64) << 32),
        ),
        _ => Err(()),
    }
}

#[derive(Debug)]
struct CmPush {
    urlist: u16,
    spimm: u16,
}

// Generally speaking most of these should be autogeneratable in macros
// this example has 4 irregular insns just for testing
impl std::fmt::Display for CmPush {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.urlist {
            4 => write!(f, "CmPush {{ra}},"),
            5 => write!(f, "CmPush {{ra, s0}},"),
            6..15 => write!(f, "CmPush {{ra, s0-s{}}},", self.urlist - 5),
            15 => write!(f, "CmPush {{ra, s0-s11}},"),
            _ => Err(std::fmt::Error::default()),
        }?;
        write!(f, "-{}", self.spimm * 16)
    }
}

#[derive(Debug)]
struct Insbi {
    rd: u8,
    imm: u8,
    shift: u8,
    width: u8,
}

impl std::fmt::Display for Insbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "insbi x{}, #0x{:x}, #0x{:x}, #0x{:x}",
            self.rd, self.imm, self.shift, self.width
        )
    }
}

#[derive(Debug)]
struct XqciELi {
    rd: u8,
    imm: u32,
}

impl std::fmt::Display for XqciELi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xqci.e.li {}, {}", self.rd, self.imm)
    }
}

#[derive(Debug)]
struct Fake64 {}

impl std::fmt::Display for Fake64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fake64")
    }
}

impl Insn for CmPush {}
impl Insn for Insbi {}
impl Insn for Fake64 {}
impl Insn for XqciELi {}

#[bitmatch]
fn decode_16(insn: u16) -> Result<Box<dyn Insn>, ()> {
    #[bitmatch]
    match insn {
        "10111000uuuuss10" => Ok(Box::new(CmPush {
            urlist: u,
            spimm: s,
        })),
        _ => Err(()),
    }
}

#[bitmatch]
fn decode_32(insn: u32) -> Result<Box<dyn Insn>, ()> {
    #[bitmatch]
    match insn {
        "00wwwwwsssssiiiii001ddddd0001011" if d != 0 => Ok(Box::new(Insbi {
            rd: d as u8,
            imm: i as u8,
            shift: s as u8,
            width: w as u8,
        })),
        _ => Err(()),
    }
}
#[bitmatch]
fn decode_48(insn: u64) -> Result<Box<dyn Insn>, ()> {
    #[bitmatch]
    match insn {
        "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii0000ddddd0011111" if d != 0 => Ok(Box::new(XqciELi {
            rd: d as u8,
            imm: i as u32,
        })),
        _ => Err(()),
    }
}

#[bitmatch]
fn decode_64(insn: u64) -> Result<Box<dyn Insn>, ()> {
    println!("DEBUG: {insn:x}");
    #[bitmatch]
    match insn {
        "00100000000000000010000000001001010000000000000000111111" => Ok(Box::new(Fake64 {})),
        _ => Err(()),
    }
}
