use riscv_codec::instruction::{assemble_line, decode_instruction, encode_instruction, Instruction};
fn main() {
    // instruction can be assembled from strings
    let instr: Instruction = assemble_line("addi t0, t1, 1024").unwrap().i();
    // and disassembled
    println!("assembled instruction: {}", instr);

    // instructions can also be decoded from binary
    let instr2 = decode_instruction(0xe0058513).unwrap();

    // and encoded
    assert_eq!(encode_instruction(&instr2), 0xe0058513);
}
