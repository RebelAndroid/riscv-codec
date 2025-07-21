use riscv_codec::instruction::{assemble_line, decode_instruction, disassemble_instruction, encode_instruction, Instruction};

#[test]
fn fence_instruction() {
    let bin = 0x0000100f;
    let expected = Instruction::FENCEI;

    // check assembler
    let i = assemble_line("fence.i").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(bin).unwrap();
    assert_eq!(i2, expected,);

    // check encoder
    let b = encode_instruction(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}