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
    const NOT_ZERO: bool = false;
    const MULTIPLE_OF: Option<T> = None;
    const FORBIDDEN: &'static [T] = &[];
    type Error;
    fn new(value: T) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn get(&self) -> T;
    fn set(&mut self, value: T) -> Result<(), Self::Error>;
}

// --- Restriction marker types ---
/// Marker type for multiple-of restriction
pub struct MultipleOf<const N: u8>;

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
    skip_display
)]
pub struct CompressedFloatingPointRegister(u8);

impl Display for CompressedFloatingPointRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.0)
    }
}

/// Generic register with bit length validation (for backward compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "0",
    max = "((1u16 << BITS) - 1) as u8",
    name = "Register",
    display = "x{}"
)]
pub struct Register<const BITS: u8>(u8);

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
    not_zero
)]
pub struct NZImmediate<const BITS: u8>(i32);

/// Non-zero unsigned immediate value with bit length validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
#[validated(
    min = "1",
    max = "((1u64 << BITS) - 1) as u32",
    name = "Non-zero unsigned immediate",
    not_zero
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
#[validated(min = "0", max = "15", name = "Fence mode", not_zero, skip_display)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipleOfImmediate<const BITS: u8, const MULTIPLE: u8>(i32);

impl<const BITS: u8, const MULTIPLE: u8> ValidatedValue<i32>
    for MultipleOfImmediate<BITS, MULTIPLE>
{
    const MIN: i32 = -(1i32 << (BITS - 1));
    const MAX: i32 = (1i32 << (BITS - 1)) - 1;
    const NOT_ZERO: bool = false;
    const MULTIPLE_OF: Option<i32> = Some(MULTIPLE as i32);
    type Error = String;

    fn new(value: i32) -> Result<Self, Self::Error> {
        if value % (MULTIPLE as i32) != 0 {
            return Err(format!(
                "Value {} must be a multiple of {}",
                value, MULTIPLE
            ));
        }
        if value >= Self::MIN && value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(format!(
                "Value {} is out of range ({} to {})",
                value,
                Self::MIN,
                Self::MAX
            ))
        }
    }

    fn get(&self) -> i32 {
        self.0
    }

    fn set(&mut self, value: i32) -> Result<(), Self::Error> {
        *self = Self::new(value)?;
        Ok(())
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Display for MultipleOfImmediate<BITS, MULTIPLE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Random for MultipleOfImmediate<BITS, MULTIPLE> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        loop {
            let min_multiple = Self::MIN / (MULTIPLE as i32);
            let max_multiple = Self::MAX / (MULTIPLE as i32);
            let multiple_factor = rng.random_range(min_multiple..=max_multiple);
            let value = multiple_factor * (MULTIPLE as i32);

            if let Ok(instance) = Self::new(value) {
                return instance;
            }
        }
    }
}

/// Unsigned immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipleOfUImmediate<const BITS: u8, const MULTIPLE: u8>(u32);

impl<const BITS: u8, const MULTIPLE: u8> ValidatedValue<u32>
    for MultipleOfUImmediate<BITS, MULTIPLE>
{
    const MIN: u32 = 0;
    const MAX: u32 = ((1u64 << BITS) - 1) as u32;
    const NOT_ZERO: bool = false;
    const MULTIPLE_OF: Option<u32> = Some(MULTIPLE as u32);
    type Error = String;

    fn new(value: u32) -> Result<Self, Self::Error> {
        if value % (MULTIPLE as u32) != 0 {
            return Err(format!(
                "Value {} must be a multiple of {}",
                value, MULTIPLE
            ));
        }
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(format!("Value {} exceeds maximum {}", value, Self::MAX))
        }
    }

    fn get(&self) -> u32 {
        self.0
    }

    fn set(&mut self, value: u32) -> Result<(), Self::Error> {
        *self = Self::new(value)?;
        Ok(())
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Display for MultipleOfUImmediate<BITS, MULTIPLE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Random for MultipleOfUImmediate<BITS, MULTIPLE> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        loop {
            let max_multiple = Self::MAX / (MULTIPLE as u32);
            let multiple_factor = rng.random_range(0..=max_multiple);
            let value = multiple_factor * (MULTIPLE as u32);

            if let Ok(instance) = Self::new(value) {
                return instance;
            }
        }
    }
}

