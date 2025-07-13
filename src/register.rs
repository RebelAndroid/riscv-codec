use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IRegister {
    Zero = 0,
    ReturnAddress = 1,
    StackPointer = 2,
    GlobalPointer = 3,
    ThreadPointer = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    /// Also called s0
    FramePointer = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

impl Display for IRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                IRegister::Zero => "zero",
                IRegister::ReturnAddress => "ra",
                IRegister::StackPointer => "sp",
                IRegister::GlobalPointer => "gp",
                IRegister::ThreadPointer => "tp",
                IRegister::T0 => "t0",
                IRegister::T1 => "t1",
                IRegister::T2 => "t2",
                IRegister::FramePointer => "s0",
                IRegister::S1 => "s1",
                IRegister::A0 => "a0",
                IRegister::A1 => "a1",
                IRegister::A2 => "a2",
                IRegister::A3 => "a3",
                IRegister::A4 => "a4",
                IRegister::A5 => "a5",
                IRegister::A6 => "a6",
                IRegister::A7 => "a7",
                IRegister::S2 => "s2",
                IRegister::S3 => "s3",
                IRegister::S4 => "s4",
                IRegister::S5 => "s5",
                IRegister::S6 => "s6",
                IRegister::S7 => "s7",
                IRegister::S8 => "s8",
                IRegister::S9 => "s9",
                IRegister::S10 => "s10",
                IRegister::S11 => "s11",
                IRegister::T3 => "t3",
                IRegister::T4 => "t4",
                IRegister::T5 => "t5",
                IRegister::T6 => "t6",
            }
        )
    }
}

impl IRegister {
    pub fn from_int(int: u32) -> Self {
        match int {
            0 => Self::Zero,
            1 => Self::ReturnAddress,
            2 => Self::StackPointer,
            3 => Self::GlobalPointer,
            4 => Self::ThreadPointer,
            5 => Self::T0,
            6 => Self::T1,
            7 => Self::T2,
            8 => Self::FramePointer,
            9 => Self::S1,
            10 => Self::A0,
            11 => Self::A1,
            12 => Self::A2,
            13 => Self::A3,
            14 => Self::A4,
            15 => Self::A5,
            16 => Self::A6,
            17 => Self::A7,
            18 => Self::S2,
            19 => Self::S3,
            20 => Self::S4,
            21 => Self::S5,
            22 => Self::S6,
            23 => Self::S7,
            24 => Self::S8,
            25 => Self::S9,
            26 => Self::S10,
            27 => Self::S11,
            28 => Self::T3,
            29 => Self::T4,
            30 => Self::T5,
            31 => Self::T6,
            x => panic!("converted invalid to integer register {}", x),
        }
    }

