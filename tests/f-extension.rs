use riscv_codec::assembly::assemble_line;
use riscv_codec::immediates::{IImmediate, SImmediate};
use riscv_codec::instruction::{Instruction, RoundingMode, disassemble_instruction};
use riscv_codec::register::{FRegister, IRegister};

#[test]
fn float_load_word() {
    let expected = Instruction::FLW {
        dest: FRegister::FA0,
        base: IRegister::A0,
        offset: IImmediate::try_from(64).unwrap(),
    };
    let bin = 0x04052507;

    // check assembler
    let i = assemble_line("flw fa0,64(a0)").unwrap().i();
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
fn float_store_word() {
    let expected = Instruction::FSW {
        base: IRegister::A5,
        src: FRegister::FS1,
        offset: SImmediate::try_from(-1).unwrap(),
    };
    let bin = 0xfe97afa7;

    // check assembler
    let i = assemble_line("fsw fs1,-1(a5)").unwrap().i();
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
fn float_multiply_add() {
    let expected = Instruction::FmaddS {
        dest: FRegister::FT2,
        src1: FRegister::FA1,
        src2: FRegister::FS3,
        src3: FRegister::FT3,
        rm: RoundingMode::RUP,
    };
    let bin = 0x1935b143;

    // check assembler
    let i = assemble_line("fmadd.s ft2, fa1, fs3, ft3, rup")
        .unwrap()
        .i();
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
fn float_multiply_subtract() {
    let expected = Instruction::FmsubS {
        dest: FRegister::FT2,
        src1: FRegister::FS0,
        src2: FRegister::FS3,
        src3: FRegister::FS11,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd9347147;

    // check assembler
    let i = assemble_line("fmsub.s ft2, fs0, fs3, fs11")
        .unwrap()
        .i();
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
fn float_negate_multiply_subtract() {
    let expected = Instruction::FnmsubS {
        dest: FRegister::FT2,
        src1: FRegister::FS0,
        src2: FRegister::FS3,
        src3: FRegister::FS11,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd934714b;

    // check assembler
    let i = assemble_line("fnmsub.s ft2, fs0, fs3, fs11")
        .unwrap()
        .i();
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
fn float_negate_multiply_add() {
    let expected = Instruction::FnmaddS {
        dest: FRegister::FA2,
        src1: FRegister::FT9,
        src2: FRegister::FS3,
        src3: FRegister::FT6,
        rm: RoundingMode::RTZ,
    };
    let bin = 0x313e964f;

    // check assembler
    let i = assemble_line("fnmadd.s fa2,ft9,fs3,ft6,rtz")
        .unwrap()
        .i();
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
fn float_add() {
    let expected = Instruction::FaddS {
        dest: FRegister::FT7,
        src1: FRegister::FA5,
        src2: FRegister::FS10,
        rm: RoundingMode::DYN,
    };
    let bin = 0x01a7f3d3;

    // check assembler
    let i = assemble_line("fadd.s ft7,fa5,fs10").unwrap().i();
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
fn float_subtract() {
    let expected = Instruction::FsubS {
        dest: FRegister::FT0,
        src1: FRegister::FT8,
        src2: FRegister::FS0,
        rm: RoundingMode::RTZ,
    };
    let bin = 0x088e1053;

    // check assembler
    let i = assemble_line("fsub.s ft0,ft8,fs0, rtz").unwrap().i();
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
fn float_multiply() {
    let expected = Instruction::FmulS {
        dest: FRegister::FS1,
        src1: FRegister::FS9,
        src2: FRegister::FT11,
        rm: RoundingMode::RMM,
    };
    let bin = 0x11fcc4d3;

    // check assembler
    let i = assemble_line("fmul.s fs1,fs9,ft11, rmm").unwrap().i();
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
fn float_divide() {
    let expected = Instruction::FdivS {
        dest: FRegister::FS6,
        src1: FRegister::FS10,
        src2: FRegister::FT2,
        rm: RoundingMode::RUP,
    };
    let bin = 0x182d3b53;

    // check assembler
    let i = assemble_line("fdiv.s fs6,fs10,ft2, rup").unwrap().i();
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
fn float_sqrt() {
    let expected = Instruction::FsqrtS {
        dest: FRegister::FT3,
        src: FRegister::FA3,
        rm: RoundingMode::RNE,
    };
    let bin = 0x580681d3;

    // check assembler
    let i = assemble_line("fsqrt.s ft3,fa3, rne").unwrap().i();
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
fn float_sign_inject() {
    let expected = Instruction::FsgnjS { dest: FRegister::FA5, src1: FRegister::FA3, src2: FRegister::FT1 };
    let bin = 0x201687d3;

    // check assembler
    let i = assemble_line("fsgnj.s fa5,fa3,ft1").unwrap().i();
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
fn float_sign_inject_negate() {
    let expected = Instruction::FsgnjnS { dest: FRegister::FA5, src1: FRegister::FA3, src2: FRegister::FT1 };
    let bin = 0x201697d3;

    // check assembler
    let i = assemble_line("fsgnjn.s fa5,fa3,ft1").unwrap().i();
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
fn float_sign_inject_xor() {
    let expected = Instruction::FsgnjxS { dest: FRegister::FS1, src1: FRegister::FT1, src2: FRegister::FT4 };
    let bin = 0x2040a4d3;

    // check assembler
    let i = assemble_line("fsgnjx.s fs1,ft1,ft4").unwrap().i();
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
fn float_minimum() {
    let expected = Instruction::FminS {
        dest: FRegister::FT1,
        src1: FRegister::FS4,
        src2: FRegister::FA5,
    };
    let bin = 0x28fa00d3;

    // check assembler
    let i = assemble_line("fmin.s ft1,fs4,fa5").unwrap().i();
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
fn float_maximum() {
    let expected = Instruction::FmaxS {
        dest: FRegister::FA3,
        src1: FRegister::FS9,
        src2: FRegister::FS2,
    };
    let bin = 0x292c96d3;

    // check assembler
    let i = assemble_line("fmax.s fa3,fs9,fs2").unwrap().i();
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
fn float_convert_word_from_single() {
    let expected = Instruction::FcvtWS {
        dest: IRegister::S1,
        src: FRegister::FS2,
        rm: RoundingMode::RUP,
    };
    let bin = 0xc00934d3;

    // check assembler
    let i = assemble_line("fcvt.w.s s1,fs2,rup").unwrap().i();
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
fn float_convert_unsigned_word_from_single() {
    let expected = Instruction::FcvtWuS {
        dest: IRegister::StackPointer,
        src: FRegister::FT3,
        rm: RoundingMode::RMM,
    };
    let bin = 0xc011c153;

    // check assembler
    let i = assemble_line("fcvt.wu.s sp,ft3, rmm").unwrap().i();
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
fn float_move_x_from_word() {
    let expected = Instruction::FmvXW {
        dest: IRegister::S2,
        src: FRegister::FT4,
    };
    let bin = 0xe0020953;

    // check assembler
    let i = assemble_line("fmv.x.w s2,ft4").unwrap().i();
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
fn float_equal_single() {
    let expected = Instruction::FeqS {
        dest: IRegister::A4,
        src1: FRegister::FS7,
        src2: FRegister::FT11,
    };
    let bin = 0xa1fba753;

    // check assembler
    let i = assemble_line("feq.s a4,fs7,ft11").unwrap().i();
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
fn float_less_than_single() {
    let expected = Instruction::FltS {
        dest: IRegister::S6,
        src1: FRegister::FT10,
        src2: FRegister::FA6,
    };
    let bin = 0xa10f1b53;

    // check assembler
    let i = assemble_line("flt.s s6,ft10,fa6").unwrap().i();
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
fn float_less_equal_single() {
    let expected = Instruction::FleS {
        dest: IRegister::S2,
        src1: FRegister::FS4,
        src2: FRegister::FT0,
    };
    let bin = 0xa00a0953;

    // check assembler
    let i = assemble_line("fle.s s2,fs4,ft0").unwrap().i();
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
fn float_class_single() {
    let expected = Instruction::FclassS {
        dest: IRegister::ThreadPointer,
        src: FRegister::FS3,
    };
    let bin = 0xe0099253;

    // check assembler
    let i = assemble_line("fclass.s tp,fs3").unwrap().i();
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
fn float_convert_single_from_word() {
    let expected = Instruction::FcvtSW {
        dest: FRegister::FA2,
        src: IRegister::T4,
        rm: RoundingMode::RDN,
    };
    let bin = 0xd00ea653;

    // check assembler
    let i = assemble_line("fcvt.s.w fa2,t4, rdn").unwrap().i();
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
fn float_convert_single_from_unsigned_word() {
    let expected = Instruction::FcvtSWu {
        dest: FRegister::FS4,
        src: IRegister::T6,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd01ffa53;

    // check assembler
    let i = assemble_line("fcvt.s.wu fs4,t6,dyn").unwrap().i();
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
fn float_move_word_from_x() {
    let expected = Instruction::FmvWX {
        dest: FRegister::FS3,
        src: IRegister::T1,
    };
    let bin = 0xf00309d3;

    // check assembler
    let i = assemble_line("fmv.w.x fs3,t1").unwrap().i();
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
fn float_convert_long_from_single() {
    let expected = Instruction::FCVTLS {
        dest: IRegister::S4,
        src: FRegister::FA7,
        rm: RoundingMode::DYN,
    };
    let bin = 0xc028fa53;

    // check assembler
    let i = assemble_line("fcvt.l.s s4,fa7").unwrap().i();
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
fn float_convert_unsigned_long_from_single() {
    let expected = Instruction::FCVTLUS {
        dest: IRegister::T2,
        src: FRegister::FT9,
        rm: RoundingMode::DYN,
    };
    let bin = 0xc03ef3d3;

    // check assembler
    let i = assemble_line("fcvt.lu.s t2,ft9").unwrap().i();
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
fn float_convert_single_from_long() {
    let expected = Instruction::FCVTSL {
        dest: FRegister::FS8,
        src: IRegister::S2,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd0297c53;

    // check assembler
    let i = assemble_line("fcvt.s.l fs8,s2").unwrap().i();
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
fn float_convert_single_from_unsigned_long() {
    let expected = Instruction::FCVTSLU {
        dest: FRegister::FT7,
        src: IRegister::FramePointer,
        rm: RoundingMode::DYN,
    };
    let bin = 0xd03473d3;

    // check assembler
    let i = assemble_line("fcvt.s.lu ft7,fp").unwrap().i();
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
