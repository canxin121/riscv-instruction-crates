//! RISC-V instruction types and register utilities

use std::fmt::{self, Display};

use rand::seq::IndexedRandom as _;
use riscv_instruction_macros::{DeriveRandom, DeriveValidatedValue};

pub trait Random {
    type Output;
    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output;
}

impl Random for bool {
    type Output = bool;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        rng.random()
    }
}

/// Common trait for validated value types
pub trait ValidatedValue<T: 'static> {
    const MIN: T;
    const MAX: T;
    const MULTIPLE_OF: Option<T> = None;
    const FORBIDDEN: &'static [T] = &[];
    const ODD_ONLY: bool = false;
    type Error;
    fn new(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn get(&self) -> T;
    fn set(&mut self, value: T) -> Result<(), Self::Error>;
}

// --- Basic register and immediate types ---

/// Integer register identifier (x0-x31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "0", max = "31", name = "Integer register", display = "x{}")]
pub struct IntegerRegister(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "11",
    name = "Saved Integer register",
    display = "s{}"
)]
pub struct SavedIntegerRegister(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "7",
    name = "Compressed Saved Integer register",
    display = "s{}"
)]
pub struct CompressedSavedIntegerRegister(u8);

/// Floating-point register identifier (f0-f31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "31",
    name = "Floating-point register",
    display = "f{}"
)]
pub struct FloatingPointRegister(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "0", max = "31", name = "Vector register", display = "v{}")]
pub struct VectorRegister(u8);

/// Compressed integer register identifier (x8-x15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "8",
    max = "15",
    name = "Compressed integer register",
    skip_display
)]
pub struct CompressedIntegerRegister(u8);

impl Display for CompressedIntegerRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}", self.0)
    }
}

/// Compressed floating-point register identifier (f8-f15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "8",
    max = "15",
    name = "Compressed floating-point register",
    display = "f{}"
)]
pub struct CompressedFloatingPointRegister(u8);

/// Immediate value with bit length validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "-(1i32 << (BITS - 1))",
    max = "(1i32 << (BITS - 1)) - 1",
    name = "Immediate"
)]
pub struct Immediate<const BITS: u8>(i32);

/// Unsigned immediate value with bit length validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "((1u64 << BITS) - 1) as u32",
    name = "Unsigned immediate"
)]
pub struct UImmediate<const BITS: u8>(u32);

/// CSR address
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "0", max = "0xFFF", name = "CSR address", display = "0x{:x}")]
pub struct CSRAddress(u16);

/// Rounding mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "0", max = "7", name = "Rounding mode", skip_display)]
pub struct RoundingMode(u8);

impl Display for RoundingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        let mnemonic = match value {
            0 => "rne",
            1 => "rtz",
            2 => "rdn",
            3 => "rup",
            4 => "rmm",
            5 => "dyn",
            6 => "dyn",
            7 => "dyn",
            _ => unreachable!(),
        };
        write!(f, "{}", mnemonic)
    }
}

/// Fence mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "15",
    name = "Fence mode",
    skip_display,
    forbidden = "0"
)]
pub struct FenceMode(u8);

impl Display for FenceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        let mut mnemonic = String::new();
        if (value & 8) != 0 {
            mnemonic.push('i');
        }
        if (value & 4) != 0 {
            mnemonic.push('o');
        }
        if (value & 2) != 0 {
            mnemonic.push('r');
        }
        if (value & 1) != 0 {
            mnemonic.push('w');
        }
        write!(f, "{}", mnemonic)
    }
}

