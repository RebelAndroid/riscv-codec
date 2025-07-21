use riscv_codec::instruction::Instruction;
fn main() {
    println!(
        "Instruction size: {} bytes",
        std::mem::size_of::<Instruction>()
    );
    println!("{}", (-2i64).pow(2));
}
