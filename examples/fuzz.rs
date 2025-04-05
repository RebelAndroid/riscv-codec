use rand::{Rng, SeedableRng};

use riscv_disassembler::instruction::{assemble_line, decode_instruction, disassemble_instruction};

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let mut i = 0;
    // println!("size: {}", std::mem::size_of::<Instruction>());
    loop {
        let x: u32 = rng.random();
        let d = decode_instruction(x);
        if let Ok(instr) = d {
            let asm: String = disassemble_instruction(&instr);
            println!("{i} assembly: {asm}");
            let new_instr = assemble_line(&asm).unwrap();
            if instr != new_instr {
                println!(
                    "ERROR WITH INSTRUCTION DISASSEMBLING: original: {instr:?}, new: {new_instr:?}"
                );
                std::process::exit(1);
            }
            i += 1; // only counting valid instructions for assembly
        }
    }
}
