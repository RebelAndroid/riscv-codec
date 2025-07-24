use riscv_codec::{assembly::assemble_line, instruction::Instruction};
fn main() {
    // instruction can be assembled from strings
    let instr: Instruction = assemble_line("addi t0, t1, 1024").unwrap().i();
    // and disassembled
    println!("assembled instruction: {}", instr);

    // instructions can also be decoded from binary
    let instr2 = Instruction::decode(0xe0058513).unwrap();

    // and encoded
    assert_eq!(Instruction::encode(&instr2), 0xe0058513);
}
