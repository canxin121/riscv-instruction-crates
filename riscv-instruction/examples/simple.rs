use riscv_instruction::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建寄存器
    let rd = IntegerRegister::new(1)?; // x1
    let rs1 = IntegerRegister::new(2)?; // x2
    let rs2 = IntegerRegister::new(3)?; // x3

    // 创建立即数
    let imm = Immediate::<12>::new(100)?;

    // 创建指令
    let add_inst = StandardSharedInstruction::I(StandardISharedInstructions::ADD { rd, rs1, rs2 });

    let addi_inst =
        StandardSharedInstruction::I(StandardISharedInstructions::ADDI { rd, rs1, imm });

    // 输出汇编
    println!("{}", add_inst); // add x1, x2, x3
    println!("{}", addi_inst); // addi x1, x2, 100

    Ok(())
}