/// Non-zero immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipleOfNZImmediate<const BITS: u8, const MULTIPLE: u8>(i32);

impl<const BITS: u8, const MULTIPLE: u8> ValidatedValue<i32>
    for MultipleOfNZImmediate<BITS, MULTIPLE>
{
    const MIN: i32 = -(1i32 << (BITS - 1));
    const MAX: i32 = (1i32 << (BITS - 1)) - 1;
    const NOT_ZERO: bool = true;
    const MULTIPLE_OF: Option<i32> = Some(MULTIPLE as i32);
    type Error = String;

    fn new(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err("Non-zero immediate cannot be zero".to_string());
        }
        if value % (MULTIPLE as i32) != 0 {
            return Err(format!(
                "Value {} must be a multiple of {}",
                value, MULTIPLE
            ));
        }
        if value >= Self::MIN && value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(format!(
                "Value {} is out of range ({} to {})",
                value,
                Self::MIN,
                Self::MAX
            ))
        }
    }

    fn get(&self) -> i32 {
        self.0
    }

    fn set(&mut self, value: i32) -> Result<(), Self::Error> {
        *self = Self::new(value)?;
        Ok(())
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Display for MultipleOfNZImmediate<BITS, MULTIPLE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Random for MultipleOfNZImmediate<BITS, MULTIPLE> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        loop {
            let min_multiple = if Self::MIN >= 0 {
                Self::MIN / (MULTIPLE as i32)
            } else {
                (Self::MIN - (MULTIPLE as i32) + 1) / (MULTIPLE as i32)
            };
            let max_multiple = Self::MAX / (MULTIPLE as i32);
            let multiple_factor = rng.random_range(min_multiple..=max_multiple);
            let value = multiple_factor * (MULTIPLE as i32);

            if value != 0 {
                if let Ok(instance) = Self::new(value) {
                    return instance;
                }
            }
        }
    }
}

/// Non-zero unsigned immediate with multiple-of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultipleOfNZUImmediate<const BITS: u8, const MULTIPLE: u8>(u32);

impl<const BITS: u8, const MULTIPLE: u8> ValidatedValue<u32>
    for MultipleOfNZUImmediate<BITS, MULTIPLE>
{
    const MIN: u32 = 1;
    const MAX: u32 = ((1u64 << BITS) - 1) as u32;
    const NOT_ZERO: bool = true;
    const MULTIPLE_OF: Option<u32> = Some(MULTIPLE as u32);
    type Error = String;

    fn new(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err("Non-zero unsigned immediate cannot be zero".to_string());
        }
        if value % (MULTIPLE as u32) != 0 {
            return Err(format!(
                "Value {} must be a multiple of {}",
                value, MULTIPLE
            ));
        }
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(format!("Value {} exceeds maximum {}", value, Self::MAX))
        }
    }

    fn get(&self) -> u32 {
        self.0
    }

    fn set(&mut self, value: u32) -> Result<(), Self::Error> {
        *self = Self::new(value)?;
        Ok(())
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Display for MultipleOfNZUImmediate<BITS, MULTIPLE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const BITS: u8, const MULTIPLE: u8> Random for MultipleOfNZUImmediate<BITS, MULTIPLE> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        loop {
            let min_multiple = if Self::MIN % (MULTIPLE as u32) == 0 {
                Self::MIN / (MULTIPLE as u32)
            } else {
                (Self::MIN + (MULTIPLE as u32) - 1) / (MULTIPLE as u32)
            };
            let max_multiple = Self::MAX / (MULTIPLE as u32);

            if min_multiple <= max_multiple && min_multiple > 0 {
                let multiple_factor = rng.random_range(min_multiple..=max_multiple);
                let value = multiple_factor * (MULTIPLE as u32);

                if let Ok(instance) = Self::new(value) {
                    return instance;
                }
            } else {
                // Fallback: try to find any valid non-zero multiple
                let value = MULTIPLE as u32;
                if value <= Self::MAX {
                    if let Ok(instance) = Self::new(value) {
                        return instance;
                    }
                }
            }
        }
    }
}

