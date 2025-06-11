pub mod march_util;

pub mod merged_instructions {
    use crate::generate_assemble_march;
    riscv_instruction_macros::generate_merged_riscv_instructions!(
        "../assets/riscv_instructions.json"
    );
    generate_assemble_march!();
}

pub mod separated_instructions {
    use crate::generate_assemble_march;
    riscv_instruction_macros::generate_separated_riscv_instructions!(
        "../assets/riscv_instructions.json"
    );
    generate_assemble_march!();
}

#[cfg(test)]
mod test {
    use super::separated_instructions::*;
    use enum_iterator::all;
    use std::process::Command;

    fn generate_rv32_instructions(extension: RV32Extensions, count: usize) -> Vec<String> {
        let mut rng = rand::rng();
        (0..count)
            .map(|_| extension.random_instruction(&mut rng).to_string())
            .collect()
    }

    fn generate_rv64_instructions(extension: RV64Extensions, count: usize) -> Vec<String> {
        let mut rng = rand::rng();
        (0..count)
            .map(|_| extension.random_instruction(&mut rng).to_string())
            .collect()
    }

    fn create_assembly_file(instructions: &[String], filename: &str) -> std::io::Result<()> {
        // 确保先删除可能存在的同名文件
        if std::path::Path::new(filename).exists() {
            std::fs::remove_file(filename)?;
        }

        // 在内存中构造完整的汇编文件内容
        let mut content = String::new();

        content.push_str(".section .text\n");
        content.push_str(".global _start\n");
        content.push_str("_start:\n");

        for inst in instructions {
            content.push_str(&format!("    {}\n", inst));
        }

        content.push_str("    # Exit program\n");
        content.push_str("    li a0, 0\n");
        content.push_str("    li a7, 93\n");
        content.push_str("    ecall\n");

        // 多次尝试写入文件
        for attempt in 1..=5 {
            match std::fs::write(filename, &content) {
                Ok(_) => {
                    // 写入成功后，等待一小段时间确保文件系统同步
                    std::thread::sleep(std::time::Duration::from_millis(10));

                    // 验证文件确实存在且可读
                    if std::path::Path::new(filename).exists() {
                        match std::fs::read_to_string(filename) {
                            Ok(read_content) => {
                                if read_content.len() == content.len() {
                                    return Ok(());
                                } else {
                                    return Err(std::io::Error::new(
                                        std::io::ErrorKind::InvalidData,
                                        format!(
                                            "文件内容长度不匹配: 期望 {}, 实际 {}",
                                            content.len(),
                                            read_content.len()
                                        ),
                                    ));
                                }
                            }
                            Err(e) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::PermissionDenied,
                                    format!("无法读取刚创建的文件: {}", e),
                                ));
                            }
                        }
                    } else {
                        if attempt < 5 {
                            println!("    尝试 {}: 文件写入后不存在，重试...", attempt);
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            continue;
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                format!("文件创建后不存在: {} (尝试 {})", filename, attempt),
                            ));
                        }
                    }
                }
                Err(e) => {
                    if attempt < 5 {
                        println!("    尝试 {}: 写入失败 {}, 重试...", attempt, e);
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        continue;
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("文件写入失败 (尝试 {}): {}", attempt, e),
                        ));
                    }
                }
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "所有文件创建尝试都失败了",
        ))
    }

    fn test_assembly(filename: &str, march: &str) -> (bool, String) {
        // 在测试前再次验证输入文件存在
        for check_attempt in 1..=3 {
            if std::path::Path::new(filename).exists() {
                break;
            } else {
                if check_attempt < 3 {
                    println!("    检查 {}: 文件不存在，等待...", check_attempt);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                } else {
                    return (false, format!("输入文件在所有检查后仍不存在: {}", filename));
                }
            }
        }

        // 尝试读取文件以确保权限正确和内容完整
        match std::fs::File::open(filename) {
            Ok(_) => {
                // 文件可以打开，继续验证内容
                match std::fs::read_to_string(filename) {
                    Ok(content) => {
                        if content.is_empty() {
                            return (false, format!("输入文件 {} 为空", filename));
                        }
                        if !content.contains(".section .text") {
                            return (false, format!("输入文件 {} 内容格式不正确", filename));
                        }
                    }
                    Err(e) => {
                        return (false, format!("无法读取输入文件 {}: {}", filename, e));
                    }
                }
            }
            Err(e) => {
                return (false, format!("无法打开输入文件 {}: {}", filename, e));
            }
        }

        // 获取文件的绝对路径
        let abs_path = match std::fs::canonicalize(filename) {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => filename.to_string(),
        };

        // 清理可能存在的输出文件
        let _ = std::fs::remove_file("output.o");

        let output = Command::new("riscv64-unknown-elf-as")
            .arg(format!("-march={}", march))
            .arg("-o")
            .arg("output.o")
            .arg(&abs_path)
            .output();

        match output {
            Ok(result) => {
                let success = result.status.success();
                let stderr = String::from_utf8_lossy(&result.stderr);
                let stdout = String::from_utf8_lossy(&result.stdout);
                let has_error = stderr.to_lowercase().contains("error");

                let error_info = if !success || has_error {
                    format!(
                        "Exit code: {}\nCommand: riscv64-unknown-elf-as -march={} -o output.o {}\nSTDOUT:\n{}\nSTDERR:\n{}",
                        result.status.code().unwrap_or(-1),
                        march,
                        abs_path,
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
        extension_name: &str,
        instructions: &[String],
        march: &str,
        error_info: &str,
    ) -> std::io::Result<()> {
        // 创建错误日志目录
        std::fs::create_dir_all("error_logs")?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let log_filename = format!("error_logs/{}_{}_errors.log", extension_name, timestamp);

        // 在内存中构造完整的日志内容
        let mut log_content = String::new();

        log_content.push_str("=== 错误日志 ===\n");
        log_content.push_str(&format!("指令集扩展: {}\n", extension_name));
        log_content.push_str(&format!("MARCH: {}\n", march));
        log_content.push_str(&format!("时间: {}\n", timestamp));
        log_content.push_str(&format!("指令数量: {}\n", instructions.len()));
        log_content.push_str("\n");

        log_content.push_str("=== 错误详情 ===\n");
        log_content.push_str(error_info);
        log_content.push_str("\n\n");

        log_content.push_str("=== 生成的指令 ===\n");
        for (i, inst) in instructions.iter().enumerate() {
            log_content.push_str(&format!("{:4}: {}\n", i + 1, inst));
        }

        // 一次性写入完整日志内容
        std::fs::write(&log_filename, log_content)?;

        // 同时保存汇编文件到错误日志目录
        let asm_filename = format!("error_logs/{}_{}.S", extension_name, timestamp);
        create_assembly_file(instructions, &asm_filename)?;

        println!("    错误日志已保存到: {}", log_filename);
        println!("    汇编文件已保存到: {}", asm_filename);

        Ok(())
    }

    #[test]
    fn test_all_separated_instructions() {
        println!("开始测试所有分离版本的 RISC-V 指令集扩展");

        // 清理可能残留的临时文件和错误日志目录
        let _ = std::fs::remove_file("output.o");
        if let Err(e) = std::fs::remove_dir_all("error_logs") {
            if e.kind() != std::io::ErrorKind::NotFound {
                println!("警告: 无法删除之前的错误日志目录: {}", e);
            }
        }

        let mut total_success = 0;
        let mut total_tests = 0;
        let mut failed_cases = Vec::new();

        // 测试所有 RV32 扩展
        println!("=== 测试 RV32 扩展 ===");
        for extension in all::<RV32Extensions>() {
            // 跳过不支持的 RV32 扩展
            match extension {
                RV32Extensions::Zalasr | RV32Extensions::Zilsd | RV32Extensions::Smrnmi => {
                    let extension_name = format!("RV32_{:?}", extension);
                    println!("跳过 {}: GNU 工具链不支持", extension_name);
                    continue;
                }
                _ => {}
            }

            let extension_name = format!("RV32_{:?}", extension);
            println!("测试 {}:", extension_name);

            let instructions = generate_rv32_instructions(extension, 1000);
            let filename = format!("test_{}.S", extension_name);

            // 确保之前的同名文件被删除
            let _ = std::fs::remove_file(&filename);

            // 创建汇编文件
            match create_assembly_file(&instructions, &filename) {
                Ok(_) => {
                    println!("  ✓ 汇编文件 {} 创建成功", filename);
                }
                Err(e) => {
                    println!("  ✗ 错误: 无法创建汇编文件 {}: {}", filename, e);
                    continue;
                }
            }

            // 最终验证文件存在
            if !std::path::Path::new(&filename).exists() {
                println!("  ✗ 错误: 汇编文件 {} 创建后不存在", filename);
                continue;
            }

            // 构建 march 字符串并测试
            let march = build_rv32_march(&vec![extension]);
            total_tests += 1;

            print!("  测试 -march={} ... ", march);
            let (success, error_info) = test_assembly(&filename, &march);
            if success {
                println!("✓ 成功");
                total_success += 1;
            } else {
                println!("✗ 失败");
                if !error_info.is_empty() {
                    println!("    === 详细错误信息 ===");
                    for line in error_info.lines() {
                        println!("    {}", line);
                    }
                    println!("    === 错误信息结束 ===");
                    
                    // 创建错误日志
                    if let Err(e) = create_error_log(&extension_name, &instructions, &march, &error_info) {
                        println!("    警告: 无法创建错误日志: {}", e);
                    }
                }
                failed_cases.push(extension_name.clone());
            }

            println!("  {} 测试完成\n", extension_name);

            // 清理临时文件
            if let Err(e) = std::fs::remove_file(&filename) {
                println!("  警告: 无法删除临时文件 {}: {}", filename, e);
            }
            let _ = std::fs::remove_file("output.o");
        }

        // 测试所有 RV64 扩展
        println!("=== 测试 RV64 扩展 ===");
        for extension in all::<RV64Extensions>() {
            // 跳过不支持的 RV64 扩展
            match extension {
                RV64Extensions::S | RV64Extensions::Zalasr | RV64Extensions::Zilsd |
                RV64Extensions::Smrnmi | RV64Extensions::Sdext => {
                    let extension_name = format!("RV64_{:?}", extension);
                    println!("跳过 {}: GNU 工具链不支持", extension_name);
                    continue;
                }
                _ => {}
            }

            let extension_name = format!("RV64_{:?}", extension);
            println!("测试 {}:", extension_name);

            let instructions = generate_rv64_instructions(extension, 1000);
            let filename = format!("test_{}.S", extension_name);

            // 确保之前的同名文件被删除
            let _ = std::fs::remove_file(&filename);

            // 创建汇编文件
            match create_assembly_file(&instructions, &filename) {
                Ok(_) => {
                    println!("  ✓ 汇编文件 {} 创建成功", filename);
                }
                Err(e) => {
                    println!("  ✗ 错误: 无法创建汇编文件 {}: {}", filename, e);
                    continue;
                }
            }

            // 最终验证文件存在
            if !std::path::Path::new(&filename).exists() {
                println!("  ✗ 错误: 汇编文件 {} 创建后不存在", filename);
                continue;
            }

            // 构建 march 字符串并测试
            let march = build_rv64_march(&vec![extension]);
            total_tests += 1;

            print!("  测试 -march={} ... ", march);
            let (success, error_info) = test_assembly(&filename, &march);
            if success {
                println!("✓ 成功");
                total_success += 1;
            } else {
                println!("✗ 失败");
                if !error_info.is_empty() {
                    println!("    === 详细错误信息 ===");
                    for line in error_info.lines() {
                        println!("    {}", line);
                    }
                    println!("    === 错误信息结束 ===");
                    
                    // 创建错误日志
                    if let Err(e) = create_error_log(&extension_name, &instructions, &march, &error_info) {
                        println!("    警告: 无法创建错误日志: {}", e);
                    }
                }
                failed_cases.push(extension_name.clone());
            }

            println!("  {} 测试完成\n", extension_name);

            // 清理临时文件
            if let Err(e) = std::fs::remove_file(&filename) {
                println!("  警告: 无法删除临时文件 {}: {}", filename, e);
            }
            let _ = std::fs::remove_file("output.o");
        }

        println!("=== 测试完成 ===");
        println!("总体结果: {}/{} 测试通过", total_success, total_tests);

        let success_rate = if total_tests > 0 {
            (total_success as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };
        println!("成功率: {:.1}%", success_rate);

        // 清理可能残留的临时文件
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("test_") && filename.ends_with(".S") {
                        let _ = std::fs::remove_file(filename);
                    }
                }
            }
        }
        let _ = std::fs::remove_file("output.o");

        // 如果有失败的测试用例，执行 panic
        if !failed_cases.is_empty() {
            println!("\n=== 测试失败总结 ===");
            println!("失败的测试用例数量: {}", failed_cases.len());
            for (i, case) in failed_cases.iter().enumerate() {
                println!("  {}. {}", i + 1, case);
            }
            println!("错误日志已保存在 'error_logs' 目录中");
            println!("详细错误信息已在上方控制台输出中显示");
            panic!("测试失败！有 {} 个测试用例失败", failed_cases.len());
        } else {
            println!("🎉 所有测试通过！");
        }
    }
}
