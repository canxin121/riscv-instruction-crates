use std::collections::HashSet;

/// 从汇编语法中提取操作数名称
pub fn extract_operands_from_asm_without_name(format: &str) -> HashSet<String> {
    if format.is_empty() {
        return HashSet::new(); // 如果格式或指令名称为空，返回空集合
    }

    let mut operands = HashSet::new();

    // 移除指令名称前缀的改进逻辑
    let operands_part_str = format;

    // 如果移除指令名称后没有剩余内容，说明是无操作数指令
    if operands_part_str.is_empty() {
        return operands;
    }

    // 方法1: 匹配大括号内的操作数，如 {rd}, {rs1}, {imm} 等
    // 这一步对于如 "{xd}, {xs1}, {xs2}" 这样的操作数部分非常有效
    operands.extend(extract_braced_operands(&operands_part_str));

    // 方法2: 使用正则表达式模式匹配操作数 (如果方法1未完全提取或格式不同)
    // 仅当大括号提取未能充分提取时，或操作数部分不使用大括号时，此方法才更有可能添加新操作数
    if operands.is_empty() || !operands_part_str.contains('{') {
        // 优化：如果已有大括号操作数，可能不需要正则
        operands.extend(extract_operands_regex(&operands_part_str));
    }

    // 方法3: 如果前面方法都没找到，尝试简单的单词匹配
    if operands.is_empty() {
        operands.extend(extract_operands_simple(&operands_part_str));
    }

    operands
}

fn extract_braced_operands(format_part: &str) -> HashSet<String> {
    // Renamed parameter for clarity
    let mut operands = HashSet::new();
    let mut chars = format_part.chars().peekable(); // Use format_part

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut operand = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    chars.next(); // 消费 '}'
                    break;
                } else {
                    operand.push(chars.next().unwrap());
                }
            }
            if !operand.is_empty() {
                operands.insert(operand);
            }
        }
    }

    operands
}

fn extract_operands_regex(format_part: &str) -> HashSet<String> {
    // Renamed parameter
    let mut operands = HashSet::new();

    // 分析汇编语法字符串，提取可能的操作数
    // 处理多种格式：
    // 1. 空格分隔的操作数: "rd, rs1, imm"
    // 2. 大括号格式: "{rd}, {rs1}, {imm}" (主要由 extract_braced_operands 处理)
    // 3. 混合格式: "rd, {rs1}, imm"
    // 4. 内存访问格式: "imm(rs1)", "offset(base)"

    // 特殊处理：内存访问格式 imm(rs1)
    extract_memory_access_operands(format_part, &mut operands); // Use format_part

    // 首先处理逗号分隔的操作数
    for part in format_part.split(',') {
        // Use format_part
        let cleaned = part.trim();

        // 跳过包含括号的部分，这些已经在内存访问处理中处理过了
        if cleaned.contains('(') && cleaned.contains(')') {
            continue;
        }

        let final_cleaned = cleaned
            .trim_matches(|c: char| "{}[]<>".contains(c)) // 移除大括号等，但保留括号用于内存访问检测
            .trim()
            .trim_matches('-'); // 移除前导的负号（如 -stack_adj）

        if is_likely_operand(final_cleaned) {
            operands.insert(final_cleaned.to_string());
        }

        // 特殊处理：强制处理带负号的操作数，如 -stack_adj, -spimm
        if cleaned.starts_with('-') {
            let without_minus = cleaned.trim_start_matches('-');
            if is_likely_operand(without_minus) {
                operands.insert(without_minus.to_string());
            }
        }
    }

    // 然后处理空格分隔的操作数
    for word in format_part.split_whitespace() {
        // Use format_part
        // 跳过包含括号的部分
        if word.contains('(') && word.contains(')') {
            continue;
        }

        let cleaned = word
            .trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '+')
            .trim_matches('-'); // 移除前导的负号

        if is_likely_operand(cleaned) {
            operands.insert(cleaned.to_string());
        }

        // 特殊处理：带负号的操作数，如 -spimm, -stack_adj
        if word.starts_with('-') {
            let without_minus = word
                .trim_start_matches('-')
                .trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '+');
            if is_likely_operand(without_minus) {
                operands.insert(without_minus.to_string());
            }
        }
    }

    // 特殊处理：对于cm.push和cm.pop指令，强制查找stack_adj
    if (format_part.contains("cm.push") || format_part.contains("cm.pop"))
        && format_part.contains("stack_adj")
    {
        // Use format_part
        operands.insert("stack_adj".to_string());
    }

    // 特殊处理：查找带索引的操作数 (如 rs1+1, vs2+1)
    extract_indexed_operands_from_format(format_part, &mut operands); // Use format_part

    operands
}

