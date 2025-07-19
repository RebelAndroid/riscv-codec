use riscv_disassembler::{
    immediates::{BImmediate, IImmediate, JImmediate, SImmediate, Shamt, UImmediate},
    instruction::{assemble_line, decode_instruction, disassemble_instruction, Instruction},
    register::IRegister,
};

#[test]
fn load_upper_immediate() {
    // check assembler
    let i = assemble_line("lui s2,400").unwrap().i();
    let expected = Instruction::LUI{dest: IRegister::S2, imm: UImmediate::try_from(400).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x00190937).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn add_upper_immediate_to_program_counter() {
    // check assembler
    let i = assemble_line("auipc a3,-1").unwrap().i();
    let expected = Instruction::AUIPC{dest: IRegister::A3, imm: UImmediate::try_from(-1).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xfffff697).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link() {
    // check assembler
    let i = assemble_line("jal zero,-1016708").unwrap().i();
    let expected = Instruction::JAL{dest: IRegister::Zero, offset: JImmediate::try_from(-1016708).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc7d0706f).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn jump_and_link_register() {
    // check assembler
    let i = assemble_line("jalr a0,-2048(t0)").unwrap().i();
    let expected = Instruction::JALR{dest: IRegister::A0, base: IRegister::T0, offset: IImmediate::try_from(-2048).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x80028567).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_equal() {
    // check assembler
    let i = assemble_line("beq t2,sp,2").unwrap().i();
    let expected = Instruction::BEQ{src1: IRegister::T2, src2: IRegister::StackPointer, offset: BImmediate::try_from(2).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x00238163).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_not_equal() {
    // check assembler
    let i = assemble_line("bne tp,a4,4094").unwrap().i();
    let expected = Instruction::BNE{src1: IRegister::ThreadPointer, src2: IRegister::A4, offset: BImmediate::try_from(4094).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x7ee21fe3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_less_than() {
    // check assembler
    let i = assemble_line("blt a1,t6,-4096").unwrap().i();
    let expected = Instruction::BLT{src1: IRegister::A1, src2: IRegister::T6, offset: BImmediate::try_from(-4096).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x81f5c063).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_greater_equal() {
    // check assembler
    let i = assemble_line("bge a1,t6,-2030").unwrap().i();
    let expected = Instruction::BGE{src1: IRegister::A1, src2: IRegister::T6, offset: BImmediate::try_from(-2030).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x81f5d9e3).unwrap();
    assert_eq!(i2, expected,);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_less_than_unsigned() {
    // check assembler
    let i = assemble_line("bltu t0,s2,512").unwrap().i();
    let expected = Instruction::BLTU{src1: IRegister::T0, src2: IRegister::S2, offset: BImmediate::try_from(512).unwrap()};
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x2122e063).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn branch_greater_equal_unsigned() {
    let i = assemble_line("bgeu s1,a3,-128").unwrap().i();
    let expected = Instruction::BGEU{src1: IRegister::S1, src2: IRegister::A3, offset: BImmediate::try_from(-128).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xf8d4f0e3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_byte() {
    let i = assemble_line("lb t2,8(a0)").unwrap().i();
    let expected = Instruction::LB{dest: IRegister::T2, base: IRegister::A0, offset: IImmediate::try_from(8).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00850383).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_half() {
    let i = assemble_line("lh s3,-16(sp)").unwrap().i();
    let expected = Instruction::LH{dest: IRegister::S3, base: IRegister::StackPointer, offset: IImmediate::try_from(-16).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xff011983).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_word() {
    let i = assemble_line("lw a4,1024(t5)").unwrap().i();
    let expected = Instruction::LW{dest: IRegister::A4, base: IRegister::T5, offset: IImmediate::try_from(1024).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x400f2703).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_byte_unsigned() {
    let i = assemble_line("lbu s0,63(a6)").unwrap().i();
    let expected = Instruction::LBU{dest: IRegister::FramePointer, base: IRegister::A6, offset: IImmediate::try_from(63).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x03f84403).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_half_unsigned() {
    let i = assemble_line("lhu t4,-2047(a1)").unwrap().i();
    let expected = Instruction::LHU{dest: IRegister::T4, base: IRegister::A1, offset: IImmediate::try_from(-2047).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x8015de83).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_byte() {
    let i = assemble_line("sb t1,127(a2)").unwrap().i();
    let expected = Instruction::SB{base: IRegister::A2, src: IRegister::T1, offset: SImmediate::try_from(127).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x06660fa3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_half() {
    let i = assemble_line("sh a5,-32(s2)").unwrap().i();
    let expected = Instruction::SH{base: IRegister::S2, src: IRegister::A5, offset: SImmediate::try_from(-32).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xfef91023).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_word() {
    let i = assemble_line("sw s7,2046(t6)").unwrap().i();
    let expected = Instruction::SW{base: IRegister::T6, src: IRegister::S7, offset: SImmediate::try_from(2046).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x7f7faf23).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn add_immediate() {
    let i = assemble_line("addi t3,s4,99").unwrap().i();
    let expected = Instruction::ADDI{dest: IRegister::T3, src: IRegister::S4, imm: IImmediate::try_from(99).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x063a0e13).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than_immediate() {
    let i = assemble_line("slti a1,t2,-12").unwrap().i();
    let expected = Instruction::SLTI{dest: IRegister::A1, src: IRegister::T2, imm: IImmediate::try_from(-12).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xff43a593).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than_immediate_unsigned() {
    let i = assemble_line("sltiu s5,a0,2047").unwrap().i();
    let expected = Instruction::SLTIU{dest: IRegister::S5, src: IRegister::A0, imm: IImmediate::try_from(2047).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x7ff53a93).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn xor_immediate() {
    let i = assemble_line("xori a7,ra,15").unwrap().i();
    let expected = Instruction::XORI{dest: IRegister::A7, src: IRegister::ReturnAddress, imm: IImmediate::try_from(15).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00f0c893).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn or_immediate() {
    let i = assemble_line("ori t6,gp,31").unwrap().i();
    let expected = Instruction::ORI{dest: IRegister::T6, src: IRegister::GlobalPointer, imm: IImmediate::try_from(31).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01f1ef93).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn and_immediate() {
    let i = assemble_line("andi gp,sp,-256").unwrap().i();
    let expected = Instruction::ANDI{dest: IRegister::GlobalPointer, src: IRegister::StackPointer, imm: IImmediate::try_from(-256).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xf0017193).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn shift_left_logical_immediate() {
    let i = assemble_line("slli t1,s0,13").unwrap().i();
    let expected = Instruction::SLLI{dest: IRegister::T1, src: IRegister::FramePointer, shamt: Shamt::try_from(13).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00d41313).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_logical_immediate() {
    let i = assemble_line("srli s2,a6,9").unwrap().i();
    let expected = Instruction::SRLI{dest: IRegister::S2, src: IRegister::A6, shamt: Shamt::try_from(9).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00985913).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_arithmetic_immediate() {
    let i = assemble_line("srai s1,s3,17").unwrap().i();
    let expected = Instruction::SRAI{dest: IRegister::S1, src: IRegister::S3, shamt: Shamt::try_from(17).unwrap()};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x4119d493).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn add() {
    let i = assemble_line("add t0,t1,t2").unwrap().i();
    let expected = Instruction::ADD{dest: IRegister::T0, src1: IRegister::T1, src2: IRegister::T2};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x007302b3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn sub() {
    let i = assemble_line("sub s4,s5,s6").unwrap().i();
    let expected = Instruction::SUB{dest: IRegister::S4, src1: IRegister::S5, src2: IRegister::S6};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x416a8a33).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn shift_left_logical() {
    let i = assemble_line("sll a2,a3,a4").unwrap().i();
    let expected = Instruction::SLL{dest: IRegister::A2, src1: IRegister::A3, src2: IRegister::A4};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00e69633).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than() {
    let i = assemble_line("slt t3,t4,t5").unwrap().i();
    let expected = Instruction::SLT{dest: IRegister::T3, src1: IRegister::T4, src2: IRegister::T5};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01eeae33).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than_unsigned() {
    let i = assemble_line("sltu s6,s7,zero").unwrap().i();
    let expected = Instruction::SLTU{dest: IRegister::S6, src1: IRegister::S7, src2: IRegister::Zero};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x000bbb33).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn xor() {
    let i = assemble_line("xor a5,a6,a7").unwrap().i();
    let expected = Instruction::XOR{dest: IRegister::A5, src1: IRegister::A6, src2: IRegister::A7};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x011847b3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_logical() {
    let i = assemble_line("srl t1,t2,t3").unwrap().i();
    let expected = Instruction::SRL{dest: IRegister::T1, src1: IRegister::T2, src2: IRegister::T3};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01c3d333).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_arithmetic() {
    let i = assemble_line("sra s0,s1,s2").unwrap().i();
    let expected = Instruction::SRA{dest: IRegister::FramePointer, src1: IRegister::S1, src2: IRegister::S2};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x4124d433).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn or() {
    let i = assemble_line("or t4,t5,t6").unwrap().i();
    let expected = Instruction::OR{dest: IRegister::T4, src1: IRegister::T5, src2: IRegister::T6};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01ff6eb3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn and() {
    let i = assemble_line("and s1,s2,s3").unwrap().i();
    let expected = Instruction::AND{dest: IRegister::S1, src1: IRegister::S2, src2: IRegister::S3};
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x013974b3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}