// filepath: riscv-instruction/examples/merged_usage.rs
use riscv_instruction::merged_instructions::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建寄存器
    let xd = IntegerRegister::new(1)?;
    let xs1 = IntegerRegister::new(2)?;
    let xs2 = IntegerRegister::new(3)?;

    // 创建一个共享指令 (例如：ADD)
    let add_inst = RiscvInstruction::Shared(SharedInstruction::I(ISharedInstructions::ADD {
        xd,
        xs1,
        xs2,
    }));
    println!("Merged ADD: {}", add_inst); // 输出: add x1, x2, x3

    // 创建一个 RV64 特有的指令 (例如：ADDW)
    let addw_inst = RiscvInstruction::Specific(SpecificInstruction::RV64(
        RV64SpecificInstruction::I(RV64ISpecificInstructions::ADDW { xd, xs1, xs2 }),
    ));
    println!("Merged ADDW (RV64): {}", addw_inst); // 输出: addw x1, x2, x3

    Ok(())
}
