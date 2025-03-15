use rand::{Rng, SeedableRng};
use std::fmt::{Display, Formatter};

mod register;
use crate::register::*;

mod instruction;
use crate::instruction::*;

mod opcode;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}", decode_instruction(0x04F7_1463).unwrap());
    // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    // let mut i = 0;
    // loop {
    //     let x: u32 = rng.random();
    //     let d = decode_instruction(x);
    //     if let Ok(instr) = d {
    //         println!("{}: {:x} {}", i, x, instr);
    //     }

    //     i += 1;
    // }
}
