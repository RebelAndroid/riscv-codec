use riscv_codec::assembly::assemble_line;
use riscv_codec::immediates::{IImmediate, SImmediate};
use riscv_codec::instruction::{Instruction, RoundingMode, disassemble_instruction};
use riscv_codec::register::{FRegister, IRegister};

#[test]
fn float_load_double() {
    let expected = Instruction::Fld {
        dest: FRegister::FA0,
        base: IRegister::A0,
        offset: IImmediate::try_from(64).unwrap(),
    };
    let bin = 0x04053507;

    // check assembler
    let i = assemble_line("fld fa0,64(a0)").unwrap().i();
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
fn float_store_double() {
    let expected = Instruction::Fsd {
        src: FRegister::FA0,
        base: IRegister::A0,
        offset: SImmediate::try_from(64).unwrap(),
    };
    let bin = 0x04a53027;

    // check assembler
    let i = assemble_line("fsd fa0,64(a0)").unwrap().i();
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
fn float_multiply_add_double() {
    let expected = Instruction::FmaddD {
        dest: FRegister::FS11,
        src1: FRegister::FT2,
        src2: FRegister::FT9,
        src3: FRegister::FA6,
        rm: RoundingMode::DYN,
    };
    let bin = 0x83d17dc3;

    // check assembler
    let i = assemble_line("fmadd.d fs11, ft2, ft9, fa6").unwrap().i();
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
fn float_multiply_subtract_double() {
    let expected = Instruction::FmsubD {
        dest: FRegister::FS11,
        src1: FRegister::FT2,
        src2: FRegister::FT9,
        src3: FRegister::FA6,
        rm: RoundingMode::DYN,
    };
    let bin = 0x83d17dc7;

    // check assembler
    let i = assemble_line("fmsub.d fs11, ft2, ft9, fa6").unwrap().i();
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
fn float_negate_multiply_subtract_double() {
    let expected = Instruction::FnmsubD {
        dest: FRegister::FS11,
        src1: FRegister::FT2,
        src2: FRegister::FT9,
        src3: FRegister::FA6,
        rm: RoundingMode::DYN,
    };
    let bin = 0x83d17dcb;

    // check assembler
    let i = assemble_line("fnmsub.d fs11, ft2, ft9, fa6").unwrap().i();
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
fn float_negate_multiply_add_double() {
    let expected = Instruction::FnmaddD {
        dest: FRegister::FS11,
        src1: FRegister::FT2,
        src2: FRegister::FT9,
        src3: FRegister::FA6,
        rm: RoundingMode::DYN,
    };
    let bin = 0x83d17dcf;

    // check assembler
    let i = assemble_line("fnmadd.d fs11, ft2, ft9, fa6").unwrap().i();
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
fn float_add_double() {
    let expected = Instruction::FaddD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FA2,
        rm: RoundingMode::DYN,
    };
    let bin = 0x02ca78d3;

    // check assembler
    let i = assemble_line("fadd.d fa7, fs4, fa2").unwrap().i();
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
fn float_subtract_double() {
    let expected = Instruction::FsubD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FA2,
        rm: RoundingMode::DYN,
    };
    let bin = 0x0aca78d3;

    // check assembler
    let i = assemble_line("fsub.d fa7, fs4, fa2").unwrap().i();
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
fn float_multiply_double() {
    let expected = Instruction::FmulD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FA2,
        rm: RoundingMode::DYN,
    };
    let bin = 0x12ca78d3;

    // check assembler
    let i = assemble_line("fmul.d fa7, fs4, fa2").unwrap().i();
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
fn float_divide_double() {
    let expected = Instruction::FdivD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FA2,
        rm: RoundingMode::DYN,
    };
    let bin = 0x1aca78d3;

    // check assembler
    let i = assemble_line("fdiv.d fa7, fs4, fa2").unwrap().i();
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
fn float_square_root_double() {
    let expected = Instruction::FsqrtD {
        dest: FRegister::FA7,
        src: FRegister::FS4,
        rm: RoundingMode::DYN,
    };
    let bin = 0x5a0a78d3;

    // check assembler
    let i = assemble_line("fsqrt.d fa7, fs4").unwrap().i();
    assert_eq!(i, expected);

    // check decoder
    let i2 = Instruction::decode(bin).unwrap();
    assert_eq!(i2, expected);

    // check encoder
    let b = Instruction::encode(&i);
    assert_eq!(b, bin);


    println!("disassembled result: {}", disassemble_instruction(&i));
    // check disassembler
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn float_sign_inject_double() {
    let expected = Instruction::FsgnjD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FT1,
    };
    let bin = 0x221a08d3;

    // check assembler
    let i = assemble_line("fsgnj.d fa7, fs4, ft1").unwrap().i();
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
fn float_sign_inject_negate_double() {
    let expected = Instruction::FsgnjnD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FT1,
    };
    let bin = 0x221a18d3;

    // check assembler
    let i = assemble_line("fsgnjn.d fa7, fs4, ft1").unwrap().i();
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
fn float_sign_inject_xor_double() {
    let expected = Instruction::FsgnjxD {
        dest: FRegister::FA7,
        src1: FRegister::FS4,
        src2: FRegister::FT1,
    };
    let bin = 0x221a28d3;

    // check assembler
    let i = assemble_line("fsgnjx.d fa7, fs4, ft1").unwrap().i();
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
fn float_min_double() {
    let expected = Instruction::FminD { dest: FRegister::FA6, src1: FRegister::FS0, src2: FRegister::FA1 };
    let bin = 0x2ab40853;

    // check assembler
    let i = assemble_line("fmin.d fa6, fs0, fa1").unwrap().i();
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
fn float_max_double() {
    let expected = Instruction::FmaxD { dest: FRegister::FA6, src1: FRegister::FS0, src2: FRegister::FA1 };
    let bin = 0x2ab41853;

    // check assembler
    let i = assemble_line("fmax.d fa6, fs0, fa1").unwrap().i();
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
fn float_convert_single_from_double() {
    let expected = Instruction::FcvtSD { dest: FRegister::FS5, src: FRegister::FS10, rm: RoundingMode::DYN };
    let bin = 0x401d7ad3;

    // check assembler
    let i = assemble_line("fcvt.s.d fs5, fs10").unwrap().i();
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
fn float_convert_double_from_single() {
    let expected = Instruction::FcvtDS { dest: FRegister::FS5, src: FRegister::FS10, rm: RoundingMode::DYN };
    let bin = 0x420d7ad3;

    // check assembler
    let i = assemble_line("fcvt.d.s fs5, fs10").unwrap().i();
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
fn float_equal_double() {
     let expected = Instruction::FeqD { dest: IRegister::T1, src1: FRegister::FS1, src2: FRegister::FS2 };
    let bin = 0xa324a353;

    // check assembler
    let i = assemble_line("feq.d t1, fs1, fs2").unwrap().i();
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
fn float_less_than_double() {
     let expected = Instruction::FltD { dest: IRegister::T1, src1: FRegister::FS1, src2: FRegister::FS2 };
    let bin = 0xa3249353;

    // check assembler
    let i = assemble_line("flt.d t1, fs1, fs2").unwrap().i();
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
fn float_less_equal_double() {
     let expected = Instruction::FleD { dest: IRegister::T1, src1: FRegister::FS1, src2: FRegister::FS2 };
    let bin = 0xa3248353;

    // check assembler
    let i = assemble_line("fle.d t1, fs1, fs2").unwrap().i();
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
fn float_class_double() {
     let expected = Instruction::FclassD { dest: IRegister::S4, src1: FRegister::FT0 };
    let bin = 0xe2001a53;

    // check assembler
    let i = assemble_line("fclass.d s4, ft0").unwrap().i();
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
fn float_convert_word_from_double() {
     let expected: Instruction = Instruction::FcvtWD { dest: IRegister::T5, src1: FRegister::FS8, rm: RoundingMode::DYN };
    let bin = 0xc20c7f53;

    // check assembler
    let i = assemble_line("fcvt.w.d t5,fs8").unwrap().i();
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
fn float_convert_unsigned_word_from_double() {
     let expected: Instruction = Instruction::FcvtWuD { dest: IRegister::A6, src1: FRegister::FA5, rm: RoundingMode::DYN };
    let bin = 0xc217f853;

    // check assembler
    let i = assemble_line("fcvt.wu.d a6,fa5").unwrap().i();
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
fn float_convert_double_from_word() {
     let expected: Instruction = Instruction::FcvtDW { dest: FRegister::FT10, src1: IRegister::S8, rm: RoundingMode::DYN };
    let bin = 0xd20c7f53;

    // check assembler
    let i = assemble_line("fcvt.d.w ft10, s8").unwrap().i();
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
fn float_convert_double_from_unsigned_word() {
     let expected: Instruction = Instruction::FcvtDWu { dest: FRegister::FT10, src1: IRegister::S8, rm: RoundingMode::DYN };
    let bin = 0xd21c7f53;

    // check assembler
    let i = assemble_line("fcvt.d.wu ft10, s8").unwrap().i();
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
fn float_convert_long_from_double() {
     let expected: Instruction = Instruction::FcvtLD { dest: IRegister::FramePointer, src1: FRegister::FS3, rm: RoundingMode::DYN };
    let bin = 0xc229f453;

    // check assembler
    let i = assemble_line("fcvt.l.d fp, fs3").unwrap().i();
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
fn float_convert_unsigned_long_from_double() {
     let expected: Instruction = Instruction::FcvtLuD { dest: IRegister::FramePointer, src1: FRegister::FS3, rm: RoundingMode::DYN };
    let bin = 0xc239f453;

    // check assembler
    let i = assemble_line("fcvt.lu.d fp, fs3").unwrap().i();
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
     let expected: Instruction = Instruction::FmvXD { dest: IRegister::A6, src: FRegister::FT11 };
    let bin = 0xe20f8853;

    // check assembler
    let i = assemble_line("fmv.x.d a6,ft11").unwrap().i();
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
fn float_convert_double_from_long() {
     let expected: Instruction = Instruction::FcvtDL { dest: FRegister::FS0, src: IRegister::A4, rm: RoundingMode::DYN };
    let bin = 0xd2277453;

    // check assembler
    let i = assemble_line("fcvt.d.l fs0, a4").unwrap().i();
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
fn float_convert_double_from_unsigned_long() {
     let expected: Instruction = Instruction::FcvtDLu { dest: FRegister::FS0, src: IRegister::A4, rm: RoundingMode::DYN };
    let bin = 0xd2377453;

    // check assembler
    let i = assemble_line("fcvt.d.lu fs0, a4").unwrap().i();
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
fn float_move_double_from_x() {
     let expected: Instruction = Instruction::FmvDX { dest: FRegister::FS7, src: IRegister::S8 };
    let bin = 0xf20c0bd3;

    // check assembler
    let i = assemble_line("fmv.d.x fs7, s8").unwrap().i();
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