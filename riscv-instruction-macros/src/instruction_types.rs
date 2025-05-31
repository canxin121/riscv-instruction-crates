use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiscvInstructionDef {
    pub rvc_instructions: Vec<Instruction>,
    pub standard_instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ISAExtension {
    I,
    M,
    A,
    F,
    D,
    Q,
    C,
    Zifencei,
    Zicsr,
}
impl Display for ISAExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ISAExtension::I => write!(f, "I"),
            ISAExtension::M => write!(f, "M"),
            ISAExtension::A => write!(f, "A"),
            ISAExtension::F => write!(f, "F"),
            ISAExtension::D => write!(f, "D"),
            ISAExtension::Q => write!(f, "Q"),
            ISAExtension::C => write!(f, "C"),
            ISAExtension::Zifencei => write!(f, "Zifencei"),
            ISAExtension::Zicsr => write!(f, "Zicsr"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ISABase {
    RV32,
    RV64,
}
impl Display for ISABase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ISABase::RV32 => write!(f, "RV32"),
            ISABase::RV64 => write!(f, "RV64"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ISAModule {
    pub base: ISABase,
    pub extension: ISAExtension,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operand {
    pub name: String,
    pub bit_lengths: HashMap<ISABase, u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub name: String,
    pub extension: ISAExtension,
    pub isa_bases: Vec<ISABase>,
    pub operands: Vec<Operand>,
    pub assembly_syntax: String,
}
