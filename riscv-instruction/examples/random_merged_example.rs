// filepath: riscv-instruction/examples/random_merged_example.rs
use riscv_instruction::merged_instructions::*;

fn main() {
    let mut rng = rand::rng();

    // 生成一个随机指令 (可能是共享的或特定于架构的)
    let random_merged_inst = RiscvInstruction::random_with_rng(&mut rng);
    println!("Random Merged Instruction: {}", random_merged_inst);

    // 生成一个随机的共享指令
    let random_shared_inst = SharedInstruction::random_with_rng(&mut rng);
    println!("Random Shared Instruction: {}", random_shared_inst);

    // 生成一个随机的 RV32 特有指令
    let random_rv32_specific_inst = RV32SpecificInstruction::random_with_rng(&mut rng);
    println!(
        "Random RV32 Specific Instruction: {}",
        random_rv32_specific_inst
    );
}
