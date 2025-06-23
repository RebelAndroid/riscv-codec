use riscv_disassembler::{
    immediates::{BImmediate, IImmediate, JImmediate, SImmediate, Shamt, UImmediate},
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

#[test]
fn branch_less_than_unsigned() {
    // check assembler
    let i = assemble_line("bltu t0,s2,512").unwrap();
    let expected = Instruction::BLTU(IRegister::T0, IRegister::S2, BImmediate::from_val(512));
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x2122e063).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn branch_greater_equal_unsigned() {
    let i = assemble_line("bgeu s1,a3,-128").unwrap();
    let expected = Instruction::BGEU(IRegister::S1, IRegister::A3, BImmediate::from_val(-128));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xf8d4f0e3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_byte() {
    let i = assemble_line("lb t2,8(a0)").unwrap();
    let expected = Instruction::LB(IRegister::T2, IRegister::A0, IImmediate::from_val(8));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00850383).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_half() {
    let i = assemble_line("lh s3,-16(sp)").unwrap();
    let expected = Instruction::LH(IRegister::S3, IRegister::StackPointer, IImmediate::from_val(-16));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xff011983).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_word() {
    let i = assemble_line("lw a4,1024(t5)").unwrap();
    let expected = Instruction::LW(IRegister::A4, IRegister::T5, IImmediate::from_val(1024));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x400f2703).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_byte_unsigned() {
    let i = assemble_line("lbu s0,63(a6)").unwrap();
    let expected = Instruction::LBU(IRegister::FramePointer, IRegister::A6, IImmediate::from_val(63));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x03f84403).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn load_half_unsigned() {
    let i = assemble_line("lhu t4,-2047(a1)").unwrap();
    let expected = Instruction::LHU(IRegister::T4, IRegister::A1, IImmediate::from_val(-2047));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x8015de83).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn store_byte() {
    let i = assemble_line("sb t1,127(a2)").unwrap();
    let expected = Instruction::SB(IRegister::A2, IRegister::T1, SImmediate::from_val(127));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x06660fa3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn store_half() {
    let i = assemble_line("sh a5,-32(s2)").unwrap();
    let expected = Instruction::SH(IRegister::S2, IRegister::A5, SImmediate::from_val(-32));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xfef91023).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn store_word() {
    let i = assemble_line("sw s7,2046(t6)").unwrap();
    let expected = Instruction::SW(IRegister::T6, IRegister::S7, SImmediate::from_val(2046));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x7f7faf23).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn add_immediate() {
    let i = assemble_line("addi t3,s4,99").unwrap();
    let expected = Instruction::ADDI(IRegister::T3, IRegister::S4, IImmediate::from_val(99));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x063a0e13).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than_immediate() {
    let i = assemble_line("slti a1,t2,-12").unwrap();
    let expected = Instruction::SLTI(IRegister::A1, IRegister::T2, IImmediate::from_val(-12));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xff43a593).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than_immediate_unsigned() {
    let i = assemble_line("sltiu s5,a0,2047").unwrap();
    let expected = Instruction::SLTIU(IRegister::S5, IRegister::A0, IImmediate::from_val(2047));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x7ff53a93).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn xor_immediate() {
    let i = assemble_line("xori a7,ra,15").unwrap();
    let expected = Instruction::XORI(IRegister::A7, IRegister::ReturnAddress, IImmediate::from_val(15));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00f0c893).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn or_immediate() {
    let i = assemble_line("ori t6,gp,31").unwrap();
    let expected = Instruction::ORI(IRegister::T6, IRegister::GlobalPointer, IImmediate::from_val(31));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01f1ef93).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn and_immediate() {
    let i = assemble_line("andi gp,sp,-256").unwrap();
    let expected = Instruction::ANDI(IRegister::GlobalPointer, IRegister::StackPointer, IImmediate::from_val(-256));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0xf0017193).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn shift_left_logical_immediate() {
    let i = assemble_line("slli t1,s0,13").unwrap();
    let expected = Instruction::SLLI(IRegister::T1, IRegister::FramePointer, Shamt::from_val(13));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00d41313).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_logical_immediate() {
    let i = assemble_line("srli s2,a6,9").unwrap();
    let expected = Instruction::SRLI(IRegister::S2, IRegister::A6, Shamt::from_val(9));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00985913).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_arithmetic_immediate() {
    let i = assemble_line("srai s1,s3,17").unwrap();
    let expected = Instruction::SRAI(IRegister::S1, IRegister::S3, Shamt::from_val(17));
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x4119d493).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn add() {
    let i = assemble_line("add t0,t1,t2").unwrap();
    let expected = Instruction::ADD(IRegister::T0, IRegister::T1, IRegister::T2);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x007302b3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn sub() {
    let i = assemble_line("sub s4,s5,s6").unwrap();
    let expected = Instruction::SUB(IRegister::S4, IRegister::S5, IRegister::S6);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x416a8a33).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn shift_left_logical() {
    let i = assemble_line("sll a2,a3,a4").unwrap();
    let expected = Instruction::SLL(IRegister::A2, IRegister::A3, IRegister::A4);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x00e69633).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than() {
    let i = assemble_line("slt t3,t4,t5").unwrap();
    let expected = Instruction::SLT(IRegister::T3, IRegister::T4, IRegister::T5);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01eeae33).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn set_less_than_unsigned() {
    let i = assemble_line("sltu s6,s7,zero").unwrap();
    let expected = Instruction::SLTU(IRegister::S6, IRegister::S7, IRegister::Zero);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x000bbb33).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn xor() {
    let i = assemble_line("xor a5,a6,a7").unwrap();
    let expected = Instruction::XOR(IRegister::A5, IRegister::A6, IRegister::A7);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x011847b3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_logical() {
    let i = assemble_line("srl t1,t2,t3").unwrap();
    let expected = Instruction::SRL(IRegister::T1, IRegister::T2, IRegister::T3);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01c3d333).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn shift_right_arithmetic() {
    let i = assemble_line("sra s0,s1,s2").unwrap();
    let expected = Instruction::SRA(IRegister::FramePointer, IRegister::S1, IRegister::S2);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x4124d433).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn or() {
    let i = assemble_line("or t4,t5,t6").unwrap();
    let expected = Instruction::OR(IRegister::T4, IRegister::T5, IRegister::T6);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x01ff6eb3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn and() {
    let i = assemble_line("and s1,s2,s3").unwrap();
    let expected = Instruction::AND(IRegister::S1, IRegister::S2, IRegister::S3);
    assert_eq!(i, expected);

    let i2 = decode_instruction(0x013974b3).unwrap();
    assert_eq!(i2, expected);

    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}