/// 代表 RISC-V "Zfa" 扩展中 `fli` 指令可以加载的32个硬件常量。
#[derive(Debug, Copy, Clone, PartialEq, DeriveRandom)]
pub enum FliConstant {
    /// 索引 0: -1.0
    NegativeOne,
    /// 索引 1: 最小正规数
    MinNormal,
    /// 索引 2: 1.0 * 2^-16
    PowerOfTwoN16,
    /// 索引 3: 1.0 * 2^-15
    PowerOfTwoN15,
    /// 索引 4: 1.0 * 2^-8
    PowerOfTwoN8,
    /// 索引 5: 1.0 * 2^-7
    PowerOfTwoN7,
    /// 索引 6: 1.0 * 2^-4
    PowerOfTwoN4,
    /// 索引 7: 1.0 * 2^-3
    PowerOfTwoN3,
    /// 索引 8: 0.25
    Point25,
    /// 索引 9: 0.3125
    Point3125,
    /// 索引 10: 0.375
    Point375,
    /// 索引 11: 0.4375
    Point4375,
    /// 索引 12: 0.5
    Point5,
    /// 索引 13: 0.625
    Point625,
    /// 索引 14: 0.75
    Point75,
    /// 索引 15: 0.875
    Point875,
    /// 索引 16: 1.0
    One,
    /// 索引 17: 1.25
    OnePoint25,
    /// 索引 18: 1.5
    OnePoint5,
    /// 索引 19: 1.75
    OnePoint75,
    /// 索引 20: 2.0
    Two,
    /// 索引 21: 2.5
    TwoPoint5,
    /// 索引 22: 3.0
    Three,
    /// 索引 23: 4.0
    Four,
    /// 索引 24: 8.0
    Eight,
    /// 索引 25: 16.0
    Sixteen,
    /// 索引 26: 2^7
    PowerOfTwo7,
    /// 索引 27: 2^8
    PowerOfTwo8,
    /// 索引 28: 2^15
    PowerOfTwo15,
    /// 索引 29: 2^16
    PowerOfTwo16,
    /// 索引 30: +inf
    PositiveInfinity,
    /// 索引 31: NaN
    NaN,
}

// 更新后的 Display 实现
impl fmt::Display for FliConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 直接返回与汇编文件中一致的字面量字符串
        let literal = match self {
            Self::NegativeOne => "-1.0",
            // 改动：直接使用 "min" 字符串
            Self::MinNormal => "min",
            // 改动：直接使用十六进制浮点字面量字符串
            Self::PowerOfTwoN16 => "0x1.0p-16",
            Self::PowerOfTwoN15 => "0x1.0p-15",
            Self::PowerOfTwoN8 => "0x1.0p-8",
            Self::PowerOfTwoN7 => "0x1.0p-7",
            Self::PowerOfTwoN4 => "0x1.0p-4",
            Self::PowerOfTwoN3 => "0x1.0p-3",
            Self::Point25 => "0.25",
            Self::Point3125 => "0.3125",
            Self::Point375 => "0.375",
            Self::Point4375 => "0.4375",
            Self::Point5 => "0.5",
            Self::Point625 => "0.625",
            Self::Point75 => "0.75",
            Self::Point875 => "0.875",
            Self::One => "1.0",
            Self::OnePoint25 => "1.25",
            Self::OnePoint5 => "1.5",
            Self::OnePoint75 => "1.75",
            Self::Two => "2.0",
            Self::TwoPoint5 => "2.5",
            Self::Three => "3.0",
            Self::Four => "4.0",
            Self::Eight => "8.0",
            Self::Sixteen => "16.0",
            Self::PowerOfTwo7 => "128.0",
            Self::PowerOfTwo8 => "256.0",
            Self::PowerOfTwo15 => "32768.0",
            Self::PowerOfTwo16 => "65536.0",
            Self::PositiveInfinity => "inf",
            Self::NaN => "nan",
        };
        write!(f, "{}", literal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedRegListError {
    InvalidRegisterRange,
    InvalidStackAdjustment,
    MismatchedStackSize,
}

impl fmt::Display for SavedRegListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRegisterRange => write!(f, "Invalid register range, must start from s0"),
            Self::InvalidStackAdjustment => write!(f, "Invalid stack adjustment value"),
            Self::MismatchedStackSize => write!(f, "Stack adjustment doesn't match register count"),
        }
    }
}

