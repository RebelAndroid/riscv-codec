use riscv_disassembler::{immediates::UImmediate, instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction}, register::IRegister};

#[test]
fn load_upper_immediate() {
    // check assembler
    let i = assemble_line("lui s2,400").unwrap();
    let expected = Instruction::LUI(IRegister::S2, UImmediate::from_val(400));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0x00190937).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}