use riscv_disassembler::{immediates::{CSRImmediate, CSR}, instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction}, register::IRegister};

#[test]
fn read_write_csr() {
    // check assembler
    let i = assemble_line("csrrw t1, 100, t2").unwrap().i();
    let expected = Instruction::CSRRW{dest: IRegister::T1, src: IRegister::T2, csr: CSR::try_from(100).unwrap()};
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
    let expected = Instruction::CSRRS{dest: IRegister::ThreadPointer, src: IRegister::S6, csr: CSR::try_from(4000).unwrap()};
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
    let expected = Instruction::CSRRC{dest: IRegister::A0, src: IRegister::A5, csr: CSR::try_from(1).unwrap()};
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
    let expected = Instruction::CSRRWI{dest: IRegister::S11, imm: CSRImmediate::try_from(31).unwrap(), csr: CSR::try_from(100).unwrap()};
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
    let expected = Instruction::CSRRSI{dest: IRegister::S4, imm: CSRImmediate::try_from(1).unwrap(), csr: CSR::try_from(1001).unwrap()};
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
    let expected = Instruction::CSRRCI{dest: IRegister::A1, imm: CSRImmediate::try_from(23).unwrap(), csr: CSR::try_from(24).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x018bf5f3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}