impl std::error::Error for SavedRegListError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedRegListWithStackAdj<const XLEN: u8> {
    /// 保存的整数寄存器列表的结束 (None 表示只有 ra，Some(n) 表示 s0 到 s[n])
    pub saved_register_end: Option<SavedIntegerRegister>,
    /// 栈调整量 (必须为正数，16字节对齐)
    pub stack_adj: u32,
}

/// RV32I 架构的寄存器列表
pub type SavedRegListWithStackAdjRv32 = SavedRegListWithStackAdj<32>;
/// RV64I 架构的寄存器列表  
pub type SavedRegListWithStackAdjRv64 = SavedRegListWithStackAdj<64>;

impl<const XLEN: u8> SavedRegListWithStackAdj<XLEN> {
    /// 创建新的寄存器列表，自动包含 ra 和 s0 到 s[saved_end]
    pub fn new(saved_end: u8, stack_adj: u32) -> Result<Self, SavedRegListError> {
        // 限制 saved_register_end 不能为 10
        if saved_end == 10 {
            return Err(SavedRegListError::InvalidRegisterRange);
        }

        let saved_register_end = Some(
            SavedIntegerRegister::new(saved_end)
                .map_err(|_| SavedRegListError::InvalidRegisterRange)?,
        );

        // 验证栈调整量
        if stack_adj == 0 || stack_adj % 16 != 0 {
            return Err(SavedRegListError::InvalidStackAdjustment);
        }

        let instance = Self {
            saved_register_end,
            stack_adj,
        };

        // 验证栈大小是否合理
        if !instance.is_valid_combination() {
            return Err(SavedRegListError::MismatchedStackSize);
        }

        Ok(instance)
    }

    /// 只有 ra 的情况
    pub fn ra_only(stack_adj: u32) -> Result<Self, SavedRegListError> {
        if stack_adj == 0 || stack_adj % 16 != 0 {
            return Err(SavedRegListError::InvalidStackAdjustment);
        }

        let instance = Self {
            saved_register_end: None,
            stack_adj,
        };

        // 验证栈大小是否合理
        if !instance.is_valid_combination() {
            return Err(SavedRegListError::MismatchedStackSize);
        }

        Ok(instance)
    }

    /// 获取寄存器数量 (包括 ra)
    pub fn register_count(&self) -> u8 {
        match self.saved_register_end {
            None => 1,                        // 只有 ra
            Some(end) => 1 + (end.get() + 1), // ra + s0 到 s[end]
        }
    }

    /// 是否只有 ra
    pub fn is_ra_only(&self) -> bool {
        self.saved_register_end.is_none()
    }

    /// 根据寄存器数量获取 rlist 编码值 (用于确定 stack_adj_base)
    fn get_rlist_encoding(&self) -> u8 {
        match self.saved_register_end {
            None => 4,                  // {ra}
            Some(end) => 5 + end.get(), // 5 + s[end] 索引
        }
    }

    /// 获取基础栈大小和允许的 stack_adj 值
    fn get_stack_adj_info(&self) -> (u32, Vec<u32>) {
        let rlist = self.get_rlist_encoding();

        match XLEN {
            32 => {
                // RV32I 架构
                match rlist {
                    4..=7 => (16, vec![16, 32, 48, 64]),   // {ra} 至 {ra, s0-s2}
                    8..=11 => (32, vec![32, 48, 64, 80]),  // {ra, s0-s3} 至 {ra, s0-s6}
                    12..=15 => (48, vec![48, 64, 80, 96]), // {ra, s0-s7} 至 {ra, s0-s10}
                    16 => (64, vec![64, 80, 96, 112]),     // {ra, s0-s11}
                    _ => (16, vec![16, 32, 48, 64]),       // Should be unreachable for valid rlist
                }
            }
            64 => {
                // RV64I 架构
                match rlist {
                    4..=5 => (16, vec![16, 32, 48, 64]),      // {ra} 至 {ra, s0}
                    6..=7 => (32, vec![32, 48, 64, 80]),      // {ra, s0-s1} 至 {ra, s0-s2}
                    8..=9 => (48, vec![48, 64, 80, 96]),      // {ra, s0-s3} 至 {ra, s0-s4}
                    10..=11 => (64, vec![64, 80, 96, 112]),   // {ra, s0-s5} 至 {ra, s0-s6}
                    12..=13 => (80, vec![80, 96, 112, 128]),  // {ra, s0-s7} 至 {ra, s0-s8}
                    14..=15 => (96, vec![96, 112, 128, 144]), // {ra, s0-s9} 至 {ra, s0-s10}
                    16 => (112, vec![112, 128, 144, 160]),    // {ra, s0-s11}
                    _ => (16, vec![16, 32, 48, 64]), // Should be unreachable for valid rlist
                }
            }
            _ => panic!("Unsupported XLEN: {}", XLEN),
        }
    }

