use riscv_disassembler::instruction::Instruction;
fn main() {
    println!(
        "Instruction size: {} bytes",
        std::mem::size_of::<Instruction>()
    );
}
