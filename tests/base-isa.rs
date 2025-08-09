use riscv_codec::{
    assembly::assemble_line,
    immediates::{BImmediate, IImmediate, JImmediate, SImmediate, Shamt, ShamtW, UImmediate},
    instruction::{Instruction, disassemble_instruction},
    register::IRegister,
};

#[test]
fn load_upper_immediate() {
    let expected = Instruction::Lui {
        dest: IRegister::S2,
        imm: UImmediate::try_from(400).unwrap(),
    };
    let bin = 0x00190937;
    // check assembler
    let i = assemble_line("lui s2,400").unwrap().i();
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
fn add_upper_immediate_to_program_counter() {
    let expected = Instruction::Auipc {
        dest: IRegister::A3,
        imm: UImmediate::try_from(-1).unwrap(),
    };
    let bin = 0xfffff697;

    // check assembler
    let i = assemble_line("auipc a3,-1").unwrap().i();
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
fn jump_and_link() {
    let expected = Instruction::Jal {
        dest: IRegister::Zero,
        offset: JImmediate::try_from(-1016708).unwrap(),
    };
    let bin = 0xc7d0706f;

    // check assembler
    let i = assemble_line("jal zero,-1016708").unwrap().i();
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
fn jump_and_link_register() {
    let expected = Instruction::Jalr {
        dest: IRegister::A0,
        base: IRegister::T0,
        offset: IImmediate::try_from(-2048).unwrap(),
    };
    let bin = 0x80028567;

    // check assembler
    let i = assemble_line("jalr a0,-2048(t0)").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin, "got: {:b} expected: {:b}", b, bin);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_equal() {
    let expected = Instruction::Beq {
        src1: IRegister::T2,
        src2: IRegister::StackPointer,
        offset: BImmediate::try_from(2).unwrap(),
    };
    let bin = 0x00238163;

    // check assembler
    let i = assemble_line("beq t2,sp,2").unwrap().i();
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
fn branch_not_equal() {
    let expected = Instruction::Bne {
        src1: IRegister::ThreadPointer,
        src2: IRegister::A4,
        offset: BImmediate::try_from(4094).unwrap(),
    };
    let bin = 0x7ee21fe3;

    // check assembler
    let i = assemble_line("bne tp,a4,4094").unwrap().i();
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
fn branch_less_than() {
    let expected = Instruction::Blt {
        src1: IRegister::A1,
        src2: IRegister::T6,
        offset: BImmediate::try_from(-4096).unwrap(),
    };
    let bin = 0x81f5c063;

    // check assembler
    let i = assemble_line("blt a1,t6,-4096").unwrap().i();
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
fn branch_greater_equal() {
    let expected = Instruction::Bge {
        src1: IRegister::A1,
        src2: IRegister::T6,
        offset: BImmediate::try_from(-2030).unwrap(),
    };
    let bin = 0x81f5d9e3;

    // check assembler
    let i = assemble_line("bge a1,t6,-2030").unwrap().i();
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
fn branch_less_than_unsigned() {
    let expected = Instruction::Bltu {
        src1: IRegister::T0,
        src2: IRegister::S2,
        offset: BImmediate::try_from(512).unwrap(),
    };
    let bin = 0x2122e063;

    // check assembler
    let i = assemble_line("bltu t0,s2,512").unwrap().i();
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
fn branch_greater_equal_unsigned() {
    let expected = Instruction::Bgeu {
        src1: IRegister::S1,
        src2: IRegister::A3,
        offset: BImmediate::try_from(-128).unwrap(),
    };
    let bin = 0xf8d4f0e3;

    // check assembler
    let i = assemble_line("bgeu s1,a3,-128").unwrap().i();
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
fn load_byte() {
    let expected = Instruction::Lb {
        dest: IRegister::T2,
        base: IRegister::A0,
        offset: IImmediate::try_from(8).unwrap(),
    };
    let bin = 0x00850383;

    // check assembler
    let i = assemble_line("lb t2,8(a0)").unwrap().i();
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
fn load_half() {
    let expected = Instruction::Lh {
        dest: IRegister::S3,
        base: IRegister::StackPointer,
        offset: IImmediate::try_from(-16).unwrap(),
    };
    let bin = 0xff011983;

    // check assembler
    let i = assemble_line("lh s3,-16(sp)").unwrap().i();
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
fn load_word() {
    let expected = Instruction::Lw {
        dest: IRegister::A4,
        base: IRegister::T5,
        offset: IImmediate::try_from(1024).unwrap(),
    };
    let bin = 0x400f2703;

    // check assembler
    let i = assemble_line("lw a4,1024(t5)").unwrap().i();
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
fn load_byte_unsigned() {
    let expected = Instruction::Lbu {
        dest: IRegister::FramePointer,
        base: IRegister::A6,
        offset: IImmediate::try_from(63).unwrap(),
    };
    let bin = 0x03f84403;

    // check assembler
    let i = assemble_line("lbu s0,63(a6)").unwrap().i();
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
fn load_half_unsigned() {
    let expected = Instruction::Lhu {
        dest: IRegister::T4,
        base: IRegister::A1,
        offset: IImmediate::try_from(-2047).unwrap(),
    };
    let bin = 0x8015de83;

    // check assembler
    let i = assemble_line("lhu t4,-2047(a1)").unwrap().i();
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
fn store_byte() {
    let expected = Instruction::Sb {
        base: IRegister::A2,
        src: IRegister::T1,
        offset: SImmediate::try_from(127).unwrap(),
    };
    let bin = 0x06660fa3;

    // check assembler
    let i = assemble_line("sb t1,127(a2)").unwrap().i();
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
fn store_half() {
    let expected = Instruction::Sh {
        base: IRegister::S2,
        src: IRegister::A5,
        offset: SImmediate::try_from(-32).unwrap(),
    };
    let bin = 0xfef91023;

    // check assembler
    let i = assemble_line("sh a5,-32(s2)").unwrap().i();
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
fn store_word() {
    let expected = Instruction::Sw {
        base: IRegister::T6,
        src: IRegister::S7,
        offset: SImmediate::try_from(2046).unwrap(),
    };
    let bin = 0x7f7faf23;

    // check assembler
    let i = assemble_line("sw s7,2046(t6)").unwrap().i();
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
fn add_immediate() {
    let expected = Instruction::Addi {
        dest: IRegister::T3,
        src: IRegister::S4,
        imm: IImmediate::try_from(99).unwrap(),
    };
    let bin = 0x063a0e13;

    // check assembler
    let i = assemble_line("addi t3,s4,99").unwrap().i();
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
fn set_less_than_immediate() {
    let expected = Instruction::Slti {
        dest: IRegister::A1,
        src: IRegister::T2,
        imm: IImmediate::try_from(-12).unwrap(),
    };
    let bin = 0xff43a593;

    // check assembler
    let i = assemble_line("slti a1,t2,-12").unwrap().i();
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
fn set_less_than_immediate_unsigned() {
    let expected = Instruction::Sltiu {
        dest: IRegister::S5,
        src: IRegister::A0,
        imm: IImmediate::try_from(2047).unwrap(),
    };
    let bin = 0x7ff53a93;

    // check assembler
    let i = assemble_line("sltiu s5,a0,2047").unwrap().i();
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
fn xor_immediate() {
    let expected = Instruction::Xori {
        dest: IRegister::A7,
        src: IRegister::ReturnAddress,
        imm: IImmediate::try_from(15).unwrap(),
    };
    let bin = 0x00f0c893;

    // check assembler
    let i = assemble_line("xori a7,ra,15").unwrap().i();
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
fn or_immediate() {
    let expected = Instruction::Ori {
        dest: IRegister::T6,
        src: IRegister::GlobalPointer,
        imm: IImmediate::try_from(31).unwrap(),
    };
    let bin = 0x01f1ef93;

    // check assembler
    let i = assemble_line("ori t6,gp,31").unwrap().i();
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
fn and_immediate() {
    let expected = Instruction::Andi {
        dest: IRegister::GlobalPointer,
        src: IRegister::StackPointer,
        imm: IImmediate::try_from(-256).unwrap(),
    };
    let bin = 0xf0017193;

    // check assembler
    let i = assemble_line("andi gp,sp,-256").unwrap().i();
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
fn shift_left_logical_immediate() {
    let expected = Instruction::Slli {
        dest: IRegister::T1,
        src: IRegister::FramePointer,
        shamt: Shamt::try_from(13).unwrap(),
    };
    let bin = 0x00d41313;

    // check assembler
    let i = assemble_line("slli t1,s0,13").unwrap().i();
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
fn shift_right_logical_immediate() {
    let expected = Instruction::Srli {
        dest: IRegister::S2,
        src: IRegister::A6,
        shamt: Shamt::try_from(9).unwrap(),
    };
    let bin = 0x00985913;

    // check assembler
    let i = assemble_line("srli s2,a6,9").unwrap().i();
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
fn shift_right_arithmetic_immediate() {
    let expected = Instruction::Srai {
        dest: IRegister::S1,
        src: IRegister::S3,
        shamt: Shamt::try_from(17).unwrap(),
    };
    let bin = 0x4119d493;

    // check assembler
    let i = assemble_line("srai s1,s3,17").unwrap().i();
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
fn add() {
    let expected = Instruction::Add {
        dest: IRegister::T0,
        src1: IRegister::T1,
        src2: IRegister::T2,
    };
    let bin = 0x007302b3;

    // check assembler
    let i = assemble_line("add t0,t1,t2").unwrap().i();
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
fn sub() {
    let expected = Instruction::Sub {
        dest: IRegister::S4,
        src1: IRegister::S5,
        src2: IRegister::S6,
    };
    let bin = 0x416a8a33;

    // check assembler
    let i = assemble_line("sub s4,s5,s6").unwrap().i();
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
fn shift_left_logical() {
    let expected = Instruction::Sll {
        dest: IRegister::A2,
        src1: IRegister::A3,
        src2: IRegister::A4,
    };
    let bin = 0x00e69633;

    // check assembler
    let i = assemble_line("sll a2,a3,a4").unwrap().i();
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
fn set_less_than() {
    let expected = Instruction::Slt {
        dest: IRegister::T3,
        src1: IRegister::T4,
        src2: IRegister::T5,
    };
    let bin = 0x01eeae33;

    // check assembler
    let i = assemble_line("slt t3,t4,t5").unwrap().i();
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
fn set_less_than_unsigned() {
    let expected = Instruction::Sltu {
        dest: IRegister::S6,
        src1: IRegister::S7,
        src2: IRegister::Zero,
    };
    let bin = 0x000bbb33;

    // check assembler
    let i = assemble_line("sltu s6,s7,zero").unwrap().i();
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
fn xor() {
    let expected = Instruction::Xor {
        dest: IRegister::A5,
        src1: IRegister::A6,
        src2: IRegister::A7,
    };
    let bin = 0x011847b3;

    // check assembler
    let i = assemble_line("xor a5,a6,a7").unwrap().i();
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
fn shift_right_logical() {
    let expected = Instruction::Srl {
        dest: IRegister::T1,
        src1: IRegister::T2,
        src2: IRegister::T3,
    };
    let bin = 0x01c3d333;

    // check assembler
    let i = assemble_line("srl t1,t2,t3").unwrap().i();
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
fn shift_right_arithmetic() {
    let expected = Instruction::Sra {
        dest: IRegister::FramePointer,
        src1: IRegister::S1,
        src2: IRegister::S2,
    };
    let bin = 0x4124d433;

    // check assembler
    let i = assemble_line("sra s0,s1,s2").unwrap().i();
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
fn or() {
    let expected = Instruction::Or {
        dest: IRegister::T4,
        src1: IRegister::T5,
        src2: IRegister::T6,
    };
    let bin = 0x01ff6eb3;

    // check assembler
    let i = assemble_line("or t4,t5,t6").unwrap().i();
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
fn and() {
    let expected = Instruction::And {
        dest: IRegister::S1,
        src1: IRegister::S2,
        src2: IRegister::S3,
    };
    let bin = 0x013974b3;

    // check assembler
    let i = assemble_line("and s1,s2,s3").unwrap().i();
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
fn load_word_unsigned() {
    let expected = Instruction::Lwu {
        dest: IRegister::A0,
        base: IRegister::A1,
        offset: IImmediate::try_from(100).unwrap(),
    };
    let bin = 0x0645e503;

    // check assembler
    let i = assemble_line("lwu a0,100(a1)").unwrap().i();
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
fn load_doubleword() {
    let expected = Instruction::Ld {
        dest: IRegister::A2,
        base: IRegister::A3,
        offset: IImmediate::try_from(200).unwrap(),
    };
    let bin = 0x0c86b603;

    // check assembler
    let i = assemble_line("ld a2,200(a3)").unwrap().i();
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
fn store_doubleword() {
    let expected = Instruction::Sd {
        base: IRegister::A4,
        src: IRegister::A5,
        offset: SImmediate::try_from(300).unwrap(),
    };
    let bin = 0x12f73623;

    // check assembler
    let i = assemble_line("sd a5,300(a4)").unwrap().i();
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
fn add_immediate_word() {
    let expected = Instruction::Addiw {
        dest: IRegister::A6,
        src: IRegister::A7,
        imm: IImmediate::try_from(123).unwrap(),
    };
    let bin = 0x07b8881b;

    // check assembler
    let i = assemble_line("addiw a6,a7,123").unwrap().i();
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
fn shift_left_logical_immediate_word() {
    let expected = Instruction::Slliw {
        dest: IRegister::FramePointer,
        src: IRegister::S1,
        shamt: ShamtW::try_from(5).unwrap(),
    };
    let bin = 0x0054941b;

    // check assembler
    let i = assemble_line("slliw fp,s1,5").unwrap().i();
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
fn shift_right_logical_immediate_word() {
    let expected = Instruction::Srliw {
        dest: IRegister::S2,
        src: IRegister::S3,
        shamt: ShamtW::try_from(10).unwrap(),
    };
    let bin = 0x00a9d91b;

    // check assembler
    let i = assemble_line("srliw s2,s3,10").unwrap().i();
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
fn shift_right_arithmetic_immediate_word() {
    let expected = Instruction::Sraiw {
        dest: IRegister::S4,
        src: IRegister::S5,
        shamt: ShamtW::try_from(15).unwrap(),
    };
    let bin = 0x40fada1b;

    // check assembler
    let i = assemble_line("sraiw s4,s5,15").unwrap().i();
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
fn add_word() {
    let expected = Instruction::Addw {
        dest: IRegister::S6,
        src1: IRegister::S7,
        src2: IRegister::T0,
    };
    let bin = 0x005b8b3b;

    // check assembler
    let i = assemble_line("addw s6,s7,t0").unwrap().i();
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
fn subtract_word() {
    let expected = Instruction::Subw {
        dest: IRegister::T1,
        src1: IRegister::T2,
        src2: IRegister::T3,
    };
    let bin = 0x41c3833b;

    // check assembler
    let i = assemble_line("subw t1,t2,t3").unwrap().i();
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
fn shift_left_logical_word() {
    let expected = Instruction::Sllw {
        dest: IRegister::T4,
        src1: IRegister::T5,
        src2: IRegister::T6,
    };
    let bin = 0x01ff1ebb;

    // check assembler
    let i = assemble_line("sllw t4,t5,t6").unwrap().i();
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
fn shift_right_logical_word() {
    let expected = Instruction::Srlw {
        dest: IRegister::A0,
        src1: IRegister::A1,
        src2: IRegister::A2,
    };
    let bin = 0x00c5d53b;

    // check assembler
    let i = assemble_line("srlw a0,a1,a2").unwrap().i();
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
fn shift_right_arithmetic_word() {
    let expected = Instruction::Sraw {
        dest: IRegister::A3,
        src1: IRegister::A4,
        src2: IRegister::A5,
    };
    let bin = 0x40f756bb;

    // check assembler
    let i = assemble_line("sraw a3,a4,a5").unwrap().i();
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
fn fence() {
    let expected = Instruction::Fence {
        rd: IRegister::Zero,
        rs1: IRegister::Zero,
        ops: 0b1010_0101,
        fm: 0,
    };
    let bin = 0x0a50000f;

    // check assembler
    let i = assemble_line("fence ir,ow").unwrap().i();
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