    pub fn from_string(str: &str) -> Result<Self, String> {
        match str {
            "zero" => Ok(Self::Zero),
            "ra" => Ok(Self::ReturnAddress),
            "sp" => Ok(Self::StackPointer),
            "gp" => Ok(Self::GlobalPointer),
            "tp" => Ok(Self::ThreadPointer),
            "t0" => Ok(Self::T0),
            "t1" => Ok(Self::T1),
            "t2" => Ok(Self::T2),
            "fp" => Ok(Self::FramePointer),
            "s0" => Ok(Self::FramePointer),
            "s1" => Ok(Self::S1),
            "a0" => Ok(Self::A0),
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            "a6" => Ok(Self::A6),
            "a7" => Ok(Self::A7),
            "s2" => Ok(Self::S2),
            "s3" => Ok(Self::S3),
            "s4" => Ok(Self::S4),
            "s5" => Ok(Self::S5),
            "s6" => Ok(Self::S6),
            "s7" => Ok(Self::S7),
            "s8" => Ok(Self::S8),
            "s9" => Ok(Self::S9),
            "s10" => Ok(Self::S10),
            "s11" => Ok(Self::S11),
            "t3" => Ok(Self::T3),
            "t4" => Ok(Self::T4),
            "t5" => Ok(Self::T5),
            "t6" => Ok(Self::T6),
            x => Err(format!("converted invalid str to integer register {}", x)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FRegister {
    FT0 = 0,
    FT1 = 1,
    FT2 = 2,
    FT3 = 3,
    FT4 = 4,
    FT5 = 5,
    FT6 = 6,
    FT7 = 7,
    FS0 = 8,
    FS1 = 9,
    FA0 = 10,
    FA1 = 11,
    FA2 = 12,
    FA3 = 13,
    FA4 = 14,
    FA5 = 15,
    FA6 = 16,
    FA7 = 17,
    FS2 = 18,
    FS3 = 19,
    FS4 = 20,
    FS5 = 21,
    FS6 = 22,
    FS7 = 23,
    FS8 = 24,
    FS9 = 25,
    FS10 = 26,
    FS11 = 27,
    FT8 = 28,
    FT9 = 29,
    FT10 = 30,
    FT11 = 31,
}

impl Display for FRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                FRegister::FT0 => "ft0",
                FRegister::FT1 => "ft1",
                FRegister::FT2 => "ft2",
                FRegister::FT3 => "ft3",
                FRegister::FT4 => "ft4",
                FRegister::FT5 => "ft5",
                FRegister::FT6 => "ft6",
                FRegister::FT7 => "ft7",
                FRegister::FS0 => "fs0",
                FRegister::FS1 => "fs1",
                FRegister::FA0 => "fa0",
                FRegister::FA1 => "fa1",
                FRegister::FA2 => "fa2",
                FRegister::FA3 => "fa3",
                FRegister::FA4 => "fa4",
                FRegister::FA5 => "fa5",
                FRegister::FA6 => "fa6",
                FRegister::FA7 => "fa7",
                FRegister::FS2 => "fs2",
                FRegister::FS3 => "fs3",
                FRegister::FS4 => "fs4",
                FRegister::FS5 => "fs5",
                FRegister::FS6 => "fs6",
                FRegister::FS7 => "fs7",
                FRegister::FS8 => "fs8",
                FRegister::FS9 => "fs9",
                FRegister::FS10 => "fs10",
                FRegister::FS11 => "fs11",
                FRegister::FT8 => "ft8",
                FRegister::FT9 => "ft9",
                FRegister::FT10 => "ft10",
                FRegister::FT11 => "ft11",
            }
        )
    }
}

impl FRegister {
    pub fn from_int(int: u32) -> Self {
        match int {
            0 => Self::FT0,
            1 => Self::FT1,
            2 => Self::FT2,
            3 => Self::FT3,
            4 => Self::FT4,
            5 => Self::FT5,
            6 => Self::FT6,
            7 => Self::FT7,
            8 => Self::FS0,
            9 => Self::FS1,
            10 => Self::FA0,
            11 => Self::FA1,
            12 => Self::FA2,
            13 => Self::FA3,
            14 => Self::FA4,
            15 => Self::FA5,
            16 => Self::FA6,
            17 => Self::FA7,
            18 => Self::FS2,
            19 => Self::FS3,
            20 => Self::FS4,
            21 => Self::FS5,
            22 => Self::FS6,
            23 => Self::FS7,
            24 => Self::FS8,
            25 => Self::FS9,
            26 => Self::FS10,
            27 => Self::FS11,
            28 => Self::FT8,
            29 => Self::FT9,
            30 => Self::FT10,
            31 => Self::FT11,
            x => panic!("converted invalid integer to float register {}", x),
        }
    }

    pub fn from_string(str: &str) -> Result<Self, String> {
        match str {
            "ft0" => Ok(Self::FT0),
            "ft1" => Ok(Self::FT1),
            "ft2" => Ok(Self::FT2),
            "ft3" => Ok(Self::FT3),
            "ft4" => Ok(Self::FT4),
            "ft5" => Ok(Self::FT5),
            "ft6" => Ok(Self::FT6),
            "ft7" => Ok(Self::FT7),
            "fs0" => Ok(Self::FS0),
            "fs1" => Ok(Self::FS1),
            "fa0" => Ok(Self::FA0),
            "fa1" => Ok(Self::FA1),
            "fa2" => Ok(Self::FA2),
            "fa3" => Ok(Self::FA3),
            "fa4" => Ok(Self::FA4),
            "fa5" => Ok(Self::FA5),
            "fa6" => Ok(Self::FA6),
            "fa7" => Ok(Self::FA7),
            "fs2" => Ok(Self::FS2),
            "fs3" => Ok(Self::FS3),
            "fs4" => Ok(Self::FS4),
            "fs5" => Ok(Self::FS5),
            "fs6" => Ok(Self::FS6),
            "fs7" => Ok(Self::FS7),
            "fs8" => Ok(Self::FS8),
            "fs9" => Ok(Self::FS9),
            "fs10" => Ok(Self::FS10),
            "fs11" => Ok(Self::FS11),
            "ft8" => Ok(Self::FT8),
            "ft9" => Ok(Self::FT9),
            "ft10" => Ok(Self::FT10),
            "ft11" => Ok(Self::FT11),
            x => Err(format!("converted invalid str to float register {}", x)),
        }
    }
}

