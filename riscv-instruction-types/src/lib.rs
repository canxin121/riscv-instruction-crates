//! RISC-V instruction types and register utilities

use std::fmt::{self, Display};

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
    type Error;
    fn new(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn get(&self) -> T;
    fn set(&mut self, value: T) -> Result<(), Self::Error>;
}

// --- Basic register and immediate types ---

/// Integer register identifier (x0-x31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "0", max = "31", name = "Integer register", display = "x{}")]
pub struct IntegerRegister(u8);

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

/// Non-zero immediate value with bit length validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "-(1i32 << (BITS - 1))",
    max = "(1i32 << (BITS - 1)) - 1",
    name = "Non-zero immediate",
    forbidden = "0"
)]
pub struct NZImmediate<const BITS: u8>(i32);

/// Non-zero unsigned immediate value with bit length validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "1",
    max = "((1u64 << BITS) - 1) as u32",
    name = "Non-zero unsigned immediate",
    forbidden = "0"
)]
pub struct NZUImmediate<const BITS: u8>(u32);

/// Shift amount with bit length validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "0", max = "((1u16 << BITS) - 1) as u8", name = "Shift amount")]
pub struct ShiftAmount<const BITS: u8>(u8);

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
#[validated(min = "0", max = "15", name = "Fence mode", forbidden = "0", skip_display)]
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

// --- Restricted types with multiple_of constraints ---

/// Immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "-(1i32 << (BITS - 1))",
    max = "(1i32 << (BITS - 1)) - 1",
    name = "Immediate",
    multiple_of = "MULTIPLE"
)]
pub struct MultipleOfImmediate<const BITS: u8, const MULTIPLE: i32>(i32);

/// Unsigned immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "((1u64 << BITS) - 1) as u32",
    name = "Unsigned immediate",
    multiple_of = "MULTIPLE"
)]
pub struct MultipleOfUImmediate<const BITS: u8, const MULTIPLE: u32>(u32);

/// Non-zero immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "-(1i32 << (BITS - 1))",
    max = "(1i32 << (BITS - 1)) - 1",
    name = "Non-zero immediate",
    forbidden = "0",
    multiple_of = "MULTIPLE"
)]
pub struct MultipleOfNZImmediate<const BITS: u8, const MULTIPLE: i32>(i32);

/// Non-zero unsigned immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "1",
    max = "((1u64 << BITS) - 1) as u32",
    name = "Non-zero unsigned immediate",
    forbidden = "0",
    multiple_of = "MULTIPLE"
)]
pub struct MultipleOfNZUImmediate<const BITS: u8, const MULTIPLE: u32>(u32);

/// Custom range immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(min = "MIN", max = "MAX", name = "Custom range immediate")]
pub struct RangeImmediate<const MIN: i32, const MAX: i32>(i32);

/// Non-zero custom range immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "MIN",
    max = "MAX",
    name = "Non-zero custom range immediate",
    forbidden = "0"
)]
pub struct NonZeroRangeImmediate<const MIN: i32, const MAX: i32>(i32);