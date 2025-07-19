use std::fmt::{Display, Formatter};

use proc_macros::make_immediate;



make_immediate!(IImmediate true (0 12 20));
make_immediate!(SImmediate true (0 5 7) (5 7 25));
make_immediate!(UImmediate true (0 20 12));
make_immediate!(JImmediate true (12 8 12) (11 1 20) (1 10 21) (20 1 31));
make_immediate!(BImmediate true (11 1 7) (1 4 8) (5 6 25) (12 1 31));

make_immediate!(Shamt false (0 6 20));
make_immediate!(ShamtW false (0 5 20));

/// The immediate value in wide immediate compressed instructions
#[derive(Debug, PartialEq)]
pub struct CWideImmediate {
    val: i32,
}


impl CWideImmediate {
    /// Extracts the `CWImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 5) & 0b1;
        let b = (x >> 6) & 0b1;
        let c = (x >> 7) & 0b1111;
        let d = (x >> 11) & 0b11;
        
        let i: i32 = ((b << 2) | (a << 3) | (d << 4) | (c << 6)) as i32;
        // CWImmediate is zero-extended
        CWideImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(10) - 1 || val < 0 {
            panic!("attempted to construct out of range CWideImmediate")
        }
        if val % 4 != 0 {
            panic!("attempted to construct non multiple of 4 CWideImmediate")
        }
        CWideImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CWideImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed, load/store doubleword instructions
#[derive(Debug, PartialEq)]
pub struct CDImmediate {
    val: i32,
}


impl CDImmediate {
    /// Extracts the `CLDImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 5) & 0b11;
        let b = (x >> 10) & 0b111;

        let i: i32 = ((b << 3) | (a << 6)) as i32;
        // CLDImmediate is zero-extended
        CDImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(8) - 1 || val < 0 {
            panic!("attempted to construct out of range CDImmediate")
        }
        if val % 8 != 0 {
            panic!("attempted to construct non multiple of 8 CDImmediate")
        }
        CDImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CDImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed, load/store word instructions
#[derive(Debug, PartialEq)]
pub struct CWImmediate {
    val: i32,
}


impl CWImmediate {
    /// Extracts the `CLWImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 5) & 0b1;
        let b = (x >> 6) & 0b1;
        let c = (x >> 10) & 0b111;

