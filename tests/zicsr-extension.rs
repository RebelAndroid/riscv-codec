use riscv_codec::{
    assembly::assemble_line, immediates::{CSRImmediate, CSR}, instruction::{disassemble_instruction, Instruction}, register::IRegister
};

#[test]
fn read_write_csr() {
    let expected = Instruction::CSRRW {
        dest: IRegister::T1,
        src: IRegister::T2,
        csr: CSR::try_from(100).unwrap(),
    };
    let bin = 0x06439373;

    // check assembler
    let i = assemble_line("csrrw t1, 100, t2").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_set_csr() {
    let expected = Instruction::CSRRS {
        dest: IRegister::ThreadPointer,
        src: IRegister::S6,
        csr: CSR::try_from(4000).unwrap(),
    };
    let bin = 0xfa0b2273;

    // check assembler
    let i = assemble_line("csrrs tp, 4000, s6").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_clear_csr() {
    let expected = Instruction::CSRRC {
        dest: IRegister::A0,
        src: IRegister::A5,
        csr: CSR::try_from(1).unwrap(),
    };
    let bin = 0x0017b573;

    // check assembler
    let i = assemble_line("csrrc a0, 1, a5").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_write_csr_immediate() {
    let expected = Instruction::CSRRWI {
        dest: IRegister::S11,
        imm: CSRImmediate::try_from(31).unwrap(),
        csr: CSR::try_from(100).unwrap(),
    };
    let bin = 0x064fddf3;

    // check assembler
    let i = assemble_line("csrrwi s11, 100, 31").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_set_csr_immediate() {
    let expected = Instruction::CSRRSI {
        dest: IRegister::S4,
        imm: CSRImmediate::try_from(1).unwrap(),
        csr: CSR::try_from(1001).unwrap(),
    };
    let bin = 0x3e90ea73;

    // check assembler
    let i = assemble_line("csrrsi s4, 1001, 1").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn read_clear_csr_immediate() {
    let expected = Instruction::CSRRCI {
        dest: IRegister::A1,
        imm: CSRImmediate::try_from(23).unwrap(),
        csr: CSR::try_from(24).unwrap(),
    };
    let bin = 0x018bf5f3;

    // check assembler
    let i = assemble_line("csrrci a1, 24, 23").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
