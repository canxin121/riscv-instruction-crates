riscv_instruction_macros::generate_riscv_instructions!("../assets/riscv_instructions.json");

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::process::Command;

    #[derive(Debug)]
    enum InstructionType {
        StandardShared,
        StandardRV64Specific,
        RVCShared,
        RVCRV32Specific,
        RVCRV64Specific,
    }

    impl InstructionType {
        fn name(&self) -> &'static str {
            match self {
                InstructionType::StandardShared => "StandardShared",
                InstructionType::StandardRV64Specific => "StandardRV64Specific",
                InstructionType::RVCShared => "RVCShared",
                InstructionType::RVCRV32Specific => "RVCRV32Specific",
                InstructionType::RVCRV64Specific => "RVCRV64Specific",
            }
        }

        fn march_flags(&self) -> Vec<&'static str> {
            match self {
                InstructionType::StandardShared => vec!["rv32gcq", "rv64gcq"],
                InstructionType::StandardRV64Specific => vec!["rv64gcq"],
                InstructionType::RVCShared => vec!["rv32gcq", "rv64gcq"],
                InstructionType::RVCRV32Specific => vec!["rv32gcq"],
                InstructionType::RVCRV64Specific => vec!["rv64gcq"],
            }
        }
    }

    fn generate_instructions(inst_type: &InstructionType, count: usize) -> Vec<String> {
        let mut rng = rand::rng();

        (0..count)
            .map(|_| match inst_type {
                InstructionType::StandardShared => {
                    StandardSharedInstruction::random_with_rng(&mut rng).to_string()
                }
                InstructionType::StandardRV64Specific => {
                    StandardRV64SpecificInstruction::random_with_rng(&mut rng).to_string()
                }
                InstructionType::RVCShared => {
                    RVCSharedInstruction::random_with_rng(&mut rng).to_string()
                }
                InstructionType::RVCRV32Specific => {
                    RVCRV32SpecificInstruction::random_with_rng(&mut rng).to_string()
                }
                InstructionType::RVCRV64Specific => {
                    RVCRV64SpecificInstruction::random_with_rng(&mut rng).to_string()
                }
            })
            .collect()
    }

    fn create_assembly_file(instructions: &[String], filename: &str) -> std::io::Result<()> {
        let mut output = std::fs::File::create(filename)?;

        writeln!(output, ".section .text")?;
        writeln!(output, ".global _start")?;
        writeln!(output, "_start:")?;

        for inst in instructions {
            writeln!(output, "    {}", inst)?;
        }

        writeln!(output, "    # Exit program")?;
        writeln!(output, "    li a0, 0")?;
        writeln!(output, "    li a7, 93")?;
        writeln!(output, "    ecall")?;

        Ok(())
    }

    fn test_assembly(filename: &str, march: &str) -> (bool, String) {
        let output = Command::new("riscv64-unknown-elf-as")
            .arg(format!("-march={}", march))
            .arg("-o")
            .arg("output.o")
            .arg(filename)
            .output();

        match output {
            Ok(result) => {
                let success = result.status.success();
                let stderr = String::from_utf8_lossy(&result.stderr);
                let stdout = String::from_utf8_lossy(&result.stdout);
                let has_error = stderr.to_lowercase().contains("error");

                let error_info = if !success || has_error {
                    format!(
                        "Exit code: {}\nSTDOUT:\n{}\nSTDERR:\n{}",
                        result.status.code().unwrap_or(-1),
                        stdout,
                        stderr
                    )
                } else {
                    String::new()
                };

                (success && !has_error, error_info)
            }
            Err(e) => (false, format!("Command execution failed: {}", e)),
        }
    }

    fn create_error_log(
        inst_type: &InstructionType,
        batch_num: usize,
        instructions: &[String],
        errors: &[(String, String)],
    ) -> std::io::Result<()> {
        // 创建错误日志目录
        std::fs::create_dir_all("error_logs")?;

        let log_filename = format!("error_logs/{}_{}_errors.log", inst_type.name(), batch_num);
        let mut log_file = std::fs::File::create(&log_filename)?;

        writeln!(log_file, "=== 错误日志 ===")?;
        writeln!(log_file, "指令集类型: {}", inst_type.name())?;
        writeln!(log_file, "批次: {}", batch_num)?;
        writeln!(
            log_file,
            "时间: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )?;
        writeln!(log_file, "指令数量: {}", instructions.len())?;
        writeln!(log_file, "")?;

        for (march, error_info) in errors {
            writeln!(log_file, "=== -march={} 失败详情 ===", march)?;
            writeln!(log_file, "{}", error_info)?;
            writeln!(log_file, "")?;
        }

        writeln!(log_file, "=== 生成的指令 ===")?;
        for (i, inst) in instructions.iter().enumerate() {
            writeln!(log_file, "{:4}: {}", i + 1, inst)?;
        }

        // 同时保存汇编文件到错误日志目录
        let asm_filename = format!("error_logs/{}_{}.S", inst_type.name(), batch_num);
        create_assembly_file(instructions, &asm_filename)?;

        println!("    错误日志已保存到: {}", log_filename);
        println!("    汇编文件已保存到: {}", asm_filename);

        Ok(())
    }

    fn run_test_batch(inst_type: &InstructionType, batch_num: usize) -> (usize, usize) {
        println!(
            "  批次 {}: 生成 10000 个 {} 指令...",
            batch_num,
            inst_type.name()
        );

        let instructions = generate_instructions(&inst_type, 10000);
        let filename = format!("test_{}_{}.S", inst_type.name(), batch_num);

        if let Err(e) = create_assembly_file(&instructions, &filename) {
            println!("    错误: 创建汇编文件失败: {}", e);
            return (0, 0);
        }

        let mut success_count = 0;
        let total_tests = inst_type.march_flags().len();
        let mut errors = Vec::new();

        for march in inst_type.march_flags() {
            print!("    测试 -march={} ... ", march);
            let (success, error_info) = test_assembly(&filename, march);
            if success {
                println!("✓ 成功");
                success_count += 1;
            } else {
                println!("✗ 失败");
                errors.push((march.to_string(), error_info));
            }
        }

        // 如果有失败的测试，生成错误日志
        if !errors.is_empty() {
            if let Err(e) = create_error_log(inst_type, batch_num, &instructions, &errors) {
                println!("    警告: 无法创建错误日志: {}", e);
            }
        }

        // 清理临时文件
        let _ = std::fs::remove_file(&filename);
        let _ = std::fs::remove_file("output.o");

        (success_count, total_tests)
    }

    #[test]
    fn test_all() {
        println!("开始 RISC-V 指令集自动化测试");
        println!("每种指令集测试 100 次，每次生成 10000 个随机指令\n");

        // 清理之前的错误日志目录
        let _ = std::fs::remove_dir_all("error_logs");

        let instruction_types = vec![
            InstructionType::StandardShared,
            InstructionType::StandardRV64Specific,
            InstructionType::RVCShared,
            InstructionType::RVCRV32Specific,
            InstructionType::RVCRV64Specific,
        ];

        let mut total_success = 0;
        let mut total_tests = 0;

        for inst_type in instruction_types {
            println!("测试 {} 指令集:", inst_type.name());

            let mut type_success = 0;
            let mut type_total = 0;

            for batch in 1..=100 {
                let (success, tests) = run_test_batch(&inst_type, batch);
                type_success += success;
                type_total += tests;
            }

            println!(
                "  {} 总结: {}/{} 测试通过\n",
                inst_type.name(),
                type_success,
                type_total
            );

            total_success += type_success;
            total_tests += type_total;
        }

        println!("=== 测试完成 ===");
        println!("总体结果: {}/{} 测试通过", total_success, total_tests);

        let test_passed = total_success == total_tests;
        
        println!("\n清理测试文件...");

        if test_passed {
            println!("🎉 所有测试都通过了！");
            // 清理错误日志目录
            if let Err(e) = std::fs::remove_dir_all("error_logs") {
                if e.kind() != std::io::ErrorKind::NotFound {
                    println!("警告: 无法删除错误日志目录: {}", e);
                }
            } else {
                println!("✓ 错误日志目录已清理");
            }
        } else {
            println!("⚠️  有 {} 个测试失败", total_tests - total_success);
            println!("错误日志已保存在 'error_logs' 目录中，请查看详细信息。");
        }


        // 清理可能残留的临时文件
        let temp_files = ["output.o"];
        for file in &temp_files {
            if let Err(e) = std::fs::remove_file(file) {
                if e.kind() != std::io::ErrorKind::NotFound {
                    println!("警告: 无法删除临时文件 {}: {}", file, e);
                }
            }
        }

        // 清理所有可能的测试汇编文件模式
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("test_") && filename.ends_with(".S") {
                        if let Err(e) = std::fs::remove_file(filename) {
                            println!("警告: 无法删除汇编文件 {}: {}", filename, e);
                        }
                    }
                }
            }
        }

        println!("✓ 测试文件清理完成");
        
        // 如果有失败的测试，让整个测试失败
        assert!(test_passed, "有 {} 个测试失败，详细信息请查看 'error_logs' 目录中的日志文件", total_tests - total_success);
    }
}
