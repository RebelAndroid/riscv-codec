```Rust
use riscv_codec::{assembly::assemble_line, instruction::Instruction};
fn main() {
    // instruction can be assembled from strings
    let instr: Instruction = assemble_line("addi t0, t1, 1024").unwrap().i();
    // and disassembled
    println!("assembled instruction: {}", instr);

    // instructions can also be decoded from binary
    let instr2 = Instruction::decode(0xe0058513).unwrap();

    // and encoded
    assert_eq!(Instruction::encode(&instr2), 0xe0058513);
}

```

A crate for working with RISC-V Instructions. Instructions can be encoded and decoded from binary. Basic assembly and disassembly is also supported (Instructions can be converted to and from strings, no support is provided for labels or other features that would be found in a complete assembler). 


# Supported Instructions
- [x] RV64I
- [x] M
- [x] A
- [x] F
- [x] D
- [x] C
- [x] Zicsr
- [x] Zifencei

This crate is (somewhat) well tested. If you find any problems, or think some part of the API could be improved, please make an issue in the github repository.