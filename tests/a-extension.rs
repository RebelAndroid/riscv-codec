use riscv_disassembler::instruction::{
    Instruction, assemble_line, decode_instruction, disassemble_instruction,
};
use riscv_disassembler::register::IRegister;

#[test]
fn load_reserved_word() {
    // check assembler
    let i = assemble_line("lr.w.aq a0,a1").unwrap();
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
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn store_conditional_word() {
    // check assembler
    let i = assemble_line("sc.w.rl ra,t4,a1").unwrap();
    assert_eq!(
        i,
        Instruction::SCW(IRegister::ReturnAddress, IRegister::T4, IRegister::A1, false, true)
    );

    // check decoder
    let i2 = decode_instruction(0x1abea0af).unwrap();
    assert_eq!(
        i2,
        Instruction::SCW(IRegister::ReturnAddress, IRegister::T4, IRegister::A1, false, true)
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}

#[test]
fn amo_swap_word() {
    // check assembler
    let i = assemble_line("amoswap.w t2,ra,t5").unwrap();
    assert_eq!(
        i,
        Instruction::AMOSWAPW(IRegister::T2, IRegister::ReturnAddress, IRegister::T5, false, false)
    );

    // check decoder
    let i2 = decode_instruction(0x09e0a3af).unwrap();
    assert_eq!(
        i2,
        Instruction::AMOSWAPW(IRegister::T2, IRegister::ReturnAddress, IRegister::T5, false, false)
    );

    // check disassembler
    println!("{}", disassemble_instruction(&i));
    let i3 = assemble_line(&disassemble_instruction(&i)).unwrap();
    assert_eq!(i, i3);
}