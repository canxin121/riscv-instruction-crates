# RISC-V 指令库 (RISC-V Instruction Crates)

一个强类型、类型安全的 RISC-V 指令汇编与操作库。本项目致力于提供一个全面的解决方案，用于生成、解析和操作 RISC-V 指令。通过过程宏自动从 `riscv-unified-db` YAML 定义生成指令集代码，确保了指令的准确性和完整性。支持 RV32 和 RV64 架构，覆盖了广泛的已批准和部分草案阶段的指令集扩展。

## 📋 目录

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [RISC-V 指令库 (RISC-V Instruction Crates)](#risc-v-指令库-risc-v-instruction-crates)
  - [📋 目录](#-目录)
  - [✨ 功能特性](#-功能特性)
    - [🔒 类型安全](#-类型安全)
    - [🎯 广泛的指令集支持](#-广泛的指令集支持)
    - [🎲 强大的随机生成与测试](#-强大的随机生成与测试)
    - [📝 便捷的汇编生成](#-便捷的汇编生成)
  - [🎯 支持的指令集](#-支持的指令集)
    - [架构支持](#架构支持)
  - [🚀 快速开始](#-快速开始)
    - [安装](#安装)
    - [基础使用](#基础使用)
    - [随机指令生成](#随机指令生成)
  - [📁 项目结构](#-项目结构)
    - [📦 子包说明](#-子包说明)
      - [`riscv-instruction`](#riscv-instruction)
      - [`riscv-instruction-types`](#riscv-instruction-types)
      - [`riscv-instruction-macros`](#riscv-instruction-macros)
      - [`riscv-instruction-parser`](#riscv-instruction-parser)
  - [🛠️ 更新生成的资源文件](#️-更新生成的资源文件)
    - [更新步骤](#更新步骤)
  - [🧪 测试](#-测试)
    - [测试要求](#测试要求)
    - [汇编器兼容性测试](#汇编器兼容性测试)
  - [📄 许可证](#-许可证)

<!-- /code_chunk_output -->




## ✨ 功能特性

### 🔒 类型安全
- **强类型检查**: 所有寄存器（整数、浮点、向量等）和立即数都拥有严格的类型定义和验证，在编译期捕捉错误。
- **参数约束**: 自动校验寄存器编号、立即数范围、操作数限制（如非零、倍数、奇偶性、禁用值）等。

### 🎯 广泛的指令集支持
- **多架构**: 全面支持 RV32 和 RV64 架构。
- **标准与压缩指令**: 包括所有基础整数指令 (I)、标准扩展 (M, F, D, Q, C, B, V, H, S) 以及众多 Z* 和 S* 系列扩展。
- **灵活的指令模块**:
    - `merged_instructions`: 提供统一的指令视图。共享指令（跨 RV32/RV64 相同）通过 `SharedInstruction` 枚举表示，特定于架构的指令通过 `SpecificInstruction`（内含 `RV32SpecificInstruction` 和 `RV64SpecificInstruction`）表示。顶层枚举为 `RiscvInstruction { Shared(SharedInstruction), Specific(SpecificInstruction) }`。
    - `separated_instructions`: 为 RV32 和 RV64 提供完全分离的指令集视图。顶层枚举为 `RiscvInstruction { RV32(RV32Instruction), RV64(RV64Instruction) }`，其中 `RV32Instruction` 和 `RV64Instruction` 分别包含对应架构的所有扩展指令。
- **核心代码由JSON驱动生成**:
    - 指令的原始定义来源于 RISC-V International 的 `riscv-unified-db` 项目中的官方 YAML 文件。
    - 这些 YAML 文件经过 `riscv-instruction-parser` 工具（本项目的一部分）处理后，生成 [`assets/riscv_instructions.json`](assets/riscv_instructions.json) 文件。
    - **最重要的一点是：`riscv-instruction` 主库中所有指令的枚举、结构体及其核心实现，均由 `riscv-instruction-macros` 子包中的过程宏在编译时根据 [`assets/riscv_instructions.json`](assets/riscv_instructions.json) 的内容自动生成。这意味着 [`assets/riscv_instructions.json`](assets/riscv_instructions.json) 文件直接决定了库提供的指令集和功能。**


### 🎲 强大的随机生成与测试
- **约束感知随机生成**: 内建对所有指令和操作数类型的随机生成功能，严格遵守其类型、范围和约束。
- **可重现性**: 支持使用种子控制随机数生成过程，便于调试和复现问题。
- **汇编器兼容性测试**: 通过生成大量随机指令并使用 GNU RISC-V 工具链 (riscv64-unknown-elf-as)进行汇编验证，确保生成的汇编代码的正确性和兼容性。绝大多数已实现的扩展都经过了此测试。

### 📝 便捷的汇编生成
- **标准汇编输出**: 所有指令类型均实现了 `Display` trait，可输出符合标准的 RISC-V 汇编语法。
- **可配置格式**: 部分指令支持自定义汇编输出格式。
- **兼容性**: 生成的汇编与主流 RISC-V 汇编器兼容。

## 🎯 支持的指令集

本库支持广泛的 RISC-V 指令集扩展。下表列出了主要支持的扩展及其通过 RISC-V GNU 工具链的测试情况。

-   **支持状态**: ✅ 表示该扩展已在本库中实现。
-   **GNU 工具链测试**:
    -   ✅: 该扩展的指令已通过 `riscv64-unknown-elf-as` 汇编器兼容性测试。
    -   ⚠️: 由于当前 `riscv64-unknown-elf-as` 工具链对这些指令的支持不完整或存在已知问题，这些扩展的部分或全部指令未进行汇编测试，但已在库中实现。
    -   RV32/RV64: 表示测试主要针对特定架构。

| 扩展               | 描述                                     | 支持状态 | GNU 工具链测试 | 备注 (来自测试配置)                              |
| :----------------- | :--------------------------------------- | :------- | :------------- | :----------------------------------------------- |
| **基本与标准扩展** |                                          |          |                |                                                  |
| I                  | 基本整数指令集                           | ✅        | ✅              | `rv32i`, `rv64i`                                 |
| M                  | 乘法和除法扩展                           | ✅        | ✅              | `rv32im`, `rv64im`                               |
| F                  | 单精度浮点扩展                           | ✅        | ✅              | `rv32if_zfa`, `rv64if_zfa`                       |
| D                  | 双精度浮点扩展                           | ✅        | ✅              | `rv32ifd_zfa`, `rv64ifd_zfa`                     |
| Q                  | 四精度浮点扩展                           | ✅        | ✅              | `rv32ifdq_zfa_zfhmin`, `rv64ifdq_zfa_zfhmin`     |
| C                  | 压缩指令扩展                             | ✅        | ✅              | `rv32ic`, `rv64ic`                               |
| B                  | 位操作扩展 (作为 Zba/Zbb/Zbc/Zbs 的集合) | ✅        | ✅              | `rv32i_zba_zbb_zbc_zbs`, `rv64i_zba_zbb_zbc_zbs` |
| V                  | 向量扩展                                 | ✅        | ✅              | `rv32iv`, `rv64iv`                               |
| H                  | Hypervisor 扩展                          | ✅        | ✅              | `rv32i_h`, `rv64i_h`                             |
| S                  | 特权架构扩展                             | ✅        | ✅              | `rv32i`, `rv64i` (S 扩展隐式包含)                |
| **Z* 系列扩展**    |                                          |          |                |                                                  |
| Zfh                | 半精度浮点扩展                           | ✅        | ✅              | `rv32ifd_zfh_zfa`, `rv64ifd_zfh_zfa`             |
| Zicsr              | CSR 操作扩展                             | ✅        | ✅              | `rv32i_zicsr`, `rv64i_zicsr`                     |
| Zifencei           | 指令流同步扩展                           | ✅        | ✅              | `rv32i_zifencei`, `rv64i_zifencei`               |
| Zba                | 地址生成位操作扩展                       | ✅        | ✅              | `rv32i_zba`, `rv64i_zba`                         |
| Zbb                | 基本位操作扩展                           | ✅        | ✅              | `rv32i_zbb`, `rv64i_zbb`                         |
| Zbc                | 进位位操作扩展                           | ✅        | ✅              | `rv32i_zbc`, `rv64i_zbc`                         |
| Zbs                | 单位位操作扩展                           | ✅        | ✅              | `rv32i_zbs`, `rv64i_zbs`                         |
| Zbkb               | 位操作加密扩展 (基本)                    | ✅        | ✅              | `rv32i_zbkb`, `rv64i_zbkb`                       |
| Zbkx               | 位操作加密扩展 (交叉)                    | ✅        | ✅              | `rv32i_zbkx`, `rv64i_zbkx`                       |
| Zkn                | 加密NIST算法扩展                         | ✅        | ✅ (RV64 Only)  | `rv64i_zkn`                                      |
| Zknd               | NIST AES解密扩展                         | ✅        | ✅              | `rv32i_zknd`, `rv64i_zknd`                       |
| Zkne               | NIST AES加密扩展                         | ✅        | ✅              | `rv32i_zkne`, `rv64i_zkne`                       |
| Zknh               | NIST SHA哈希扩展                         | ✅        | ✅              | `rv32i_zknh`, `rv64i_zknh`                       |
| Zks                | 加密ShangMi算法扩展                      | ✅        | ✅              | `rv32i_zks`, `rv64i_zks`                         |
| Zcb                | 压缩基本扩展 (位操作相关)                | ✅        | ✅              | `rv32ic_zcb_zbb_m`, `rv64ic_zcb_zbb_zba_m`       |
| Zcd                | 压缩双精度浮点扩展                       | ✅        | ✅              | `rv32ifd_zcd`, `rv64ifd_zcd`                     |
| Zcf                | 压缩单精度浮点扩展                       | ✅        | ✅ (RV32 Only)  | `rv32if_zcf`                                     |
| Zcmp               | 压缩指针操作扩展                         | ✅        | ✅              | `rv32ic_zcmp`, `rv64ic_zcmp`                     |
| Zcmop              | 压缩条件移动/原子操作扩展                | ✅        | ✅              | `rv32ic_zcmop_zacas`, `rv64ic_zcmop`             |
| Zfbfmin            | 标量BF16转换扩展                         | ✅        | ✅              | `rv32if_zfbfmin`, `rv64if_zfbfmin`               |
| Zicbom             | 缓存块管理扩展                           | ✅        | ✅              | `rv32i_zicbom`, `rv64i_zicbom`                   |
| Zicboz             | 缓存块清零扩展                           | ✅        | ✅              | `rv32i_zicboz`, `rv64i_zicboz`                   |
| Zicfilp            | 控制流完整性扩展                         | ✅        | ✅              | `rv32i_zicfilp`, `rv64i_zicfilp`                 |
| Zicfiss            | 影子栈扩展                               | ✅        | ✅              | `rv32i_zicfiss`, `rv64i_zicfiss`                 |
| Zicond             | 条件操作扩展                             | ✅        | ✅              | `rv32i_zicond`, `rv64i_zicond`                   |
| Zilsd              | 负载存储成对扩展                         | ✅        | ⚠️              | 工具链尚不支持 `zilsd`                           |
| Zimop              | 可能操作扩展                             | ✅        | ✅              | `rv32i_zimop`, `rv64i_zimop`                     |
| Zaamo              | 原子内存操作扩展                         | ✅        | ✅              | `rv32ia_zaamo`, `rv64ia_zaamo`                   |
| Zabha              | 字节和半字原子操作扩展                   | ✅        | ✅              | `rv32ia_zabha_zacas`, `rv64ia_zabha_zacas`       |
| Zacas              | 比较交换原子操作扩展                     | ✅        | ✅              | `rv32ia_zacas`, `rv64ia_zacas`                   |
| Zalasr             | 加载保留/存储条件扩展                    | ✅        | ⚠️              | 工具链尚不支持 `zalasr`                          |
| Zalrsc             | LR/SC原子操作扩展                        | ✅        | ✅              | `rv32ia`, `rv64ia`                               |
| Zawrs              | 等待保留集扩展                           | ✅        | ✅              | `rv32i_zawrs`, `rv64i_zawrs`                     |
| Zvbb               | 向量基本位操作扩展                       | ✅        | ✅              | `rv32iv_zvbb`, `rv64iv_zvbb`                     |
| Zvbc               | 向量进位位操作扩展                       | ✅        | ✅              | `rv32iv_zvbc`, `rv64iv_zvbc`                     |
| Zvfbfmin           | 向量BF16转换扩展                         | ✅        | ✅              | `rv32ifv_zvfbfmin`, `rv64ifv_zvfbfmin`           |
| Zvfbfwma           | 向量BF16乘加扩展                         | ✅        | ✅              | `rv32ifv_zvfbfwma`, `rv64ifv_zvfbfwma`           |
| Zvkg               | 向量GCM/GMAC扩展                         | ✅        | ✅              | `rv32iv_zvkg`, `rv64iv_zvkg`                     |
| Zvkned             | 向量NIST AES扩展                         | ✅        | ✅              | `rv32iv_zvkned`, `rv64iv_zvkned`                 |
| Zvknha             | 向量NIST SHA-2扩展                       | ✅        | ✅              | `rv32iv_zvknha`, `rv64iv_zvknha`                 |
| Zvks               | 向量ShangMi扩展                          | ✅        | ✅              | `rv32iv_zvks`, `rv64iv_zvks`                     |
| **S* 系列扩展**    |                                          |          |                |                                                  |
| Sdext              | 调试扩展                                 | ✅        | ✅              | `rv32i_sdext`, `rv64i_sdext`                     |
| Smdbltrp           | M模式双陷阱扩展                          | ✅        | ✅              | `rv32i_smdbltrp_smctr`, `rv64i_smdbltrp_smctr`   |
| Smrnmi             | M模式可恢复非屏蔽中断扩展                | ✅        | ⚠️              | 工具链尚不支持 `smrnmi`                          |
| Svinval            | 细粒度地址转换缓存无效化扩展             | ✅        | ✅              | `rv32i_svinval`, `rv64i_svinval`                 |

*注意: 上述列表可能并非详尽无遗，完整的支持细节和指令列表请参阅 [`assets/riscv_detailed_extension_report.md`](assets/riscv_detailed_extension_report.md) 或生成的代码。部分非常见或特定领域的扩展可能未在此处一一列出。*

### 架构支持

-   **RV32**: 完整支持 32 位 RISC-V 架构的指令。
-   **RV64**: 完整支持 64 位 RISC-V 架构的指令。
-   **共享指令**: 许多指令在 RV32 和 RV64 之间是共享的，本库通过 `merged_instructions` 模块提供了统一处理方式。
-   **特定指令**: 针对 RV32 或 RV64 特有的指令（如 `ADDW` 仅用于 RV64），本库也进行了区分和支持。

## 🚀 快速开始

### 安装

将以下内容添加到您的 `Cargo.toml` 中：

```toml
[dependencies]
riscv-instruction = { git = "https://github.com/canxin121/riscv-instruction-crates" }
```

### 基础使用

本库提供两种主要的指令访问方式：`merged_instructions` 和 `separated_instructions`。

**1. 使用 `merged_instructions` (合并指令视图):**

此模块将跨 RV32/RV64 共享的指令聚合，并为特定于架构的指令提供单独的枚举。

```rust
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
```

**2. 使用 `separated_instructions` (分离指令视图):**

此模块为 RV32 和 RV64 提供各自独立的完整指令集枚举。

```rust
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
```

### 随机指令生成

**1. 使用 `merged_instructions` 进行随机生成:**

```rust
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
```

**2. 使用 `separated_instructions` 进行随机生成:**

```rust
// filepath: riscv-instruction/examples/random_separated_example.rs
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
```

## 📁 项目结构

本项目采用 Cargo 工作空间结构，包含以下主要子包：

```
riscv-instruction-crates/
├── riscv-instruction/          # 主库，提供用户接口，导出生成的指令枚举
├── riscv-instruction-types/    # 基础类型定义（寄存器、立即数、约束等）
├── riscv-instruction-macros/   # 过程宏（如 DeriveInstructionDisplay, DeriveRandom, DeriveValidatedValue）和代码生成逻辑
├── riscv-instruction-parser/   # 从 YAML 解析指令定义并进行修复和转换的工具
└── assets/
    ├── riscv-unified-db/       # Git submodule: RISC-V 官方指令定义 YAML 文件
    └── riscv_instructions.json # 从 YAML 解析并转换后供宏使用的 JSON 指令定义文件
    └── riscv_detailed_extension_report.md # 自动生成的指令集情况报告
```

### 📦 子包说明

#### `riscv-instruction`
-   用户直接交互的主库。
-   通过 `merged_instructions` 和 `separated_instructions` 两个模块导出所有自动生成的 RISC-V 指令枚举和相关类型。
-   包含汇编器兼容性测试。

#### `riscv-instruction-types`
-   定义了所有基础数据类型，例如：
    -   各种寄存器类型 (`IntegerRegister`, `FloatingPointRegister`, `VectorRegister` 等)。
    -   参数化的立即数类型 (`Immediate<N>`, `SignedImmediate<N>`, `UImmediate<N>`)。
    -   用于强类型检查的约束类型和特质 (`ValidatedValue`, `Random` 等)。
    -   CSR 地址、舍入模式、Fence 模式等特殊类型。

#### `riscv-instruction-macros`
-   实现了核心的过程宏：
    -   `generate_merged_riscv_instructions!`: 从 [`assets/riscv_instructions.json`](assets/riscv_instructions.json) 生成合并视图的指令枚举。
    -   `generate_separated_riscv_instructions!`: 从 [`assets/riscv_instructions.json`](assets/riscv_instructions.json) 生成分离视图的指令枚举。
    -   `DeriveInstructionDisplay`: 为指令枚举自动实现 `std::fmt::Display` 以输出汇编代码。
    -   `DeriveRandom`: 为指令枚举和操作数类型自动实现随机生成逻辑。
    -   `DeriveValidatedValue`: 为操作数新类型自动实现值验证和约束逻辑。
-   包含从解析后的 `Instruction` 结构列表生成 Rust 代码的逻辑。

#### `riscv-instruction-parser`
-   负责解析 `riscv-unified-db` 中的 YAML 指令定义文件。
-   对解析的原始指令数据进行必要的修正和规范化（例如，统一操作数名称，处理汇编语法变体）。
-   将处理后的指令数据序列化为 [`assets/riscv_instructions.json`](assets/riscv_instructions.json) 文件，供宏使用。
-   生成详细的指令集支持报告 [`assets/riscv_detailed_extension_report.md`](assets/riscv_detailed_extension_report.md)。


## 🛠️ 更新生成的资源文件

本项目的核心指令定义 ([`assets/riscv_instructions.json`](assets/riscv_instructions.json)) 和详细的扩展支持报告 ([`assets/riscv_detailed_extension_report.md`](assets/riscv_detailed_extension_report.md)) 是通过 `riscv-instruction-parser` 子包中的工具自动生成的。如果您需要基于最新的 `riscv-unified-db`（RISC-V 官方指令定义 YAML 文件）或对解析/修复逻辑进行了修改，可以按以下步骤重新生成这些文件。

### 更新步骤

1.  **确保 `riscv-unified-db` 是最新的**:
    `riscv-unified-db` 是作为 Git submodule 集成在 `assets/` 目录下的。在生成文件之前，请确保它是最新的。
    ```bash
    # 在项目根目录下
    git submodule update --init --recursive
    git submodule foreach git pull origin main
    ```

2.  **运行解析和生成脚本**:
    `riscv-instruction-parser` 包内包含一个可执行目标，用于执行解析、修复、序列化为 JSON 以及生成 Markdown 报告的完整流程。
    ```bash
    # 在项目根目录下运行
    cargo run --package riscv-instruction-parser
    ```
    执行此命令后，[`assets/riscv_instructions.json`](assets/riscv_instructions.json) 和 [`assets/riscv_detailed_extension_report.md`](assets/riscv_detailed_extension_report.md) 将会被更新。


## 🧪 测试

### 测试要求

为了运行完整的汇编器兼容性测试，您需要在您的系统上安装 RISC-V GNU 工具链，特别是 `riscv64-unknown-elf-as` 汇编器。

```bash
# Ubuntu/Debian 示例
sudo apt-get install gcc-riscv64-unknown-elf

# 其他系统或从源码安装，请参考官方文档：
# https://github.com/riscv-collab/riscv-gnu-toolchain
```

### 汇编器兼容性测试

本库包含一个全面的自动化测试套件，用于验证生成的指令汇编输出与 GNU RISC-V 汇编器的兼容性。

```bash
# 在 riscv-instruction 子包目录下运行测试
cd riscv-instruction
cargo test --release -- --show-output
```

测试流程大致如下：
1.  对于每个支持的指令集扩展（及其 RV32/RV64 变体）：
    a.  随机生成大量（例如 10,000 条）该扩展的指令。
    b.  将这些指令输出为汇编代码，并创建一个完整的汇编文件 (`.S`)。
    c.  使用 `riscv64-unknown-elf-as` 并配合适当的 `-march` 参数尝试汇编该文件。
    d.  验证汇编过程是否成功，无错误输出。
2.  如果任何测试用例失败，会生成详细的错误日志和对应的汇编文件，存放在 `riscv-instruction/error_logs` 目录中，便于分析。

目前，绝大多数已实现的指令扩展都已通过此兼容性测试。少数几个扩展（如 `Zalasr`, `Zilsd`, `Smrnmi`）由于当前版本的 `riscv64-unknown-elf-as` 工具链尚不支持或支持不完善，暂时无法进行汇编测试，但这部分指令的类型定义和随机生成逻辑依然在库中提供。



## 📄 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。详见：

-   [MIT License](LICENSE-MIT)
-   [Apache License 2.0](LICENSE-APACHE)