        let i: i32 = ((b << 2) | (c << 3) | (a << 6)) as i32;
        // CLWImmediate is zero-extended
        CWImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(7) - 1 || val < 0 {
            panic!("attempted to construct out of range CWImmediate")
        }
        if val % 4 != 0 {
            panic!("attempted to construct non multiple of 4 CWImmediate")
        }
        CWImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CWImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed immediate operation instructions
#[derive(Debug, PartialEq)]
pub struct CIImmediate {
    val: i32,
}

impl CIImmediate {
    /// Extracts the `CIImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1_1111;
        let b = (x >> 12) & 0b1;
        let i: i32 = (a | (b << 5)) as i32;
        let i2 = (i << 26) >> 26;
        CIImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(5) - 1 || val < -2i64.pow(5) {
            panic!("attempted to construct out of range CIImmediate")
        }
        CIImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CIImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed branch instructions
#[derive(Debug, PartialEq)]
pub struct CBImmediate {
    val: i32,
}

impl CBImmediate {
    /// Extracts the `CBImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1;
        let b = (x >> 3) & 0b11;
        let c = (x >> 5) & 0b11;
        let d = (x >> 10) & 0b11;
        let e = (x >> 12) & 0b1;
        let i: i32 = ((b << 1) | (d << 3) | (a << 5) | (c << 6) | (e << 8)) as i32;
        let i2 = (i << 23) >> 23;
        CBImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(8) - 1 || val < -2i64.pow(8) {
            panic!("attempted to construct out of range CBImmediate")
        }
        CBImmediate { val: val as i32 }
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CBImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in compressed bit shift instructions
#[derive(Debug, PartialEq)]
pub struct CShamt {
    val: u32,
}

impl CShamt {
    /// Extracts the `CShamt` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1_1111;
        let b = (x >> 12) & 0b1;
        let i: u32 = (a | (b << 5)) as u32;
        CShamt { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(6) - 1 || val < 0 {
            panic!("attempted to construct out of range CIImmediate")
        }
        CShamt { val: val as u32 }
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CShamt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in a compressed jump instruction
#[derive(Debug, PartialEq)]
pub struct CJImmediate {
    val: i32,
}

impl CJImmediate {
    /// Extracts the `CJImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = (x >> 2) & 0b1;
        let b = (x >> 3) & 0b111;
        let c = (x >> 6) & 0b1;
        let d = (x >> 7) & 0b1;
        let e = (x >> 8) & 0b1;
        let f = (x >> 9) & 0b11;
        let g = (x >> 11) & 0b1;
        let h = (x >> 12) & 0b1;


        let i: i32 = ((b << 1) | (g << 4) | (a << 5) | (d << 6) | (c << 7) | (f << 8) | (e << 10) | (h << 11)) as i32;
        let i2 = (i << 20) >> 20;
        CJImmediate { val: i2 }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(11) - 1 || val < -2i64.pow(11) {
            panic!("attempted to construct out of range CJImmediate")
        }
        CJImmediate { val: val as i32}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CJImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in a CFLDSP or CLDSP instruction
#[derive(Debug, PartialEq)]
pub struct CDSPImmediate {
    val: i32,
}

impl CDSPImmediate {
    /// Extracts the `CDSPImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = x >> 2 & 0b111;
        let b = x >> 5 & 0b11;
        let c = x >> 12 & 0b1;


        let i: i32 = ((b << 3) | (c << 5) | (a << 6)) as i32;
        CDSPImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(9) - 1 || val < 0 {
            panic!("attempted to construct out of range CDSPImmediate")
        }
        CDSPImmediate { val: val as i32}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CDSPImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in a CLWSP instruction
#[derive(Debug, PartialEq)]
pub struct CWSPImmediate {
    val: i32,
}

impl CWSPImmediate {
    /// Extracts the `CDSPImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = x >> 2 & 0b11;
        let b = x >> 4 & 0b111;
        let c = x >> 12 & 0b1;
        println!("a: {a}, b: {b}, c: {c}");

        let i: i32 = ((b << 2) | (c << 5) | (a << 6)) as i32;
        CWSPImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(9) - 1 || val < 0 {
            panic!("attempted to construct out of range CDSPImmediate")
        }
        CWSPImmediate { val: val as i32}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CWSPImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in a FSDSP or SDSP instruction
#[derive(Debug, PartialEq)]
pub struct CSDSPImmediate {
    val: i32,
}

impl CSDSPImmediate {
    /// Extracts the `CSDSPImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = x >> 7 & 0b111;
        let b = x >> 10 & 0b111;
        let i: i32 = ((b << 3) | (a << 6)) as i32;
        CSDSPImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(9) - 1 || val < 0 {
            panic!("attempted to construct out of range CDSPImmediate")
        }
        CSDSPImmediate { val: val as i32}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CSDSPImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in a SWSP instruction
#[derive(Debug, PartialEq)]
pub struct CSWSPImmediate {
    val: i32,
}

impl CSWSPImmediate {
    /// Extracts the `CSWSPImmediate` from the appropriate position in a 16-bit instruction
    pub fn from_u16(x: u16) -> Self {
        let a = x >> 7 & 0b11;
        let b = x >> 9 & 0b1111;

        let i: i32 = ((b << 2) | (a << 6)) as i32;
        CSWSPImmediate { val: i }
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(8) - 1 || val < 0 {
            panic!("attempted to construct out of range CSWSPImmediate")
        }
        CSWSPImmediate { val: val as i32}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CSWSPImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The index of a CSR
#[derive(Debug, PartialEq)]
pub struct CSR {
    val: u16,
}

impl CSR {
    /// Extracts the `CSR` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let i = (x >> 20) & 0b1111_1111_1111;
        CSR { val: i as u16}
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(12) - 1 || val < 0 {
            panic!("attempted to construct out of range CSR")
        }
        CSR { val: val as u16}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CSR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}

/// The immediate value in a CSR immediate instruction
#[derive(Debug, PartialEq)]
pub struct CSRImmediate {
    val: u16,
}

impl CSRImmediate {
    /// Extracts the `CSRImmediate` from the appropriate position in a 32-bit instruction
    pub fn from_u32(x: u32) -> Self {
        let i = (x >> 15) & 0b1_1111;
        CSRImmediate { val: i as u16}
    }

    pub fn from_val(val: i64) -> Self {
        if val > 2i64.pow(5) - 1 || val < 0 {
            panic!("attempted to construct out of range CSR")
        }
        CSRImmediate { val: val as u16}
    }

    pub fn val(&self) -> i64 {
        self.val.into()
    }
}

impl Display for CSRImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.val)
    }
}