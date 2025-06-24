use riscv_disassembler::immediates::{CDImmediate, CWImmediate, CWideImmediate};
use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_compressed_instruction, disassemble_instruction,
};
use riscv_disassembler::register::{FRegister, IRegister};

#[test]
fn add_4_immediate_stack_pointer() {
    // check assembler
    let i = assemble_line("c.addi4spn a0,280").unwrap();
    let expected = Instruction::CADDI4SPN(IRegister::A0, CWideImmediate::from_val(280));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0x0a28).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_load_double() {
    // check assembler
    let i = assemble_line("c.fld fa0,152(a1)").unwrap();
    let expected = Instruction::CFLD(FRegister::FA0, IRegister::A1, CDImmediate::from_val(152));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0x2dc8).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_word() {
    // check assembler
    let i = assemble_line("c.lw a2,0(fp)").unwrap();
    let expected = Instruction::CLW(IRegister::A2, IRegister::FramePointer, CWImmediate::from_val(0));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0x4010).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_doubleword() {
    // check assembler
    let i = assemble_line("c.ld a3,248(a4)").unwrap();
    let expected = Instruction::CLD(IRegister::A3, IRegister::A4, CDImmediate::from_val(248));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0x7f74).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn float_store_double() {
    // check assembler
    let i = assemble_line("c.fsd fs0,8(a5)").unwrap();
    let expected = Instruction::CFSD(FRegister::FS0, IRegister::A5, CDImmediate::from_val(8));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0xa780).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn store_word() {
    // check assembler
    let i = assemble_line("c.sw a2,124(a2)").unwrap();
    let expected = Instruction::CSW(IRegister::A2, IRegister::A2, CWImmediate::from_val(124));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0xde70).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn store_double() {
    // check assembler
    let i = assemble_line("c.sd a4,248(a3)").unwrap();
    let expected = Instruction::CSD(IRegister::A4, IRegister::A3, CDImmediate::from_val(248));
    assert_eq!(
        i,
        expected
    );

    // check decoder
    let i2 = decode_compressed_instruction(0xfef8).unwrap();
    assert_eq!(
        i2,
        expected
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}