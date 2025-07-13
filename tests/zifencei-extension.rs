use riscv_disassembler::instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction};

#[test]
fn fence_instruction() {
    // check assembler
    let i = assemble_line("fence.i").unwrap().i();
    let expected = Instruction::FENCEI;
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x0000100f).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}