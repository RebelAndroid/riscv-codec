use riscv_disassembler::{immediates::{CSRImmediate, CSR}, instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction}, register::IRegister};

#[test]
fn read_write_csr() {
    // check assembler
    let i = assemble_line("csrrw t1, 100, t2").unwrap().i();
    let expected = Instruction::CSRRW(IRegister::T1, IRegister::T2, CSR::from_val(100));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x06439373).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_set_csr() {
    // check assembler
    let i = assemble_line("csrrs tp, 4000, s6").unwrap().i();
    let expected = Instruction::CSRRS(IRegister::ThreadPointer, IRegister::S6, CSR::from_val(4000));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xfa0b2273).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_clear_csr() {
    // check assembler
    let i = assemble_line("csrrc a0, 1, a5").unwrap().i();
    let expected = Instruction::CSRRC(IRegister::A0, IRegister::A5, CSR::from_val(1));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x0017b573).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_write_csr_immediate() {
    // check assembler
    let i = assemble_line("csrrwi s11, 100, 31").unwrap().i();
    let expected = Instruction::CSRRWI(IRegister::S11, CSRImmediate::from_val(31), CSR::from_val(100));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x064fddf3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_set_csr_immediate() {
    // check assembler
    let i = assemble_line("csrrsi s4, 1001, 1").unwrap().i();
    let expected = Instruction::CSRRSI(IRegister::S4, CSRImmediate::from_val(1), CSR::from_val(1001));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x3e90ea73).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_clear_csr_immediate() {
    // check assembler
    let i = assemble_line("csrrci a1, 24, 23").unwrap().i();
    let expected = Instruction::CSRRCI(IRegister::A1, CSRImmediate::from_val(23), CSR::from_val(24));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x018bf5f3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}