use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::IRegister;

#[test]
fn load_reserved_word() {
    // check assembler
    let i = assemble_line("lr.w.aq a0,a1").unwrap();
    assert_eq!(
        i,
        Instruction::LRW(IRegister::A0, IRegister::A1, true, false)
    );

    // check decoder
    let i2 = decode_instruction(0x1405a52f).unwrap();
    assert_eq!(
        i2,
        Instruction::LRW(IRegister::A0, IRegister::A1, true, false)
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}