fn extract_memory_access_operands(format_part: &str, operands: &mut HashSet<String>) {
    // Renamed parameter
    // 查找内存访问模式: imm(rs1), offset(base), 等
    // 模式：操作数(寄存器)

    let mut i = 0;
    let chars: Vec<char> = format_part.chars().collect(); // Use format_part

    while i < chars.len() {
        if chars[i] == '(' {
            // 找到开括号，向前查找立即数
            let mut start = i;
            while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
                start -= 1;
            }

            // 提取立即数操作数
            if start < i {
                let imm_operand: String = chars[start..i].iter().collect();
                let imm_cleaned = imm_operand.trim();
                if is_likely_operand(imm_cleaned) {
                    operands.insert(imm_cleaned.to_string());
                }
            }

            // 提取括号内的寄存器
            let reg_start = i + 1;
            let mut reg_end = reg_start;
            while reg_end < chars.len() && chars[reg_end] != ')' {
                reg_end += 1;
            }

            if reg_end > reg_start && reg_end < chars.len() {
                let reg_operand: String = chars[reg_start..reg_end].iter().collect();
                let reg_cleaned = reg_operand.trim();
                if is_likely_operand(reg_cleaned) {
                    operands.insert(reg_cleaned.to_string());
                }
                i = reg_end; // 跳过已处理的部分
            }
        }
        i += 1;
    }
}

fn extract_indexed_operands_from_format(format_part: &str, operands: &mut HashSet<String>) {
    // Renamed parameter
    // 查找类似 "rs1+1", "vs2+1", "fs1+1" 等模式
    let indexed_patterns = [
        "rs1+1", "rs2+1", "rs3+1", "vs1+1", "vs2+1", "vs3+1", "fs1+1", "fs2+1", "fs3+1", "xs1+1",
        "xs2+1", "xs3+1",
    ];

    for pattern in &indexed_patterns {
        if format_part.contains(pattern) {
            // Use format_part
            // 提取基础寄存器名
            if let Some(base_reg) = pattern.split('+').next() {
                operands.insert(base_reg.to_string());
            }
        }
    }

    // 查找其他可能的索引模式，如 "rd+rs1"
    for word in format_part.split_whitespace() {
        // Use format_part
        if word.contains('+') && !word.starts_with(|c: char| c.is_ascii_digit()) {
            // 分解包含'+'的词，提取可能的寄存器名
            for part in word.split('+') {
                let cleaned = part.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if is_likely_operand(cleaned) {
                    operands.insert(cleaned.to_string());
                }
            }
        }
    }
}

fn extract_operands_simple(format_part: &str) -> HashSet<String> {
    // Renamed parameter
    let mut operands = HashSet::new();

    // 扩展的操作数模式，包括更多变体
    let common_operands = [
        // 通用寄存器
        "rd", "rs1", "rs2", "rs3", // 向量寄存器
        "vd", "vs1", "vs2", "vs3", "vm", // 浮点寄存器
        "fd", "fs1", "fs2", "fs3", // 四精度浮点寄存器
        "qd", "qs1", "qs2", "qs3", // 密码学扩展操作数
        "xd", "xs1", "xs2", "xs3", // 立即数变体
        "imm", "uimm", "simm", "shamt", "zimm", "zimm5", "zimm10", "zimm11",
        // 其他操作数
        "aq", "rl", "rm", "pred", "succ", "offset", "csr", "nf",
        // 压缩指令特有操作数
        "c_rd", "c_rs1", "c_rs2", "c_imm", // 原子指令操作数
        "ordering",
    ];

    for &operand in &common_operands {
        if is_word_boundary(format_part, operand) {
            // Use format_part
            operands.insert(operand.to_string());
        }
    }

    // 特殊处理：查找模式化的操作数（如 rs1+1, vs2+1 等）
    extract_indexed_operands(format_part, &mut operands); // Use format_part

    operands
}

