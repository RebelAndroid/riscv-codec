use std::process::exit;

use riscv_codec::{assembly::assemble_line, instruction::Instruction,};
fn main() {
    let mut valid: u32 = 0;
    for x in 0..u32::MAX {
        if x % 10000 == 0 {
            println!("{:.4}%", 100.0 * (x as f64) / (u32::MAX as f64))
        }
        if let Ok(i) = Instruction::decode(x) {
            valid += 1;
            let e = Instruction::encode(&i);
            if x != e {
                println!("i: {i}\nx: {x:032b}\ne: {e:032b}");
                exit(1);
            }

            let d = i.to_string();
            let i2 = assemble_line(&d).unwrap().i();
            if i != i2 {
                println!("disassembled {i:#?} to get {d}. Assembled to get {i2:#?}!");
                exit(1);
            }
        }
    }
    println!("Done.");
    println!("proportion of encoding space used: {}%", 100.0 * (valid as f64) / (u32::MAX as f64))
}