    /// 验证寄存器数量和栈调整量的组合是否有效
    pub fn is_valid_combination(&self) -> bool {
        let (_, valid_adjustments) = self.get_stack_adj_info();
        valid_adjustments.contains(&self.stack_adj)
    }

    /// 获取所有有效的栈调整量
    pub fn valid_stack_adjustments(&self) -> Vec<u32> {
        let (_, valid_adjustments) = self.get_stack_adj_info();
        valid_adjustments
    }

    pub fn get_saved_reg_list_string(&self) -> String {
        // 像Display实现一样，生成寄存器列表字符串
        let mut reg_list = String::new();
        reg_list.push_str("{ra");
        if let Some(end) = self.saved_register_end {
            let end_val = end.get();
            if end_val == 0 {
                reg_list.push_str(", s0");
            } else {
                reg_list.push_str(&format!(", s0-s{}", end_val));
            }
        }
        reg_list.push_str("}");
        reg_list
    }

    pub fn get_stack_adjustment(&self) -> u32 {
        self.stack_adj
    }
}

impl<const XLEN: u8> Display for SavedRegListWithStackAdj<XLEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        // 总是包含 ra
        write!(f, "ra")?;

        // 如果不是只有 ra，添加 s 寄存器
        if let Some(end) = self.saved_register_end {
            let end_val = end.get();
            if end_val == 0 {
                write!(f, ", s0")?;
            } else {
                write!(f, ", s0-s{}", end_val)?;
            }
        }

        write!(f, "}}, {}", self.stack_adj)
    }
}

impl<const XLEN: u8> Random for SavedRegListWithStackAdj<XLEN> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        // 随机选择寄存器数量
        let end_reg_options: Vec<Option<u8>> = vec![
            None,
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(11),
        ];

        let saved_end = *end_reg_options.choose(rng).unwrap();

        // 创建临时实例以获取有效的 stack_adj 值
        let temp_instance = Self {
            saved_register_end: saved_end.map(|end| SavedIntegerRegister::new(end).unwrap()),
            stack_adj: 16, // 临时值
        };

        let valid_adjustments = temp_instance.valid_stack_adjustments();
        let stack_adj = *valid_adjustments.choose(rng).unwrap();

        match saved_end {
            None => Self::ra_only(stack_adj).unwrap(),
            Some(end) => Self::new(end, stack_adj).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotEqualCompressedSavedIntegerRegisterPair {
    pub reg1: CompressedSavedIntegerRegister,
    pub reg2: CompressedSavedIntegerRegister,
}

impl Random for NotEqualCompressedSavedIntegerRegisterPair {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        let reg1 = CompressedSavedIntegerRegister::random_with_rng(rng);
        let mut reg2 = CompressedSavedIntegerRegister::random_with_rng(rng);

        // 确保 reg2 不等于 reg1
        while reg2 == reg1 {
            reg2 = CompressedSavedIntegerRegister::random_with_rng(rng);
        }

        Self { reg1, reg2 }
    }
}

impl Display for NotEqualCompressedSavedIntegerRegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.reg1, self.reg2)
    }
}
