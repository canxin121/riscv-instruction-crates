// filepath: riscv-instruction/examples/separated_usage.rs
use riscv_instruction::separated_instructions::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建寄存器
    let xd = IntegerRegister::new(5)?;
    let xs1 = IntegerRegister::new(6)?;
    let xs2 = IntegerRegister::new(7)?;

    // 创建立即数 (假设为12位立即数类型)
    let imm12 = riscv_instruction_types::Immediate::<12>::new(200)?;

    // 创建一条 RV32I 指令 (例如：ADDI)
    let addi_rv32_inst = RiscvInstruction::RV32(RV32Instruction::I(RV32IInstructions::ADDI {
        xd,
        xs1,
        imm: imm12,
    }));
    println!("Separated ADDI (RV32): {}", addi_rv32_inst); // 输出: addi x5, x6, 200

    // 创建一条 RV64M 指令 (例如：MULW)
    let mulw_rv64_inst =
        RiscvInstruction::RV64(RV64Instruction::M(RV64MInstructions::MULW { xd, xs1, xs2 }));
    println!("Separated MULW (RV64): {}", mulw_rv64_inst); // 输出: mulw x5, x6, x7

    Ok(())
}