/// Custom range immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeImmediate<const MIN: i32, const MAX: i32>(i32);

impl<const MIN: i32, const MAX: i32> ValidatedValue<i32> for RangeImmediate<MIN, MAX> {
    const MIN: i32 = MIN;
    const MAX: i32 = MAX;
    const NOT_ZERO: bool = false;
    const MULTIPLE_OF: Option<i32> = None;
    type Error = String;

    fn new(value: i32) -> Result<Self, Self::Error> {
        if value >= Self::MIN && value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(format!(
                "Value {} is out of range ({} to {})",
                value,
                Self::MIN,
                Self::MAX
            ))
        }
    }

    fn get(&self) -> i32 {
        self.0
    }

    fn set(&mut self, value: i32) -> Result<(), Self::Error> {
        *self = Self::new(value)?;
        Ok(())
    }
}

impl<const MIN: i32, const MAX: i32> Display for RangeImmediate<MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MIN: i32, const MAX: i32> Random for RangeImmediate<MIN, MAX> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        let random_value = rng.random_range(Self::MIN..=Self::MAX);
        Self::new(random_value).unwrap()
    }
}

/// Non-zero custom range immediate value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonZeroRangeImmediate<const MIN: i32, const MAX: i32>(i32);

impl<const MIN: i32, const MAX: i32> ValidatedValue<i32> for NonZeroRangeImmediate<MIN, MAX> {
    const MIN: i32 = MIN;
    const MAX: i32 = MAX;
    const NOT_ZERO: bool = true;
    const MULTIPLE_OF: Option<i32> = None;
    type Error = String;

    fn new(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err("Non-zero range immediate cannot be zero".to_string());
        }
        if value >= Self::MIN && value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(format!(
                "Value {} is out of range ({} to {})",
                value,
                Self::MIN,
                Self::MAX
            ))
        }
    }

    fn get(&self) -> i32 {
        self.0
    }

    fn set(&mut self, value: i32) -> Result<(), Self::Error> {
        *self = Self::new(value)?;
        Ok(())
    }
}

impl<const MIN: i32, const MAX: i32> Display for NonZeroRangeImmediate<MIN, MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const MIN: i32, const MAX: i32> Random for NonZeroRangeImmediate<MIN, MAX> {
    type Output = Self;

    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
        const MAX_ATTEMPTS: usize = 1000;
        for _ in 0..MAX_ATTEMPTS {
            let random_value = rng.random_range(Self::MIN..=Self::MAX);
            if random_value != 0 {
                if let Ok(instance) = Self::new(random_value) {
                    return instance;
                }
            }
        }
        panic!(
            "Failed to generate a valid non-zero random value for NonZeroRangeImmediate<{}, {}> after {} attempts",
            MIN, MAX, MAX_ATTEMPTS
        );
    }
}

// --- Legacy restricted types (keeping for backward compatibility) ---

/// Restricted register type with additional constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestrictedRegister<const BITS: u8, const RESTRICTION: u8>(u8);

/// Restricted immediate type with additional constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestrictedImmediate<const BITS: u8, const RESTRICTION: u8>(i32);

/// Restricted unsigned immediate type with additional constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestrictedUImmediate<const BITS: u8, const RESTRICTION: u8>(u32);

/// Restricted shift amount type with additional constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestrictedShiftAmount<const BITS: u8, const RESTRICTION: u8>(u8);