fn extract_indexed_operands(format_part: &str, operands: &mut HashSet<String>) {
    // Renamed parameter
    // 查找类似 "rs1+1", "vs2+1" 等模式
    let patterns = ["rs1+1", "vs2+1", "fs1+1"];

    for pattern in &patterns {
        if format_part.contains(pattern) {
            // Use format_part
            // 提取基础寄存器名
            let base_reg = pattern.split('+').next().unwrap();
            operands.insert(base_reg.to_string());
        }
    }
}

fn is_word_boundary(text: &str, word: &str) -> bool {
    if let Some(start) = text.find(word) {
        let end = start + word.len();

        // 检查前面的字符
        let before_ok = start == 0 || !text.chars().nth(start - 1).unwrap_or(' ').is_alphanumeric();

        // 检查后面的字符
        let after_ok = end >= text.len() || !text.chars().nth(end).unwrap_or(' ').is_alphanumeric();

        before_ok && after_ok
    } else {
        false
    }
}

fn is_likely_operand(word: &str) -> bool {
    if word.is_empty() || word.len() > 15 {
        // 增加长度限制，因为有些操作数较长
        return false;
    }

    // 检查是否匹配常见的操作数模式
    let common_patterns = [
        // 寄存器模式
        |s: &str| s.starts_with("rd") || s.starts_with("rs") || s.starts_with("rt"),
        |s: &str| s.starts_with("fd") || s.starts_with("fs") || s.starts_with("ft"),
        |s: &str| s.starts_with("vd") || s.starts_with("vs") || s.starts_with("vt"),
        |s: &str| s.starts_with("xd") || s.starts_with("xs"),
        |s: &str| s.starts_with("hd") || s.starts_with("hs"),
        |s: &str| s.starts_with("qd") || s.starts_with("qs"),
        |s: &str| s.starts_with("dd"),
        // 立即数模式
        |s: &str| s.contains("imm"),
        |s: &str| s.starts_with("shamt"),
        |s: &str| s.starts_with("zimm") || s.starts_with("uimm") || s.starts_with("simm"),
        |s: &str| s == "offset" || s.starts_with("csr"),
        // 特殊操作数
        |s: &str| s == "aq" || s == "rl" || s == "rm" || s == "vm",
        |s: &str| s == "pred" || s == "succ" || s == "nf",
        // 压缩指令特有操作数
        |s: &str| s == "reg_list" || s == "stack_adj", // cm.push/cm.pop 操作数
        |s: &str| s == "rlist" || s == "spimm",        // cm.push/cm.pop 内部操作数名称
        // 其他常见操作数
        |s: &str| s == "ordering" || s == "fence" || s == "fm",
        |s: &str| {
            s.starts_with("c_") && (s.contains("rd") || s.contains("rs") || s.contains("imm"))
        },
        // 向量操作数
        |s: &str| {
            s.starts_with("v") && (s.len() >= 2) && s[1..].chars().all(|c| c.is_alphanumeric())
        },
        // 浮点相关
        |s: &str| {
            s.starts_with("f") && s.len() >= 2 && s.chars().skip(1).all(|c| c.is_alphanumeric())
        },
        // 通用模式：包含下划线的标识符（很可能是操作数）
        |s: &str| s.contains('_') && s.chars().all(|c| c.is_alphanumeric() || c == '_'),
        // 简单的字母数字组合（至少2个字符，避免误匹配单个字母）
        |s: &str| {
            s.len() >= 2
                && s.chars().all(|c| c.is_alphanumeric())
                && s.chars().any(|c| c.is_alphabetic())
        },
    ];

    common_patterns.iter().any(|pattern| pattern(word))
}
