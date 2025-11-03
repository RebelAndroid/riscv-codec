use riscv_codec::instruction::Instruction;
use std::io::{self, Read};

/// A simple RISCV disassembler that reads machine code from stdin and outputs text to stdout.
///
/// Doesn't support compressed instructions (assumes each isntruction is 4 bytes)
fn main() -> io::Result<()> {
    let mut source = io::stdin();

    loop {
        let word: u32 = {
            let mut buffer: [u8; 4] = [0; 4];
            match source.read_exact(&mut buffer) {
                Ok(()) => u32::from_le_bytes(buffer),
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        };
        match Instruction::decode(word) {
            Ok(instr) => println!("{}", instr),
            Err(e) => eprintln!("Failed to decode 0x{:08x}: {:?}", word, e),
        }
    }

    Ok(())
}
