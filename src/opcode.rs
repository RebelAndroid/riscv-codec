// Table 70, page 553 of the Unprivileged ISA Manual
#[derive(Debug)]
pub enum Opcode {
    Load = 0b00_000_11,
    Auipc = 0b00_101_11,
    Store = 0b01_000_11,
    Lui = 0b01_101_11,
    Op = 0b01_100_11,
    Op32 = 0b01_110_11,
    OpImm = 0b00_100_11,
    OpImm32 = 0b00_110_11,
    Jalr = 0b11_001_11,
    Jal = 0b11_011_11,
    Reserved = 0,
}

impl Opcode {
    pub fn from_int(int: u32) -> Self {
        if int > 0b11_111_11 {
            panic!("attempted to convert too large int to opcode")
        }
        return match int {
            0b00_000_11 => Self::Load,
            0b00_101_11 => Self::Auipc,
            0b01_000_11 => Self::Store,
            0b01_101_11 => Self::Lui,
            0b01_100_11 => Self::Op,
            0b01_110_11 => Self::Op32,
            0b00_100_11 => Self::OpImm,
            0b00_110_11 => Self::OpImm32,
            0b11_001_11 => Self::Jalr,
            0b11_011_11 => Self::Jal,
            _ => Self::Reserved,
        };
    }
}