/// One of the limited set of registers available in compressed instructions
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CIRegister {
    FramePointer,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
}

impl CIRegister {
    pub fn from_int(int: u16) -> Self {
        match int {
            0 => Self::FramePointer,
            1 => Self::S1,
            2 => Self::A0,
            3 => Self::A1,
            4 => Self::A2,
            5 => Self::A3,
            6 => Self::A4,
            7 => Self::A5,
            x => panic!(
                "converted invalid integer to register in compressed instruction: {}",
                x
            ),
        }
    }

    pub fn from_string(str: &str) -> Result<Self, String> {
        match str {
            "fp" => Ok(Self::FramePointer),
            "s0" => Ok(Self::FramePointer),
            "s1" => Ok(Self::S1),
            "a0" => Ok(Self::A0),
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            x => Err(format!(
                "converted invalid str to integer register in compressed instruction: {}",
                x
            )),
        }
    }

    pub fn expand(&self) -> IRegister {
        match self {
            CIRegister::FramePointer => IRegister::FramePointer,
            CIRegister::S1 => IRegister::S1,
            CIRegister::A0 => IRegister::A0,
            CIRegister::A1 => IRegister::A1,
            CIRegister::A2 => IRegister::A2,
            CIRegister::A3 => IRegister::A3,
            CIRegister::A4 => IRegister::A4,
            CIRegister::A5 => IRegister::A5,
        }
    }
}

impl Display for CIRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                CIRegister::FramePointer => "s0",
                CIRegister::S1 => "s1",
                CIRegister::A0 => "a0",
                CIRegister::A1 => "a1",
                CIRegister::A2 => "a2",
                CIRegister::A3 => "a3",
                CIRegister::A4 => "a4",
                CIRegister::A5 => "a5",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CFRegister {
    FS0,
    FS1,
    FA0,
    FA1,
    FA2,
    FA3,
    FA4,
    FA5,
}

impl CFRegister {
    pub fn from_int(int: u16) -> Self {
        match int {
            0 => Self::FS0,
            1 => Self::FS1,
            2 => Self::FA0,
            3 => Self::FA1,
            4 => Self::FA2,
            5 => Self::FA3,
            6 => Self::FA4,
            7 => Self::FA5,
            x => panic!(
                "converted invalid integer to float register in compressed instruction: {}",
                x
            ),
        }
    }

    pub fn from_string(str: &str) -> Result<Self, String> {
        match str {
            "fs0" => Ok(Self::FS0),
            "fs1" => Ok(Self::FS1),
            "fa0" => Ok(Self::FA0),
            "fa1" => Ok(Self::FA1),
            "fa2" => Ok(Self::FA2),
            "fa3" => Ok(Self::FA3),
            "fa4" => Ok(Self::FA4),
            "fa5" => Ok(Self::FA5),
            x => Err(format!(
                "converted invalid str to float register in compressed instruction {}",
                x
            )),
        }
    }
    
    pub fn expand(&self) -> FRegister {
        match self {
            CFRegister::FS0 => FRegister::FS0,
            CFRegister::FS1 => FRegister::FS1,
            CFRegister::FA0 => FRegister::FA0,
            CFRegister::FA1 => FRegister::FA1,
            CFRegister::FA2 => FRegister::FA2,
            CFRegister::FA3 => FRegister::FA3,
            CFRegister::FA4 => FRegister::FA4,
            CFRegister::FA5 => FRegister::FA5,
        }
    }
}

impl Display for CFRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                CFRegister::FS0 => "fs0",
                CFRegister::FS1 => "fs1",
                CFRegister::FA0 => "fa0",
                CFRegister::FA1 => "fa1",
                CFRegister::FA2 => "fa2",
                CFRegister::FA3 => "fa3",
                CFRegister::FA4 => "fa4",
                CFRegister::FA5 => "fa5",
            }
        )
    }
}
