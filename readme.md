# RISC-V 指令库 (RISC-V Instruction Crates)

一个强类型、类型安全的 RISC-V 指令汇编库，用于生成和操作 RISC-V 指令。该库提供了完整的类型系统来表示 RISC-V 指令集，包括标准指令和压缩指令（RVC），支持 RV32 和 RV64 架构。

## 📋 目录

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [RISC-V 指令库 (RISC-V Instruction Crates)](#risc-v-指令库-risc-v-instruction-crates)
  - [📋 目录](#-目录)
  - [✨ 功能特性](#-功能特性)
    - [🔒 类型安全](#-类型安全)
    - [🎯 完整的指令集支持](#-完整的指令集支持)
    - [🛡️ 参数验证](#️-参数验证)
    - [🎲 随机生成](#-随机生成)
    - [📝 汇编输出](#-汇编输出)
  - [🚀 快速开始](#-快速开始)
    - [安装](#安装)
    - [基础使用](#基础使用)
    - [随机指令生成](#随机指令生成)
  - [📁 项目结构](#-项目结构)
    - [📦 子包说明](#-子包说明)
      - [`riscv-instruction`](#riscv-instruction)
      - [`riscv-instruction-types`](#riscv-instruction-types)
      - [`riscv-instruction-macros`](#riscv-instruction-macros)
  - [🎯 支持的指令集](#-支持的指令集)
    - [标准指令集](#标准指令集)
    - [压缩指令集](#压缩指令集)
    - [架构支持](#架构支持)
  - [🧪 测试](#-测试)
    - [测试要求](#测试要求)
    - [汇编器兼容性测试](#汇编器兼容性测试)
  - [🔧 宏扩展](#-宏扩展)
  - [📄 许可证](#-许可证)

<!-- /code_chunk_output -->



## ✨ 功能特性

### 🔒 类型安全
- **强类型检查**: 所有寄存器和立即数都有严格的类型验证

### 🎯 完整的指令集支持
- **标准指令集**: I、M、A、F、D、Q、Zifencei、Zicsr 扩展
- **压缩指令集**: RVC 扩展
- **多架构**: 支持 RV32 和 RV64
- **自动生成**: 从 JSON 配置自动生成指令定义

### 🛡️ 参数验证
- **寄存器验证**: 自动验证寄存器编号范围
- **立即数验证**: 检查立即数位长度和取值范围
- **约束检查**: 操作数非零、倍数、禁用值等约束

### 🎲 随机生成
- **测试支持**: 内置随机指令生成功能
- **约束感知**: 随机生成遵循所有类型约束
- **可重现**: 支持种子控制的随机生成

### 📝 汇编输出
- **可读格式**: 自动生成标准 RISC-V 汇编语法
- **格式化**: 支持自定义汇编格式
- **兼容性**: 与标准汇编器兼容

## 🚀 快速开始

### 安装

将以下内容添加到您的 `Cargo.toml`:

```toml
[dependencies]
riscv-instruction = { git = "https://github.com/canxin121/riscv-instruction-crates" }
```

### 基础使用

```rust
use riscv_instruction::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建寄存器
    let rd = IntegerRegister::new(1)?;  // x1
    let rs1 = IntegerRegister::new(2)?; // x2
    let rs2 = IntegerRegister::new(3)?; // x3
    
    // 创建立即数
    let imm = Immediate::<12>::new(100)?;
    
    // 创建指令
    let add_inst = StandardSharedInstruction::I(StandardISharedInstructions::ADD { 
        rd, rs1, rs2 
    });
    
    let addi_inst = StandardSharedInstruction::I(StandardISharedInstructions::ADDI { 
        rd, rs1, imm 
    });
    
    // 输出汇编
    println!("{}", add_inst);   // add x1, x2, x3
    println!("{}", addi_inst);  // addi x1, x2, 100
    
    Ok(())
}
```

### 随机指令生成

```rust
use riscv_instruction::*;

fn main() {
    let mut rng = rand::rng();
    
    // 生成随机标准指令
    let random_inst = StandardSharedInstruction::random_with_rng(&mut rng);
    println!("随机指令: {}", random_inst);
    
    // 生成随机压缩指令
    let random_compressed = RVCSharedInstruction::random_with_rng(&mut rng);
    println!("随机压缩指令: {}", random_compressed);
}
```

## 📁 项目结构

本项目采用 Cargo 工作空间结构，包含三个子包：

```
riscv-instruction-crates/
├── riscv-instruction/          # 主库，包含生成的指令定义
├── riscv-instruction-types/    # 基础类型定义（寄存器、立即数等）
├── riscv-instruction-macros/   # 过程宏和代码生成器
└── assets/
    └── riscv_instructions.json # 指令定义配置文件
```

### 📦 子包说明

#### `riscv-instruction`
- 主要的用户接口库
- 包含自动生成的所有 RISC-V 指令定义
- 提供完整的指令枚举和类型

#### `riscv-instruction-types`
- 基础类型定义
- 寄存器类型（整数寄存器、浮点寄存器等）
- 立即数类型（有符号、无符号、约束类型等）
- 特殊类型（CSR 地址、舍入模式等）

#### `riscv-instruction-macros`
- 过程宏实现
- 代码生成器
- 自动从 JSON 配置生成指令定义

## 🎯 支持的指令集

### 标准指令集

| 扩展 | 描述 | 支持状态 |
|------|------|----------|
| I | 基础整数指令集 | ✅ |
| M | 乘法和除法扩展 | ✅ |
| A | 原子操作扩展 | ✅ |
| F | 单精度浮点扩展 | ✅ |
| D | 双精度浮点扩展 | ✅ |
| Q | 四精度浮点扩展 | ✅ |
| Zifencei | 指令围栏扩展 | ✅ |
| Zicsr | CSR 操作扩展 | ✅ |

### 压缩指令集

| 扩展 | 描述 | 支持状态 |
|------|------|----------|
| C | 压缩指令扩展 | ✅ |

### 架构支持

- **RV32**: 32位 RISC-V 架构
- **RV64**: 64位 RISC-V 架构
- **共享指令**: 在两种架构间共享的指令
- **特定指令**: 特定于某一架构的指令

## 🧪 测试

### 测试要求

运行完整测试需要安装 RISC-V GNU 工具链：

```bash
# Ubuntu/Debian
sudo apt-get install gcc-riscv64-unknown-elf

# 或从官方下载
# https://github.com/riscv-collab/riscv-gnu-toolchain
```

### 汇编器兼容性测试

本库包含自动化测试，验证生成的指令与 GNU RISC-V 汇编器的兼容性：

```bash
# 运行汇编器兼容性测试（需要安装 riscv64-unknown-elf-as）
cargo test
```

测试过程：
1. 随机生成 10000 个指令
2. 创建汇编文件
3. 使用 `riscv64-unknown-elf-as` 汇编
4. 验证汇编成功
5. 重复 100 次以确保稳定性


## 🔧 宏扩展

```rust
// Recursive expansion of generate_riscv_instructions! macro
// ==========================================================

use riscv_instruction_macros::{DeriveInstructionDisplay, DeriveRandom, DeriveValidatedValue};
pub use riscv_instruction_types::*;
use std::fmt::{self, Display};
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "31",
    name = "IntegerRegisterExcept0",
    display = "x{}",
    forbidden = "0"
)]
pub struct IntegerRegisterExcept0(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "31",
    name = "IntegerRegisterExcept2",
    display = "x{}",
    forbidden = "2"
)]
pub struct IntegerRegisterExcept2(u8);

#[doc = "RVC shared instructions for I extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum RVCISharedInstructions {
    #[asm("c.xor {rs1p}, {rs2p}")]
    C_XOR {
        rs1p:CompressedIntegerRegister,rs2p:CompressedIntegerRegister
    }, #[asm("c.jr {rs1}")]
    C_JR {
        rs1:IntegerRegisterExcept0
    }, #[asm("c.ebreak ")]
    C_EBREAK, #[asm("c.jalr {rs1}")]
    C_JALR {
        rs1:IntegerRegisterExcept0
    }, #[asm("c.add {rs1}, {rs2}")]
    C_ADD {
        rs1:IntegerRegister,rs2:IntegerRegisterExcept0
    }, #[asm("c.and {rs1p}, {rs2p}")]
    C_AND {
        rs2p:CompressedIntegerRegister,rs1p:CompressedIntegerRegister
    }, #[asm("c.bnez {rs1p}, {imm}")]
    C_BNEZ {
        imm:Immediate<8> ,rs1p:CompressedIntegerRegister
    }, #[asm("c.lwsp {rd}, {uimm}(sp)")]
    C_LWSP {
        rd:IntegerRegisterExcept0,uimm:MultipleOfUImmediate<6,4>
    }, #[asm("c.srli64 {rs1p}")]
    C_SRLI64 {
        rs1p:CompressedIntegerRegister
    }, #[asm("c.addi16sp sp, {nzimm}")]
    C_ADDI16SP {
        nzimm:MultipleOfNZImmediate<6,16>
    }, #[asm("c.addi4spn {rdp}, sp, {nzuimm}")]
    C_ADDI4SPN {
        nzuimm:MultipleOfNZUImmediate<8,4> ,rdp:CompressedIntegerRegister
    }, #[asm("c.srai64 {rs1p}")]
    C_SRAI64 {
        rs1p:CompressedIntegerRegister
    }, #[asm("c.nop {nzimm}")]
    C_NOP {
        nzimm:NZImmediate<6>
    }, #[asm("c.sub {rs1p}, {rs2p}")]
    C_SUB {
        rs1p:CompressedIntegerRegister,rs2p:CompressedIntegerRegister
    }, #[asm("c.andi {rs1p}, {imm}")]
    C_ANDI {
        imm:Immediate<6> ,rs1p:CompressedIntegerRegister
    }, #[asm("c.beqz {rs1p}, {imm}")]
    C_BEQZ {
        imm:Immediate<8> ,rs1p:CompressedIntegerRegister
    }, #[asm("c.slli64 {rs1}")]
    C_SLLI64 {
        rs1:IntegerRegister
    }, #[asm("c.srai {rs1p}, {nzuimm}")]
    C_SRAI {
        nzuimm:NZUImmediate<5> ,rs1p:CompressedIntegerRegister
    }, #[asm("c.j {imm}")]
    C_J {
        imm:Immediate<11>
    }, #[asm("c.mv {rd}, {rs2}")]
    C_MV {
        rd:IntegerRegister,rs2:IntegerRegisterExcept0
    }, #[asm("c.addi {rs1}, {nzimm}")]
    C_ADDI {
        rs1:IntegerRegister,nzimm:NZImmediate<6>
    }, #[asm("c.li {rd}, {imm}")]
    C_LI {
        rd:IntegerRegister,imm:Immediate<6>
    }, #[asm_code("format!(\"c.lui {}, 0x{:x}\", rd, rimm.get() as u32 & 0xfffff)")]
    C_LUI {
        rd:IntegerRegisterExcept2,rimm:NonZeroRangeImmediate<-32,31>
    }, #[asm("c.sw {rs2p}, {uimm}({rs1p})")]
    C_SW {
        uimm:MultipleOfUImmediate<5,4> ,rs2p:CompressedIntegerRegister,rs1p:CompressedIntegerRegister
    }, #[asm("c.srli {rs1p}, {nzuimm}")]
    C_SRLI {
        nzuimm:NZUImmediate<5> ,rs1p:CompressedIntegerRegister
    }, #[asm("c.swsp {rs2}, {uimm}(sp)")]
    C_SWSP {
        rs2:IntegerRegister,uimm:MultipleOfUImmediate<6,4>
    }, #[asm("c.slli {rs1}, {nzuimm}")]
    C_SLLI {
        rs1:IntegerRegister,nzuimm:NZUImmediate<5>
    }, #[asm("c.lw {rdp}, {uimm}({rs1p})")]
    C_LW {
        uimm:MultipleOfUImmediate<5,4> ,rs1p:CompressedIntegerRegister,rdp:CompressedIntegerRegister
    }, #[asm("c.or {rs1p}, {rs2p}")]
    C_OR {
        rs2p:CompressedIntegerRegister,rs1p:CompressedIntegerRegister
    }
}
#[doc = "RVC shared instructions for D extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum RVCDSharedInstructions {
    #[asm("c.fsd {fs2p}, {uimm}({rs1p})")]
    C_FSD {
        uimm:MultipleOfUImmediate<5,8> ,rs1p:CompressedIntegerRegister,fs2p:CompressedFloatingPointRegister
    }, #[asm("c.fsdsp {fs2}, {uimm}(sp)")]
    C_FSDSP {
        fs2:FloatingPointRegister,uimm:MultipleOfUImmediate<6,8>
    }, #[asm("c.fldsp {fd}, {uimm}(sp)")]
    C_FLDSP {
        fd:FloatingPointRegister,uimm:MultipleOfUImmediate<6,8>
    }, #[asm("c.fld {fdp}, {uimm}({rs1p})")]
    C_FLD {
        uimm:MultipleOfUImmediate<5,8> ,fdp:CompressedFloatingPointRegister,rs1p:CompressedIntegerRegister
    }
}
#[doc = "RVC RV32 specific instructions for F extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum RVCRV32FSpecificInstructions {
    #[asm("c.fswsp {fs2}, {uimm}(sp)")]
    C_FSWSP {
        fs2:FloatingPointRegister,uimm:MultipleOfUImmediate<6,4>
    }, #[asm("c.flw {fdp}, {uimm}({rs1p})")]
    C_FLW {
        fdp:CompressedFloatingPointRegister,uimm:MultipleOfUImmediate<5,4> ,rs1p:CompressedIntegerRegister
    }, #[asm("c.flwsp {fd}, {uimm}(sp)")]
    C_FLWSP {
        fd:FloatingPointRegister,uimm:MultipleOfUImmediate<6,4>
    }, #[asm("c.fsw {fs2p}, {uimm}({rs1p})")]
    C_FSW {
        fs2p:CompressedFloatingPointRegister,uimm:MultipleOfUImmediate<5,4> ,rs1p:CompressedIntegerRegister
    }
}
#[doc = "RVC RV32 specific instructions for I extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum RVCRV32ISpecificInstructions {
    #[asm("c.jal {imm}")]
    C_JAL {
        imm:Immediate<11>
    }
}
#[doc = "RVC RV64 specific instructions for I extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum RVCRV64ISpecificInstructions {
    #[asm("c.sdsp {rs2}, {uimm}(sp)")]
    C_SDSP {
        rs2:IntegerRegister,uimm:MultipleOfUImmediate<6,8>
    }, #[asm("c.ld {rdp}, {uimm}({rs1p})")]
    C_LD {
        uimm:MultipleOfUImmediate<5,8> ,rdp:CompressedIntegerRegister,rs1p:CompressedIntegerRegister
    }, #[asm("c.addiw {rs1}, {imm}")]
    C_ADDIW {
        rs1:IntegerRegisterExcept0,imm:Immediate<6>
    }, #[asm("c.ldsp {rd}, {uimm}(sp)")]
    C_LDSP {
        rd:IntegerRegisterExcept0,uimm:MultipleOfUImmediate<6,8>
    }, #[asm("c.addw {rs1p}, {rs2p}")]
    C_ADDW {
        rs1p:CompressedIntegerRegister,rs2p:CompressedIntegerRegister
    }, #[asm("c.sd {rs2p}, {uimm}({rs1p})")]
    C_SD {
        uimm:MultipleOfUImmediate<5,8> ,rs2p:CompressedIntegerRegister,rs1p:CompressedIntegerRegister
    }, #[asm("c.subw {rs1p}, {rs2p}")]
    C_SUBW {
        rs2p:CompressedIntegerRegister,rs1p:CompressedIntegerRegister
    }
}
#[doc = "RVC instructions shared across all ISA bases, grouped by extension."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum RVCSharedInstruction {
    I(RVCISharedInstructions),
    D(RVCDSharedInstructions),
}
#[doc = "RVC RV32 specific instructions, grouped by extension."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum RVCRV32SpecificInstruction {
    F(RVCRV32FSpecificInstructions),
    I(RVCRV32ISpecificInstructions),
}
#[doc = "RVC RV64 specific instructions, grouped by extension."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum RVCRV64SpecificInstruction {
    I(RVCRV64ISpecificInstructions),
}
#[doc = "RVC ISA base specific instructions."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum RVCSpecificInstruction {
    RV32(RVCRV32SpecificInstruction),
    RV64(RVCRV64SpecificInstruction),
}
#[doc = "RVC RISC-V instructions, dispatching to shared or specific instructions."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum RVCInstruction {
    #[doc = "Instructions shared across ISA bases"]
    Shared(RVCSharedInstruction),
    #[doc = "ISA base specific instructions"]
    Specific(RVCSpecificInstruction),
}
#[doc = "Standard shared instructions for F extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardFSharedInstructions {
    #[asm("fsgnj.s {fd}, {fs1}, {fs2}")]
    FSGNJ_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fmax.s {fd}, {fs1}, {fs2}")]
    FMAX_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsw {fs2}, {imm}({rs1})")]
    FSW {
        rs1:IntegerRegister,fs2:FloatingPointRegister,imm:Immediate<12>
    }, #[asm("flt.s {rd}, {fs1}, {fs2}")]
    FLT_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fmv.x.w {rd}, {fs1}")]
    FMV_X_W {
        rd:IntegerRegister,fs1:FloatingPointRegister
    }, #[asm("fcvt.s.w {fd}, {rs1}, {rm}")]
    FCVT_S_W {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }, #[asm("fmin.s {fd}, {fs1}, {fs2}")]
    FMIN_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsgnjn.s {fd}, {fs1}, {fs2}")]
    FSGNJN_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fnmsub.s {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FNMSUB_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fadd.s {fd}, {fs1}, {fs2}, {rm}")]
    FADD_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fnmadd.s {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FNMADD_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsgnjx.s {fd}, {fs1}, {fs2}")]
    FSGNJX_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsqrt.s {fd}, {fs1}, {rm}")]
    FSQRT_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.w.s {rd}, {fs1}, {rm}")]
    FCVT_W_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmadd.s {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FMADD_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmsub.s {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FMSUB_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("feq.s {rd}, {fs1}, {fs2}")]
    FEQ_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("flw {fd}, {imm}({rs1})")]
    FLW {
        fd:FloatingPointRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("fmul.s {fd}, {fs1}, {fs2}, {rm}")]
    FMUL_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmv.w.x {fd}, {rs1}")]
    FMV_W_X {
        fd:FloatingPointRegister,rs1:IntegerRegister
    }, #[asm("fclass.s {rd}, {fs1}")]
    FCLASS_S {
        rd:IntegerRegister,fs1:FloatingPointRegister
    }, #[asm("fcvt.wu.s {rd}, {fs1}, {rm}")]
    FCVT_WU_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsub.s {fd}, {fs1}, {fs2}, {rm}")]
    FSUB_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.s.wu {fd}, {rs1}, {rm}")]
    FCVT_S_WU {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }, #[asm("fle.s {rd}, {fs1}, {fs2}")]
    FLE_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fdiv.s {fd}, {fs1}, {fs2}, {rm}")]
    FDIV_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }
}
#[doc = "Standard shared instructions for D extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardDSharedInstructions {
    #[asm("fmax.d {fd}, {fs1}, {fs2}")]
    FMAX_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fcvt.d.w {fd}, {rs1}")]
    FCVT_D_W {
        fd:FloatingPointRegister,rs1:IntegerRegister
    }, #[asm("fld {fd}, {imm}({rs1})")]
    FLD {
        fd:FloatingPointRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("fmin.d {fd}, {fs1}, {fs2}")]
    FMIN_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fclass.d {rd}, {fs1}")]
    FCLASS_D {
        rd:IntegerRegister,fs1:FloatingPointRegister
    }, #[asm("fmul.d {fd}, {fs1}, {fs2}, {rm}")]
    FMUL_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsgnjx.d {fd}, {fs1}, {fs2}")]
    FSGNJX_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fdiv.d {fd}, {fs1}, {fs2}, {rm}")]
    FDIV_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.d.s {fd}, {fs1}")]
    FCVT_D_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister
    }, #[asm("fsqrt.d {fd}, {fs1}, {rm}")]
    FSQRT_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmadd.d {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FMADD_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.wu.d {rd}, {fs1}, {rm}")]
    FCVT_WU_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fle.d {rd}, {fs1}, {fs2}")]
    FLE_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fcvt.w.d {rd}, {fs1}, {rm}")]
    FCVT_W_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fnmsub.d {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FNMSUB_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.d.wu {fd}, {rs1}")]
    FCVT_D_WU {
        fd:FloatingPointRegister,rs1:IntegerRegister
    }, #[asm("fnmadd.d {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FNMADD_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmsub.d {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FMSUB_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsub.d {fd}, {fs1}, {fs2}, {rm}")]
    FSUB_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fadd.d {fd}, {fs1}, {fs2}, {rm}")]
    FADD_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.s.d {fd}, {fs1}, {rm}")]
    FCVT_S_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("feq.d {rd}, {fs1}, {fs2}")]
    FEQ_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsd {fs2}, {imm}({rs1})")]
    FSD {
        rs1:IntegerRegister,fs2:FloatingPointRegister,imm:Immediate<12>
    }, #[asm("flt.d {rd}, {fs1}, {fs2}")]
    FLT_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsgnjn.d {fd}, {fs1}, {fs2}")]
    FSGNJN_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsgnj.d {fd}, {fs1}, {fs2}")]
    FSGNJ_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }
}
#[doc = "Standard shared instructions for A extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardASharedInstructions {
    #[asm_code("if *aq && *rl {\n    format!(\"amoswap.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoswap.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoswap.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoswap.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOSWAP_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoand.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoand.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoand.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoand.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOAND_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amomin.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amomin.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amomin.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amomin.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMIN_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoor.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoor.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoor.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoor.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOOR_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"lr.w.aqrl {}, ({})\", rd, rs1)\n} else if *aq {\n    format!(\"lr.w.aq {}, ({})\", rd, rs1)\n} else if *rl {\n    format!(\"lr.w.rl {}, ({})\", rd, rs1)\n} else {\n    format!(\"lr.w {}, ({})\", rd, rs1)\n}")]
    LR_W {
        rd:IntegerRegister,rs1:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amominu.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amominu.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amominu.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amominu.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMINU_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amomax.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amomax.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amomax.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amomax.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMAX_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoxor.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoxor.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoxor.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoxor.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOXOR_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amomaxu.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amomaxu.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amomaxu.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amomaxu.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMAXU_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoadd.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoadd.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoadd.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoadd.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOADD_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"sc.w.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"sc.w.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"sc.w.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"sc.w {}, {}, ({})\", rd, rs2, rs1)\n}")]
    SC_W {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }
}
#[doc = "Standard shared instructions for I extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardISharedInstructions {
    #[asm("sltu {rd}, {rs1}, {rs2}")]
    SLTU {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("jal {rd}, {imm}")]
    JAL {
        rd:IntegerRegister,imm:Immediate<20>
    }, #[asm("and {rd}, {rs1}, {rs2}")]
    AND {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("bgeu {rs1}, {rs2}, {imm}")]
    BGEU {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("sb {rs2}, {imm}({rs1})")]
    SB {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("jalr {rd}, {rs1}, {imm}")]
    JALR {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("ecall ")]
    ECALL, #[asm("lhu {rd}, {imm}({rs1})")]
    LHU {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("sub {rd}, {rs1}, {rs2}")]
    SUB {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("lw {rd}, {imm}({rs1})")]
    LW {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("lui {rd}, {uimm}")]
    LUI {
        rd:IntegerRegister,uimm:UImmediate<20>
    }, #[asm("auipc {rd}, {uimm}")]
    AUIPC {
        rd:IntegerRegister,uimm:UImmediate<20>
    }, #[asm("bge {rs1}, {rs2}, {imm}")]
    BGE {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("add {rd}, {rs1}, {rs2}")]
    ADD {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("xor {rd}, {rs1}, {rs2}")]
    XOR {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("srli {rd}, {rs1}, {shamt}")]
    SRLI {
        rd:IntegerRegister,rs1:IntegerRegister,shamt:ShiftAmount<5>
    }, #[asm("slt {rd}, {rs1}, {rs2}")]
    SLT {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("srai {rd}, {rs1}, {shamt}")]
    SRAI {
        rd:IntegerRegister,rs1:IntegerRegister,shamt:ShiftAmount<5>
    }, #[asm("bltu {rs1}, {rs2}, {imm}")]
    BLTU {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("addi {rd}, {rs1}, {imm}")]
    ADDI {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("sltiu {rd}, {rs1}, {imm}")]
    SLTIU {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("blt {rs1}, {rs2}, {imm}")]
    BLT {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("andi {rd}, {rs1}, {imm}")]
    ANDI {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("sll {rd}, {rs1}, {rs2}")]
    SLL {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("slli {rd}, {rs1}, {shamt}")]
    SLLI {
        rd:IntegerRegister,rs1:IntegerRegister,shamt:ShiftAmount<5>
    }, #[asm("bne {rs1}, {rs2}, {imm}")]
    BNE {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("lb {rd}, {imm}({rs1})")]
    LB {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("beq {rs1}, {rs2}, {imm}")]
    BEQ {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("sw {rs2}, {imm}({rs1})")]
    SW {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("lbu {rd}, {imm}({rs1})")]
    LBU {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("or {rd}, {rs1}, {rs2}")]
    OR {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("xori {rd}, {rs1}, {imm}")]
    XORI {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("lh {rd}, {imm}({rs1})")]
    LH {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("ebreak ")]
    EBREAK, #[asm("fence {pred}, {succ}")]
    FENCE {
        pred:FenceMode,succ:FenceMode
    }, #[asm("sh {rs2}, {imm}({rs1})")]
    SH {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("sra {rd}, {rs1}, {rs2}")]
    SRA {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("srl {rd}, {rs1}, {rs2}")]
    SRL {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("slti {rd}, {rs1}, {imm}")]
    SLTI {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("ori {rd}, {rs1}, {imm}")]
    ORI {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }
}
#[doc = "Standard shared instructions for M extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardMSharedInstructions {
    #[asm("remu {rd}, {rs1}, {rs2}")]
    REMU {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("mulhsu {rd}, {rs1}, {rs2}")]
    MULHSU {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("divu {rd}, {rs1}, {rs2}")]
    DIVU {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("rem {rd}, {rs1}, {rs2}")]
    REM {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("mulhu {rd}, {rs1}, {rs2}")]
    MULHU {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("mulh {rd}, {rs1}, {rs2}")]
    MULH {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("div {rd}, {rs1}, {rs2}")]
    DIV {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("mul {rd}, {rs1}, {rs2}")]
    MUL {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }
}
#[doc = "Standard shared instructions for Zifencei extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardZifenceiSharedInstructions {
    #[asm("fence.i")]
    FENCE_I
}
#[doc = "Standard shared instructions for Zicsr extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardZicsrSharedInstructions {
    #[asm("csrrc {rd}, {csr}, {rs1}")]
    CSRRC {
        rd:IntegerRegister,rs1:IntegerRegister,csr:CSRAddress
    }, #[asm("csrrw {rd}, {csr}, {rs1}")]
    CSRRW {
        rd:IntegerRegister,rs1:IntegerRegister,csr:CSRAddress
    }, #[asm("csrrsi {rd}, {csr}, {uimm}")]
    CSRRSI {
        rd:IntegerRegister,uimm:UImmediate<5> ,csr:CSRAddress
    }, #[asm("csrrs {rd}, {csr}, {rs1}")]
    CSRRS {
        rd:IntegerRegister,rs1:IntegerRegister,csr:CSRAddress
    }, #[asm("csrrwi {rd}, {csr}, {uimm}")]
    CSRRWI {
        rd:IntegerRegister,uimm:UImmediate<5> ,csr:CSRAddress
    }, #[asm("csrrci {rd}, {csr}, {uimm}")]
    CSRRCI {
        rd:IntegerRegister,uimm:UImmediate<5> ,csr:CSRAddress
    }
}
#[doc = "Standard shared instructions for Q extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardQSharedInstructions {
    #[asm("fcvt.q.s {fd}, {fs1}")]
    FCVT_Q_S {
        fd:FloatingPointRegister,fs1:FloatingPointRegister
    }, #[asm("feq.q {rd}, {fs1}, {fs2}")]
    FEQ_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsgnjx.q {fd}, {fs1}, {fs2}")]
    FSGNJX_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("flq {fd}, {imm}({rs1})")]
    FLQ {
        fd:FloatingPointRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("fclass.q {rd}, {fs1}")]
    FCLASS_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister
    }, #[asm("fmadd.q {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FMADD_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsgnjn.q {fd}, {fs1}, {fs2}")]
    FSGNJN_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fcvt.w.q {rd}, {fs1}, {rm}")]
    FCVT_W_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsgnj.q {fd}, {fs1}, {fs2}")]
    FSGNJ_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsqrt.q {fd}, {fs1}, {rm}")]
    FSQRT_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fadd.q {fd}, {fs1}, {fs2}, {rm}")]
    FADD_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.s.q {fd}, {fs1}, {rm}")]
    FCVT_S_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.q.w {fd}, {rs1}")]
    FCVT_Q_W {
        fd:FloatingPointRegister,rs1:IntegerRegister
    }, #[asm("flt.q {rd}, {fs1}, {fs2}")]
    FLT_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fcvt.q.wu {fd}, {rs1}")]
    FCVT_Q_WU {
        fd:FloatingPointRegister,rs1:IntegerRegister
    }, #[asm("fmsub.q {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FMSUB_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.wu.q {rd}, {fs1}, {rm}")]
    FCVT_WU_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fsq {fs2}, {imm}({rs1})")]
    FSQ {
        rs1:IntegerRegister,fs2:FloatingPointRegister,imm:Immediate<12>
    }, #[asm("fmul.q {fd}, {fs1}, {fs2}, {rm}")]
    FMUL_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmin.q {fd}, {fs1}, {fs2}")]
    FMIN_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fsub.q {fd}, {fs1}, {fs2}, {rm}")]
    FSUB_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fnmsub.q {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FNMSUB_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fdiv.q {fd}, {fs1}, {fs2}, {rm}")]
    FDIV_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fle.q {rd}, {fs1}, {fs2}")]
    FLE_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fnmadd.q {fd}, {fs1}, {fs2}, {fs3}, {rm}")]
    FNMADD_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister,fs3:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmax.q {fd}, {fs1}, {fs2}")]
    FMAX_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,fs2:FloatingPointRegister
    }, #[asm("fcvt.d.q {fd}, {fs1}, {rm}")]
    FCVT_D_Q {
        fd:FloatingPointRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.q.d {fd}, {fs1}")]
    FCVT_Q_D {
        fd:FloatingPointRegister,fs1:FloatingPointRegister
    }
}
#[doc = "Standard RV64 specific instructions for F extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardRV64FSpecificInstructions {
    #[asm("fcvt.l.s {rd}, {fs1}, {rm}")]
    FCVT_L_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.lu.s {rd}, {fs1}, {rm}")]
    FCVT_LU_S {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.s.lu {fd}, {rs1}, {rm}")]
    FCVT_S_LU {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }, #[asm("fcvt.s.l {fd}, {rs1}, {rm}")]
    FCVT_S_L {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }
}
#[doc = "Standard RV64 specific instructions for D extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardRV64DSpecificInstructions {
    #[asm("fcvt.lu.d {rd}, {fs1}, {rm}")]
    FCVT_LU_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.l.d {rd}, {fs1}, {rm}")]
    FCVT_L_D {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fmv.d.x {fd}, {rs1}")]
    FMV_D_X {
        fd:FloatingPointRegister,rs1:IntegerRegister
    }, #[asm("fcvt.d.l {fd}, {rs1}, {rm}")]
    FCVT_D_L {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }, #[asm("fcvt.d.lu {fd}, {rs1}, {rm}")]
    FCVT_D_LU {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }, #[asm("fmv.x.d {rd}, {fs1}")]
    FMV_X_D {
        rd:IntegerRegister,fs1:FloatingPointRegister
    }
}
#[doc = "Standard RV64 specific instructions for A extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardRV64ASpecificInstructions {
    #[asm_code("if *aq && *rl {\n    format!(\"amoswap.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoswap.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoswap.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoswap.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOSWAP_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amomin.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amomin.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amomin.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amomin.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMIN_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amomax.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amomax.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amomax.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amomax.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMAX_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoand.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoand.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoand.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoand.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOAND_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amomaxu.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amomaxu.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amomaxu.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amomaxu.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMAXU_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"sc.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"sc.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"sc.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"sc.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    SC_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoxor.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoxor.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoxor.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoxor.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOXOR_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoadd.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoadd.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoadd.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoadd.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOADD_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amominu.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amominu.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amominu.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amominu.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOMINU_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"amoor.d.aqrl {}, {}, ({})\", rd, rs2, rs1)\n} else if *aq {\n    format!(\"amoor.d.aq {}, {}, ({})\", rd, rs2, rs1)\n} else if *rl {\n    format!(\"amoor.d.rl {}, {}, ({})\", rd, rs2, rs1)\n} else {\n    format!(\"amoor.d {}, {}, ({})\", rd, rs2, rs1)\n}")]
    AMOOR_D {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister,aq:bool,rl:bool
    }, #[asm_code("if *aq && *rl {\n    format!(\"lr.d.aqrl {}, ({})\", rd, rs1)\n} else if *aq {\n    format!(\"lr.d.aq {}, ({})\", rd, rs1)\n} else if *rl {\n    format!(\"lr.d.rl {}, ({})\", rd, rs1)\n} else {\n    format!(\"lr.d {}, ({})\", rd, rs1)\n}")]
    LR_D {
        rd:IntegerRegister,rs1:IntegerRegister,aq:bool,rl:bool
    }
}
#[doc = "Standard RV64 specific instructions for I extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardRV64ISpecificInstructions {
    #[asm("addw {rd}, {rs1}, {rs2}")]
    ADDW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("subw {rd}, {rs1}, {rs2}")]
    SUBW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("sllw {rd}, {rs1}, {rs2}")]
    SLLW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("sd {rs2}, {imm}({rs1})")]
    SD {
        rs1:IntegerRegister,rs2:IntegerRegister,imm:Immediate<12>
    }, #[asm("srliw {rd}, {rs1}, {shamt}")]
    SRLIW {
        rd:IntegerRegister,rs1:IntegerRegister,shamt:ShiftAmount<5>
    }, #[asm("lwu {rd}, {imm}({rs1})")]
    LWU {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("sraw {rd}, {rs1}, {rs2}")]
    SRAW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("addiw {rd}, {rs1}, {imm}")]
    ADDIW {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("ld {rd}, {imm}({rs1})")]
    LD {
        rd:IntegerRegister,rs1:IntegerRegister,imm:Immediate<12>
    }, #[asm("sraiw {rd}, {rs1}, {shamt}")]
    SRAIW {
        rd:IntegerRegister,rs1:IntegerRegister,shamt:ShiftAmount<5>
    }, #[asm("srlw {rd}, {rs1}, {rs2}")]
    SRLW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("slliw {rd}, {rs1}, {shamt}")]
    SLLIW {
        rd:IntegerRegister,rs1:IntegerRegister,shamt:ShiftAmount<5>
    }
}
#[doc = "Standard RV64 specific instructions for M extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardRV64MSpecificInstructions {
    #[asm("remuw {rd}, {rs1}, {rs2}")]
    REMUW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("mulw {rd}, {rs1}, {rs2}")]
    MULW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("divw {rd}, {rs1}, {rs2}")]
    DIVW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("divuw {rd}, {rs1}, {rs2}")]
    DIVUW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }, #[asm("remw {rd}, {rs1}, {rs2}")]
    REMW {
        rd:IntegerRegister,rs1:IntegerRegister,rs2:IntegerRegister
    }
}
#[doc = "Standard RV64 specific instructions for Q extension"]
#[derive(Debug,Clone,PartialEq,DeriveInstructionDisplay,DeriveRandom)]
#[rustfmt::skip]
pub enum StandardRV64QSpecificInstructions {
    #[asm("fcvt.lu.q {rd}, {fs1}, {rm}")]
    FCVT_LU_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.l.q {rd}, {fs1}, {rm}")]
    FCVT_L_Q {
        rd:IntegerRegister,fs1:FloatingPointRegister,rm:RoundingMode
    }, #[asm("fcvt.q.l {fd}, {rs1}, {rm}")]
    FCVT_Q_L {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }, #[asm("fcvt.q.lu {fd}, {rs1}, {rm}")]
    FCVT_Q_LU {
        fd:FloatingPointRegister,rs1:IntegerRegister,rm:RoundingMode
    }
}
#[doc = "Standard instructions shared across all ISA bases, grouped by extension."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum StandardSharedInstruction {
    F(StandardFSharedInstructions),
    D(StandardDSharedInstructions),
    A(StandardASharedInstructions),
    I(StandardISharedInstructions),
    M(StandardMSharedInstructions),
    Zifencei(StandardZifenceiSharedInstructions),
    Zicsr(StandardZicsrSharedInstructions),
    Q(StandardQSharedInstructions),
}
#[doc = "Standard RV64 specific instructions, grouped by extension."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum StandardRV64SpecificInstruction {
    F(StandardRV64FSpecificInstructions),
    D(StandardRV64DSpecificInstructions),
    A(StandardRV64ASpecificInstructions),
    I(StandardRV64ISpecificInstructions),
    M(StandardRV64MSpecificInstructions),
    Q(StandardRV64QSpecificInstructions),
}
#[doc = "Standard ISA base specific instructions."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum StandardSpecificInstruction {
    RV64(StandardRV64SpecificInstruction),
}
#[doc = "Standard RISC-V instructions, dispatching to shared or specific instructions."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum StandardInstruction {
    #[doc = "Instructions shared across ISA bases"]
    Shared(StandardSharedInstruction),
    #[doc = "ISA base specific instructions"]
    Specific(StandardSpecificInstruction),
}
#[doc = r" Main RISC-V instruction enum, dispatching to Standard or RVC instructions."]
#[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
pub enum RiscvInstruction {
    #[doc = "Standard RISC-V instructions"]
    Standard(StandardInstruction),
    #[doc = "RISC-V Compressed instructions"]
    RVC(RVCInstruction),
}
```

## 📄 许可证

本项目使用 MIT 或 Apache-2.0 双重许可证。详见：

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)
