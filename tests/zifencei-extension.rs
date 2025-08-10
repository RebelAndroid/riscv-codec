use riscv_codec::{
    assembly::assemble_line,
    instruction::{Instruction, disassemble_instruction},
};

#[test]
fn fence_instruction() {
    let bin = 0x0000100f;
    let expected = Instruction::FenceI;

    // check assembler
    let i = assemble_line("fence.i").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected,);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
