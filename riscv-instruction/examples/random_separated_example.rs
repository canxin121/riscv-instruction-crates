use riscv_instruction::separated_instructions::*;

fn main() {
    let mut rng = rand::rng();

    // 生成一个随机的 RV32 指令 (来自其任何扩展)
    let random_rv32_inst = RV32Instruction::random_with_rng(&mut rng);
    println!("Random RV32 Instruction: {}", random_rv32_inst);

    // 生成一个随机的 RV64 指令 (来自其任何扩展)
    let random_rv64_inst = RV64Instruction::random_with_rng(&mut rng);
    println!("Random RV64 Instruction: {}", random_rv64_inst);

    // 从特定的 RV64 扩展中生成随机指令 (例如：RV64M)
    let random_rv64m_inst = RV64MInstructions::random_with_rng(&mut rng);
    println!("Random RV64M Instruction: {}", random_rv64m_inst);
}
