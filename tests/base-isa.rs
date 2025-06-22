use riscv_disassembler::{immediates::{JImmediate, UImmediate}, instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction}, register::IRegister};

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

#[test]
fn add_upper_immediate_to_program_counter() {
    // check assembler
    let i = assemble_line("auipc a3,-1").unwrap();
    let expected = Instruction::AUIPC(IRegister::A3, UImmediate::from_val(-1));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xfffff697).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link() {
    // check assembler
    let i = assemble_line("jal zero,-1016708").unwrap();
    let expected = Instruction::JAL(IRegister::Zero, JImmediate::from_val(-1016708));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_instruction(0xc7d0706f).unwrap();
    assert_eq!(
        i2,
        expected,
    );

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}