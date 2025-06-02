use riscv_instruction::*;

fn main() {
    let mut rng = rand::rng();

    // 生成随机标准指令
    let random_inst = StandardSharedInstruction::random_with_rng(&mut rng);
    println!("随机指令: {}", random_inst);

    // 生成随机压缩指令
    let random_compressed = RVCSharedInstruction::random_with_rng(&mut rng);
    println!("随机压缩指令: {}", random_compressed);
}
