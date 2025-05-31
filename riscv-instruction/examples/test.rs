fn main() {
    use riscv_instruction::*;
    let rd = Register::<5>::new(8).unwrap();
    let rs1 = Register::<5>::new(9).unwrap();
    let rs2 = Register::<5>::new(10).unwrap();

    let instruction = StandardISharedInstructions::ADD { rd, rs1, rs2 };
    println!("{}", instruction);
}
