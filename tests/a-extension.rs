use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::IRegister;

#[test]
fn load_reserved_word() {
    // check assembler
    let i = assemble_line("lr.w.aq a0,a1").unwrap().i();
    assert_eq!(
        i,
        Instruction::LRW(IRegister::A0, IRegister::A1, true, false)
    );

    // check decoder
    let i2 = decode_instruction(0x1405a52f).unwrap();
    assert_eq!(
        i2,
        Instruction::LRW(IRegister::A0, IRegister::A1, true, false)
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_conditional_word() {
    // check assembler
    let i = assemble_line("sc.w.rl ra,t4,a1").unwrap().i();
    assert_eq!(
        i,
        Instruction::SCW(
            IRegister::ReturnAddress,
            IRegister::T4,
            IRegister::A1,
            false,
            true
        )
    );

    // check decoder
    let i2 = decode_instruction(0x1abea0af).unwrap();
    assert_eq!(
        i2,
        Instruction::SCW(
            IRegister::ReturnAddress,
            IRegister::T4,
            IRegister::A1,
            false,
            true
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_swap_word() {
    // check assembler
    let i = assemble_line("amoswap.w t2,ra,t5").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOSWAPW(
            IRegister::T2,
            IRegister::ReturnAddress,
            IRegister::T5,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x09e0a3af).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOSWAPW(
            IRegister::T2,
            IRegister::ReturnAddress,
            IRegister::T5,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_add_word() {
    // check assembler
    let i = assemble_line("amoadd.w.aqrl a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOADDW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            true,
            true
        )
    );

    // check decoder
    let i2 = decode_instruction(0x0741a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOADDW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            true,
            true
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_xor_word() {
    // check assembler
    let i = assemble_line("amoxor.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOXORW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x2141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOXORW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_and_word() {
    // check assembler
    let i = assemble_line("amoand.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOANDW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x6141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOANDW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_or_word() {
    // check assembler
    let i = assemble_line("amoor.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOORW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x4141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOORW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_word() {
    // check assembler
    let i = assemble_line("amomin.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMINW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x8141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMINW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_word() {
    // check assembler
    let i = assemble_line("amomax.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMAXW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0xa141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMAXW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_unsigned_word() {
    // check assembler
    let i = assemble_line("amominu.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMINUW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0xc141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMINUW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_unsigned_word() {
    // check assembler
    let i = assemble_line("amomaxu.w a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMAXUW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0xe141a72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMAXUW(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn load_reserved_doubleword() {
    // check assembler
    let i = assemble_line("lr.d.rl s6,s7").unwrap().i();
    assert_eq!(
        i,
        Instruction::LRD(IRegister::S6, IRegister::S7, false, true)
    );

    // check decoder
    let i2 = decode_instruction(0x120bbb2f).unwrap();
    assert_eq!(
        i2,
        Instruction::LRD(IRegister::S6, IRegister::S7, false, true)
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn store_conditional_doubleword() {
    // check assembler
    let i = assemble_line("sc.d.aqrl tp,s10,a2").unwrap().i();
    assert_eq!(
        i,
        Instruction::SCD(
            IRegister::ThreadPointer,
            IRegister::S10,
            IRegister::A2,
            true,
            true
        )
    );

    // check decoder
    let i2 = decode_instruction(0x1ecd322f).unwrap();
    assert_eq!(
        i2,
        Instruction::SCD(
            IRegister::ThreadPointer,
            IRegister::S10,
            IRegister::A2,
            true,
            true
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_swap_doubleword() {
    // check assembler
    let i = assemble_line("amoswap.d t2,ra,t5").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOSWAPD(
            IRegister::T2,
            IRegister::ReturnAddress,
            IRegister::T5,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x09e0b3af).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOSWAPD(
            IRegister::T2,
            IRegister::ReturnAddress,
            IRegister::T5,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_add_doubleword() {
    // check assembler
    let i = assemble_line("amoadd.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOADDD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x0141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOADDD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_xor_doubleword() {
    // check assembler
    let i = assemble_line("amoxor.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOXORD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x2141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOXORD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_and_doubleword() {
    // check assembler
    let i = assemble_line("amoand.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOANDD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x6141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOANDD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_or_doubleword() {
    // check assembler
    let i = assemble_line("amoor.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOORD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x4141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOORD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_doubleword() {
    // check assembler
    let i = assemble_line("amomin.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMIND(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0x8141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMIND(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_doubleword() {
    // check assembler
    let i = assemble_line("amomax.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMAXD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0xa141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMAXD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_min_unsigned_doubleword() {
    // check assembler
    let i = assemble_line("amominu.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMINUD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0xc141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMINUD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}

#[test]
fn amo_max_unsigned_doubleword() {
    // check assembler
    let i = assemble_line("amomaxu.d a4,gp,s4").unwrap().i();
    assert_eq!(
        i,
        Instruction::AMOMAXUD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check decoder
    let i2 = decode_instruction(0xe141b72f).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOMAXUD(
            IRegister::A4,
            IRegister::GlobalPointer,
            IRegister::S4,
            false,
            false
        )
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap().i();
    assert_eq!(i, i3);
}
