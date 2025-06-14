use riscv_disassembler::immediates::IImmediate;
use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::{IRegister, FRegister};

#[test]
fn float_load_word() {
    // check assembler
    let i = assemble_line("flw fa0,64(a0)").unwrap();
    assert_eq!(
        i,
        Instruction::FLW(FRegister::FA0, IRegister::A0, IImmediate::from_val(64))
    );

    // check decoder
    let i2 = decode_instruction(0x04052507).unwrap();
    assert_eq!(
        i2,
        Instruction::FLW(FRegister::FA0, IRegister::A0, IImmediate::from_val(64))
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}