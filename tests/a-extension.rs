use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::IRegister;

#[test]
fn load_reserved_word() {
    // check assembler
    let i = assemble_line("lr.w.aq a0,a1").unwrap().i();
    let expected = Instruction::LRW {
        dest: IRegister::A0,
        addr: IRegister::A1,
        aq: true,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x1405a52f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_conditional_word() {
    // check assembler
    let i = assemble_line("sc.w.rl ra,t4,a1").unwrap().i();
    let expected = Instruction::SCW {
        dest: IRegister::ReturnAddress,
        addr: IRegister::T4,
        src: IRegister::A1,
        aq: false,
        rl: true,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x1abea0af).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_swap_word() {
    // check assembler
    let i = assemble_line("amoswap.w t2,ra,t5").unwrap().i();
    let expected = Instruction::AMOSWAPW {
        dest: IRegister::T2,
        addr: IRegister::ReturnAddress,
        src: IRegister::T5,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x09e0a3af).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_add_word() {
    // check assembler
    let i = assemble_line("amoadd.w.aqrl a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOADDW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: true,
        rl: true,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x0741a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_xor_word() {
    // check assembler
    let i = assemble_line("amoxor.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOXORW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x2141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_and_word() {
    // check assembler
    let i = assemble_line("amoand.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOANDW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x6141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_or_word() {
    // check assembler
    let i = assemble_line("amoor.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOORW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x4141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_word() {
    // check assembler
    let i = assemble_line("amomin.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMINW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x8141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_word() {
    // check assembler
    let i = assemble_line("amomax.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMAXW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xa141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_unsigned_word() {
    // check assembler
    let i = assemble_line("amominu.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMINUW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_unsigned_word() {
    // check assembler
    let i = assemble_line("amomaxu.w a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMAXUW {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xe141a72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_reserved_doubleword() {
    // check assembler
    let i = assemble_line("lr.d.rl s6,s7").unwrap().i();
    let expected = Instruction::LRD {
        dest: IRegister::S6,
        addr: IRegister::S7,
        aq: false,
        rl: true,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x120bbb2f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_conditional_doubleword() {
    // check assembler
    let i = assemble_line("sc.d.aqrl tp,s10,a2").unwrap().i();
    let expected = Instruction::SCD {
        dest: IRegister::ThreadPointer,
        addr: IRegister::S10,
        src: IRegister::A2,
        aq: true,
        rl: true,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x1ecd322f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_swap_doubleword() {
    // check assembler
    let i = assemble_line("amoswap.d t2,ra,t5").unwrap().i();
    let expected = Instruction::AMOSWAPD {
        dest: IRegister::T2,
        addr: IRegister::ReturnAddress,
        src: IRegister::T5,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x09e0b3af).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_add_doubleword() {
    // check assembler
    let i = assemble_line("amoadd.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOADDD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x0141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_xor_doubleword() {
    // check assembler
    let i = assemble_line("amoxor.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOXORD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x2141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_and_doubleword() {
    // check assembler
    let i = assemble_line("amoand.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOANDD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x6141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_or_doubleword() {
    // check assembler
    let i = assemble_line("amoor.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOORD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x4141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_doubleword() {
    // check assembler
    let i = assemble_line("amomin.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMIND {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0x8141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_doubleword() {
    // check assembler
    let i = assemble_line("amomax.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMAXD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xa141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_unsigned_doubleword() {
    // check assembler
    let i = assemble_line("amominu.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMINUD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xc141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_unsigned_doubleword() {
    // check assembler
    let i = assemble_line("amomaxu.d a4,gp,s4").unwrap().i();
    let expected = Instruction::AMOMAXUD {
        dest: IRegister::A4,
        addr: IRegister::GlobalPointer,
        src: IRegister::S4,
        aq: false,
        rl: false,
    };
    assert_eq!(i, expected);

    // check decoder
    let i2 = decode_instruction(0xe141b72f).unwrap();
    assert_eq!(i2, expected);

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
