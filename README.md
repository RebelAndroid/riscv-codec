```Rust
fn main() {
    // instruction can be assembled from strings
    let instr: Instruction = assemble_line("addi t0, t1, 1024").unwrap().i();
    // and disassembled
    println!("assembled instruction: {}", instr);

    // instructions can also be decoded from binary
    let instr2 = decode_instruction(0xe0058513).unwrap();

    // and encoded
    assert_eq!(encode_instruction(&instr2), 0xe0058513);
}
```

This crate is (somewhat) well tested. If you find any problems, or think some part of the API could be improved, please make an issue in the github repository.

# Supported Instructions
- [x] RV64I
- [x] M
- [x] A
- [x] F
- [ ] D
- [x] C
- [x] Zicsr
- [x] Zifencei