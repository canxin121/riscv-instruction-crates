#[macro_export]
macro_rules! generate_assemble_march {
    () => {
        use std::collections::BTreeSet;

        /// 将收集到的扩展组件组装成最终的 march 字符串。
        fn assemble_march(
            base: &str,
            mut std_exts: BTreeSet<char>,
            other_exts: BTreeSet<String>,
        ) -> String {
            // 如果没有指定任何扩展，则返回仅包含基础整数指令集 'i' 的字符串。
            if std_exts.is_empty() && other_exts.is_empty() {
                return format!("{}i", base);
            }

            // 如果指定了任何其他扩展，则基础的 'i' 扩展是必须的。
            std_exts.insert('i');

            // 按照规范顺序（IMAFDQCV）构建标准扩展部分。
            let mut std_str = String::new();
            let canonical_order = "imafdqcv";

            for ext_char in canonical_order.chars() {
                if std_exts.remove(&ext_char) {
                    std_str.push(ext_char);
                }
            }
            // 附加任何在规范顺序之外但在集合中的标准扩展（按字母顺序）。
            for ext_char in std_exts {
                std_str.push(ext_char);
            }

            let mut result = format!("{}{}", base, std_str);

            // 附加所有其他扩展，用下划线分隔。BTreeSet 保证了它们是按字母顺序排列的。
            if !other_exts.is_empty() {
                let other_str = other_exts.into_iter().collect::<Vec<String>>().join("_");
                result.push('_');
                result.push_str(&other_str);
            }

            result
        }

        /// 根据给定的 RV32Extensions 向量构建一个 RISC-V march 字符串。
        pub fn build_rv32_march(extensions: &[RV32Extensions]) -> String {
            if extensions.is_empty() {
                return "rv32i".to_string();
            }
            let mut std_exts = BTreeSet::new();
            let mut other_exts = BTreeSet::new();

            for ext in extensions {
                match ext {
                    // 标准扩展
                    RV32Extensions::I => {
                        std_exts.insert('i');
                    }
                    RV32Extensions::M => {
                        std_exts.insert('m');
                    }
                    RV32Extensions::F => {
                        std_exts.insert('f');
                        other_exts.insert("zfa".to_string());
                    }
                    RV32Extensions::D => {
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zfa".to_string());
                    }
                    RV32Extensions::Q => {
                        std_exts.insert('q');
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zfa".to_string());
                        other_exts.insert("zfhmin".to_string());
                    }
                    RV32Extensions::C => {
                        std_exts.insert('c');
                    }
                    RV32Extensions::V => {
                        std_exts.insert('v');
                    }
                    RV32Extensions::H => {
                        other_exts.insert("h".to_string());
                    }

                    // 'B' 是多个 Zba/Zbb/Zbc/Zbs 扩展的组合
                    RV32Extensions::B => {
                        other_exts.insert("zba".to_string());
                        other_exts.insert("zbb".to_string());
                        other_exts.insert("zbc".to_string());
                        other_exts.insert("zbs".to_string());
                    }

                    // 'A' (原子) 扩展由 Za* 隐含
                    RV32Extensions::Zaamo => {
                        std_exts.insert('a');
                        other_exts.insert("zaamo".to_string());
                    }
                    RV32Extensions::Zalrsc => {
                        std_exts.insert('a');
                    } // Zalrsc 是 'A' 的一部分
                    RV32Extensions::Zacas => {
                        std_exts.insert('a');
                        other_exts.insert("zacas".to_string());
                    }
                    RV32Extensions::Zabha => {
                        std_exts.insert('a');
                        other_exts.insert("zabha".to_string());
                        other_exts.insert("zacas".to_string());
                    }

                    // 具有复杂依赖的 Zc* 扩展
                    RV32Extensions::Zcb => {
                        std_exts.insert('c');
                        std_exts.insert('m');
                        other_exts.insert("zcb".to_string());
                        other_exts.insert("zbb".to_string());
                    }
                    RV32Extensions::Zcmp => {
                        std_exts.insert('c');
                        other_exts.insert("zcmp".to_string());
                    }
                    RV32Extensions::Zcmop => {
                        std_exts.insert('c');
                        std_exts.insert('a');
                        other_exts.insert("zcmop".to_string());
                        other_exts.insert("zacas".to_string());
                    }
                    RV32Extensions::Zcd => {
                        std_exts.insert('c');
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zcd".to_string());
                    }
                    RV32Extensions::Zcf => {
                        std_exts.insert('c');
                        std_exts.insert('f');
                        other_exts.insert("zcf".to_string());
                    }

                    // 矢量 (Vector) 扩展，隐含 'V'
                    RV32Extensions::Zvbb
                    | RV32Extensions::Zvbc
                    | RV32Extensions::Zvkg
                    | RV32Extensions::Zvks
                    | RV32Extensions::Zvkned
                    | RV32Extensions::Zvknha => {
                        std_exts.insert('v');
                        other_exts.insert(format!("{:?}", ext).to_lowercase());
                    }
                    RV32Extensions::Zvfbfmin | RV32Extensions::Zvfbfwma => {
                        std_exts.insert('v');
                        std_exts.insert('f');
                        other_exts.insert(format!("{:?}", ext).to_lowercase());
                    }

                    // 根据参考逻辑，这些扩展不直接修改 march 字符串
                    RV32Extensions::S
                    | RV32Extensions::Zalasr
                    | RV32Extensions::Zilsd
                    | RV32Extensions::Smrnmi
                    | RV32Extensions::Sdext => {}

                    // 其他 'Z' 扩展
                    RV32Extensions::Zfh => {
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zfh".to_string());
                        other_exts.insert("zfa".to_string());
                    }
                    RV32Extensions::Zfbfmin => {
                        std_exts.insert('f');
                        other_exts.insert("zfbfmin".to_string());
                    }
                    RV32Extensions::Svinval => {
                        other_exts.insert("svinval".to_string());
                    }
                    RV32Extensions::Smdbltrp => {
                        other_exts.insert("smdbltrp".to_string());
                        other_exts.insert("smctr".to_string());
                    }

                    // 其他所有扩展直接将其小写名称添加
                    _ => {
                        other_exts.insert(format!("{:?}", ext).to_lowercase());
                    }
                }
            }
            assemble_march("rv32", std_exts, other_exts)
        }

        /// 根据给定的 RV64Extensions 向量构建一个 RISC-V march 字符串。
        pub fn build_rv64_march(extensions: &[RV64Extensions]) -> String {
            if extensions.is_empty() {
                return "rv64i".to_string();
            }
            let mut std_exts = BTreeSet::new();
            let mut other_exts = BTreeSet::new();

            for ext in extensions {
                match ext {
                    // 标准扩展
                    RV64Extensions::I => {
                        std_exts.insert('i');
                    }
                    RV64Extensions::M => {
                        std_exts.insert('m');
                    }
                    RV64Extensions::F => {
                        std_exts.insert('f');
                        other_exts.insert("zfa".to_string());
                    }
                    RV64Extensions::D => {
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zfa".to_string());
                    }
                    RV64Extensions::Q => {
                        std_exts.insert('q');
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zfa".to_string());
                        other_exts.insert("zfhmin".to_string());
                    }
                    RV64Extensions::C => {
                        std_exts.insert('c');
                    }
                    RV64Extensions::V => {
                        std_exts.insert('v');
                    }
                    RV64Extensions::H => {
                        other_exts.insert("h".to_string());
                    }
                    RV64Extensions::B => {
                        other_exts.insert("zba".to_string());
                        other_exts.insert("zbb".to_string());
                        other_exts.insert("zbc".to_string());
                        other_exts.insert("zbs".to_string());
                    }

                    // 'A' (原子) 扩展
                    RV64Extensions::Zaamo => {
                        std_exts.insert('a');
                        other_exts.insert("zaamo".to_string());
                    }
                    RV64Extensions::Zalrsc => {
                        std_exts.insert('a');
                    }
                    RV64Extensions::Zacas => {
                        std_exts.insert('a');
                        other_exts.insert("zacas".to_string());
                    }
                    RV64Extensions::Zabha => {
                        std_exts.insert('a');
                        other_exts.insert("zabha".to_string());
                        other_exts.insert("zacas".to_string());
                    }

                    // Zc* 扩展
                    RV64Extensions::Zcb => {
                        std_exts.insert('c');
                        std_exts.insert('m');
                        other_exts.insert("zcb".to_string());
                        other_exts.insert("zbb".to_string());
                        other_exts.insert("zba".to_string());
                    }
                    RV64Extensions::Zcmp => {
                        std_exts.insert('c');
                        other_exts.insert("zcmp".to_string());
                    }
                    RV64Extensions::Zcmop => {
                        std_exts.insert('c');
                        other_exts.insert("zcmop".to_string());
                    }
                    RV64Extensions::Zcd => {
                        std_exts.insert('c');
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zcd".to_string());
                    }
                    // Zcf 不在您的 RV64Extensions 枚举中

                    // 矢量 (Vector) 扩展
                    RV64Extensions::Zvbb
                    | RV64Extensions::Zvbc
                    | RV64Extensions::Zvkg
                    | RV64Extensions::Zvks
                    | RV64Extensions::Zvkned
                    | RV64Extensions::Zvknha => {
                        std_exts.insert('v');
                        other_exts.insert(format!("{:?}", ext).to_lowercase());
                    }
                    RV64Extensions::Zvfbfmin | RV64Extensions::Zvfbfwma => {
                        std_exts.insert('v');
                        std_exts.insert('f');
                        other_exts.insert(format!("{:?}", ext).to_lowercase());
                    }

                    // 被忽略的扩展
                    RV64Extensions::S
                    | RV64Extensions::Zalasr
                    | RV64Extensions::Zilsd
                    | RV64Extensions::Smrnmi
                    | RV64Extensions::Sdext => {}

                    // 其他 'Z' 扩展
                    RV64Extensions::Zfh => {
                        std_exts.insert('d');
                        std_exts.insert('f');
                        other_exts.insert("zfh".to_string());
                        other_exts.insert("zfa".to_string());
                    }
                    RV64Extensions::Zfbfmin => {
                        std_exts.insert('f');
                        other_exts.insert("zfbfmin".to_string());
                    }
                    RV64Extensions::Svinval => {
                        other_exts.insert("svinval".to_string());
                    }
                    RV64Extensions::Smdbltrp => {
                        other_exts.insert("smdbltrp".to_string());
                        other_exts.insert("smctr".to_string());
                    }
                    RV64Extensions::Zkn => {
                        other_exts.insert("zkn".to_string());
                    } // RV64 特有

                    // 其他所有扩展
                    _ => {
                        other_exts.insert(format!("{:?}", ext).to_lowercase());
                    }
                }
            }

            assemble_march("rv64", std_exts, other_exts)
        }
    };
}
