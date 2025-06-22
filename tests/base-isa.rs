use riscv_disassembler::{
    immediates::{BImmediate, IImmediate, JImmediate, UImmediate},
    instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction},
    register::IRegister,
};

#[test]
fn load_upper_immediate() {
    // check assembler
    let i = assemble_line("lui s2,400").unwrap();
    let expected = Instruction::LUI(IRegister::S2, UImmediate::from_val(400));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x00190937).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn add_upper_immediate_to_program_counter() {
    // check assembler
    let i = assemble_line("auipc a3,-1").unwrap();
    let expected = Instruction::AUIPC(IRegister::A3, UImmediate::from_val(-1));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xfffff697).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link() {
    // check assembler
    let i = assemble_line("jal zero,-1016708").unwrap();
    let expected = Instruction::JAL(IRegister::Zero, JImmediate::from_val(-1016708));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc7d0706f).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link_register() {
    // check assembler
    let i = assemble_line("jalr a0,-2048(t0)").unwrap();
    let expected = Instruction::JALR(IRegister::A0, IRegister::T0, IImmediate::from_val(-2048));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x80028567).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn branch_equal() {
    // check assembler
    let i = assemble_line("beq t2,sp,2").unwrap();
    let expected = Instruction::BEQ(IRegister::T2, IRegister::StackPointer, BImmediate::from_val(2));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x00238163).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn branch_not_equal() {
    // check assembler
    let i = assemble_line("bne tp,a4,4094").unwrap();
    let expected = Instruction::BNE(IRegister::ThreadPointer, IRegister::A4, BImmediate::from_val(4094));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x7ee21fe3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn branch_less_than() {
    // check assembler
    let i = assemble_line("blt a1,t6,-4096").unwrap();
    let expected = Instruction::BLT(IRegister::A1, IRegister::T6, BImmediate::from_val(-4096));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x81f5c063).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn branch_greater_equal() {
    // check assembler
    let i = assemble_line("bge a1,t6,-2030").unwrap();
    let expected = Instruction::BGE(IRegister::A1, IRegister::T6, BImmediate::from_val(-2030));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x81f5d9e3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}
