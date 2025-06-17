use std::fmt::{Display, Formatter};

/// The immediate values in I-type instructions (like addi)
#[derive(Debug, PartialEq)]
pub struct IImmediate {
    val: i16,
}

impl IImmediate {
    /// Extracts the IImmediate from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let unsigned: u32 = ((x >> 20) & 0b1111_1111_1111).try_into().unwrap();
        println!("u {:b}", unsigned);
        // sign extend 12 bit value
        let y = unsigned.overflowing_shl(20).0 as i32;
        println!("y {:b}", y);
        let val = y.overflowing_shr(20).0 as i16;
        println!("v {:b}", val);
        IImmediate { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2047 || val < -2048 {
            panic!("attempted to construct out of range IImediate")
        }
        IImmediate { val: val as i16 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for IImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in S-type instructions (like SW)
#[derive(Debug, PartialEq)]
pub struct SImmediate {
    val: i16,
}

impl SImmediate {
    /// Extracts the IImmediate from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let unsigned: u32 =
            (((x >> 25) & 0b111_1111) << 5) | ((x >> 7) & 0b1_1111);
        // sign extend 12 bit value
        let y = unsigned.overflowing_shl(20).0 as i32;
        let val = y.overflowing_shr(20).0 as i16;
        SImmediate { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2047 || val < -2048 {
            panic!("attempted to construct out of range IImediate")
        }
        SImmediate { val: val as i16 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for SImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in shift-by-immediate instructions (SRAI)
#[derive(Debug, PartialEq)]
pub struct Shamt {
    val: u8,
}

impl Shamt {
    /// Extracts the IImmediate from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let val: u8 = ((x >> 20) & 0b11_1111) as u8;
        Shamt { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 63 || val < 0 {
            panic!("attempted to construct out of range Shamt")
        }
        Shamt { val: val as u8 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for Shamt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate values in shift-by-immediate word instructions (SRAIW)
#[derive(Debug, PartialEq)]
pub struct ShamtW {
    val: u8,
}

impl ShamtW {
    /// Extracts the IImmediate from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let val: u8 = ((x >> 20) & 0b1_1111) as u8;
        ShamtW { val }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 31 || val < 0 {
            panic!("attempted to construct out of range Shamtw")
        }
        ShamtW { val: val as u8 }
    }

    pub fn val(&self) -> i64 {
        return self.val.into();
    }
}

impl Display for ShamtW {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}
