pub mod merged_instructions {
    riscv_instruction_macros::generate_merged_riscv_instructions!(
        "../assets/riscv_instructions.json"
    );
}

pub mod separated_instructions {
    riscv_instruction_macros::generate_separated_riscv_instructions!(
        "../assets/riscv_instructions.json"
    );
}

#[cfg(test)]
mod test {
    use super::separated_instructions::*;
    use std::process::Command;

    #[derive(Debug)]
    enum InstructionTestCase {
        RV32Extension(RV32Extension),
        RV64Extension(RV64Extension),
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum RV32Extension {
        I,
        M,
        F,
        D,
        Q,
        C,
        B,
        V,
        H,
        S,
        Zicsr,
        Zifencei,
        Zicond,
        Zicbom,
        Zicboz,
        Zicfilp,
        Zicfiss,
        Zba,
        Zbb,
        Zbc,
        Zbs,
        Zbkb,
        Zbkx,
        Zknd,
        Zkne,
        Zknh,
        Zks,
        Zcb,
        Zcmp,
        Zcmop,
        Zcd,
        Zcf,
        Zvbb,
        Zvbc,
        Zvkg,
        Zvks,
        Zvkned,
        Zvknha,
        Zvfbfmin,
        Zvfbfwma,
        Zfh,
        Zfbfmin,
        Zaamo,
        Zalasr,
        Zalrsc,
        Zacas,
        Zabha,
        Zawrs,
        Zimop,
        Zilsd,
        Svinval,
        Smrnmi,
        Smdbltrp,
        Sdext,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum RV64Extension {
        I,
        M,
        F,
        D,
        Q,
        C,
        B,
        V,
        H,
        S,
        Zicsr,
        Zifencei,
        Zicond,
        Zicbom,
        Zicboz,
        Zicfilp,
        Zicfiss,
        Zba,
        Zbb,
        Zbc,
        Zbs,
        Zbkb,
        Zbkx,
        Zkn,
        Zknd,
        Zkne,
        Zknh,
        Zks,
        Zcb,
        Zcmp,
        Zcmop,
        Zcd,
        Zvbb,
        Zvbc,
        Zvkg,
        Zvks,
        Zvkned,
        Zvknha,
        Zvfbfmin,
        Zvfbfwma,
        Zfh,
        Zfbfmin,
        Zaamo,
        Zalasr,
        Zalrsc,
        Zacas,
        Zabha,
        Zawrs,
        Zimop,
        Zilsd,
        Svinval,
        Smrnmi,
        Smdbltrp,
        Sdext,
    }

    impl InstructionTestCase {
        fn name(&self) -> String {
            match self {
                InstructionTestCase::RV32Extension(ext) => format!("RV32_{:?}", ext),
                InstructionTestCase::RV64Extension(ext) => format!("RV64_{:?}", ext),
            }
        }

        fn march_flags(&self) -> Vec<String> {
            match self {
                InstructionTestCase::RV32Extension(ext) => match ext {
                    RV32Extension::I => vec!["rv32i".to_string()],
                    RV32Extension::M => vec!["rv32im".to_string()],
                    RV32Extension::F => vec!["rv32if_zfa".to_string()],
                    RV32Extension::D => vec!["rv32ifd_zfa".to_string()],
                    RV32Extension::Q => vec!["rv32ifdq_zfa_zfhmin".to_string()],
                    RV32Extension::C => vec!["rv32ic".to_string()],
                    RV32Extension::B => vec!["rv32i_zba_zbb_zbc_zbs".to_string()],
                    RV32Extension::V => vec!["rv32iv".to_string()],
                    RV32Extension::H => vec!["rv32i_h".to_string()],
                    RV32Extension::S => vec!["rv32i".to_string()], // S extension is implicit
                    RV32Extension::Zicsr => vec!["rv32i_zicsr".to_string()],
                    RV32Extension::Zifencei => vec!["rv32i_zifencei".to_string()],
                    RV32Extension::Zicond => vec!["rv32i_zicond".to_string()],
                    RV32Extension::Zicbom => vec!["rv32i_zicbom".to_string()],
                    RV32Extension::Zicboz => vec!["rv32i_zicboz".to_string()],
                    RV32Extension::Zicfilp => vec!["rv32i_zicfilp".to_string()],
                    RV32Extension::Zicfiss => vec!["rv32i_zicfiss".to_string()],
                    RV32Extension::Zba => vec!["rv32i_zba".to_string()],
                    RV32Extension::Zbb => vec!["rv32i_zbb".to_string()],
                    RV32Extension::Zbc => vec!["rv32i_zbc".to_string()],
                    RV32Extension::Zbs => vec!["rv32i_zbs".to_string()],
                    RV32Extension::Zbkb => vec!["rv32i_zbkb".to_string()],
                    RV32Extension::Zbkx => vec!["rv32i_zbkx".to_string()],
                    RV32Extension::Zknd => vec!["rv32i_zknd".to_string()],
                    RV32Extension::Zkne => vec!["rv32i_zkne".to_string()],
                    RV32Extension::Zknh => vec!["rv32i_zknh".to_string()],
                    RV32Extension::Zks => vec!["rv32i_zks".to_string()],
                    RV32Extension::Zcb => vec![
                        "rv32ic_zcb_zbb_m".to_string(),
                        "rv32ic_zcb_zbb_zmmul".to_string(),
                    ],
                    RV32Extension::Zcmp => vec!["rv32ic_zcmp".to_string()],
                    RV32Extension::Zcmop => vec!["rv32ic_zcmop_zacas".to_string()],
                    RV32Extension::Zcd => vec!["rv32ifd_zcd".to_string()],
                    RV32Extension::Zcf => vec!["rv32if_zcf".to_string()],
                    RV32Extension::Zvbb => vec!["rv32iv_zvbb".to_string()],
                    RV32Extension::Zvbc => vec!["rv32iv_zvbc".to_string()],
                    RV32Extension::Zvkg => vec!["rv32iv_zvkg".to_string()],
                    RV32Extension::Zvks => vec!["rv32iv_zvks".to_string()],
                    RV32Extension::Zvkned => vec!["rv32iv_zvkned".to_string()],
                    RV32Extension::Zvknha => vec!["rv32iv_zvknha".to_string()],
                    RV32Extension::Zvfbfmin => vec!["rv32ifv_zvfbfmin".to_string()],
                    RV32Extension::Zvfbfwma => vec!["rv32ifv_zvfbfwma".to_string()],
                    RV32Extension::Zfh => vec!["rv32ifd_zfh_zfa".to_string()],
                    RV32Extension::Zfbfmin => vec!["rv32if_zfbfmin".to_string()],
                    RV32Extension::Zaamo => vec!["rv32ia_zaamo".to_string()],
                    RV32Extension::Zalasr => vec!["rv32i".to_string()],
                    RV32Extension::Zalrsc => vec!["rv32ia".to_string()],
                    RV32Extension::Zacas => vec!["rv32ia_zacas".to_string()],
                    RV32Extension::Zabha => vec!["rv32ia_zabha_zacas".to_string()],
                    RV32Extension::Zawrs => vec!["rv32i_zawrs".to_string()],
                    RV32Extension::Zimop => vec!["rv32i_zimop".to_string()],
                    RV32Extension::Zilsd => vec!["rv32i".to_string()],
                    RV32Extension::Svinval => vec!["rv32i_svinval".to_string()],
                    RV32Extension::Smrnmi => vec!["rv32i".to_string()],
                    RV32Extension::Smdbltrp => vec!["rv32i_smdbltrp_smctr".to_string()],
                    RV32Extension::Sdext => vec!["rv32i".to_string()],
                },
                InstructionTestCase::RV64Extension(ext) => match ext {
                    RV64Extension::I => vec!["rv64i".to_string()],
                    RV64Extension::M => vec!["rv64im".to_string()],
                    RV64Extension::F => vec!["rv64if_zfa".to_string()],
                    RV64Extension::D => vec!["rv64ifd_zfa".to_string()],
                    RV64Extension::Q => vec!["rv64ifdq_zfa_zfhmin".to_string()],
                    RV64Extension::C => vec!["rv64ic".to_string()],
                    RV64Extension::B => vec!["rv64i_zba_zbb_zbc_zbs".to_string()],
                    RV64Extension::V => vec!["rv64iv".to_string()],
                    RV64Extension::H => vec!["rv64i_h".to_string()],
                    RV64Extension::S => vec!["rv64i".to_string()],
                    RV64Extension::Zicsr => vec!["rv64i_zicsr".to_string()],
                    RV64Extension::Zifencei => vec!["rv64i_zifencei".to_string()],
                    RV64Extension::Zicond => vec!["rv64i_zicond".to_string()],
                    RV64Extension::Zicbom => vec!["rv64i_zicbom".to_string()],
                    RV64Extension::Zicboz => vec!["rv64i_zicboz".to_string()],
                    RV64Extension::Zicfilp => vec!["rv64i_zicfilp".to_string()],
                    RV64Extension::Zicfiss => vec!["rv64i_zicfiss".to_string()],
                    RV64Extension::Zba => vec!["rv64i_zba".to_string()],
                    RV64Extension::Zbb => vec!["rv64i_zbb".to_string()],
                    RV64Extension::Zbc => vec!["rv64i_zbc".to_string()],
                    RV64Extension::Zbs => vec!["rv64i_zbs".to_string()],
                    RV64Extension::Zbkb => vec!["rv64i_zbkb".to_string()],
                    RV64Extension::Zbkx => vec!["rv64i_zbkx".to_string()],
                    RV64Extension::Zkn => vec!["rv64i_zkn".to_string()],
                    RV64Extension::Zknd => vec!["rv64i_zknd".to_string()],
                    RV64Extension::Zkne => vec!["rv64i_zkne".to_string()],
                    RV64Extension::Zknh => vec!["rv64i_zknh".to_string()],
                    RV64Extension::Zks => vec!["rv64i_zks".to_string()],
                    RV64Extension::Zcb => vec![
                        "rv64ic_zcb_zbb_zba_m".to_string(),
                        "rv64ic_zcb_zbb_zba_zmmul".to_string(),
                    ],
                    RV64Extension::Zcmp => vec!["rv64ic_zcmp".to_string()],
                    RV64Extension::Zcmop => vec!["rv64ic_zcmop".to_string()],
                    RV64Extension::Zcd => vec!["rv64ifd_zcd".to_string()],
                    RV64Extension::Zvbb => vec!["rv64iv_zvbb".to_string()],
                    RV64Extension::Zvbc => vec!["rv64iv_zvbc".to_string()],
                    RV64Extension::Zvkg => vec!["rv64iv_zvkg".to_string()],
                    RV64Extension::Zvks => vec!["rv64iv_zvks".to_string()],
                    RV64Extension::Zvkned => vec!["rv64iv_zvkned".to_string()],
                    RV64Extension::Zvknha => vec!["rv64iv_zvknha".to_string()],
                    RV64Extension::Zvfbfmin => vec!["rv64ifv_zvfbfmin".to_string()],
                    RV64Extension::Zvfbfwma => vec!["rv64ifv_zvfbfwma".to_string()],
                    RV64Extension::Zfh => vec!["rv64ifd_zfh_zfa".to_string()],
                    RV64Extension::Zfbfmin => vec!["rv64if_zfbfmin".to_string()],
                    RV64Extension::Zaamo => vec!["rv64ia_zaamo".to_string()],
                    RV64Extension::Zalasr => vec!["rv64i".to_string()],
                    RV64Extension::Zalrsc => vec!["rv64ia".to_string()],
                    RV64Extension::Zacas => vec!["rv64ia_zacas".to_string()],
                    RV64Extension::Zabha => vec!["rv64ia_zabha_zacas".to_string()],
                    RV64Extension::Zawrs => vec!["rv64i_zawrs".to_string()],
                    RV64Extension::Zimop => vec!["rv64i_zimop".to_string()],
                    RV64Extension::Zilsd => vec!["rv64i".to_string()],
                    RV64Extension::Svinval => vec!["rv64i_svinval".to_string()],
                    RV64Extension::Smrnmi => vec!["rv64i".to_string()],
                    RV64Extension::Smdbltrp => vec!["rv64i_smdbltrp_smctr".to_string()],
                    RV64Extension::Sdext => vec!["rv64i".to_string()],
                },
            }
        }
    }

    fn generate_instructions(test_case: &InstructionTestCase, count: usize) -> Vec<String> {
        let mut rng = rand::rng();

        (0..count)
            .map(|_| match test_case {
                InstructionTestCase::RV32Extension(ext) => match ext {
                    RV32Extension::I => RV32IInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::M => RV32MInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::F => RV32FInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::D => RV32DInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::Q => RV32QInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::C => RV32CInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::B => RV32BInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::V => RV32VInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::H => RV32HInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::S => RV32SInstructions::random_with_rng(&mut rng).to_string(),
                    RV32Extension::Zicsr => {
                        RV32ZicsrInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zifencei => {
                        RV32ZifenceiInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zicond => {
                        RV32ZicondInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zicbom => {
                        RV32ZicbomInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zicboz => {
                        RV32ZicbozInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zicfilp => {
                        RV32ZicfilpInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zicfiss => {
                        RV32ZicfissInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zba => {
                        RV32ZbaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zbb => {
                        RV32ZbbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zbc => {
                        RV32ZbcInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zbs => {
                        RV32ZbsInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zbkb => {
                        RV32ZbkbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zbkx => {
                        RV32ZbkxInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zknd => {
                        RV32ZkndInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zkne => {
                        RV32ZkneInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zknh => {
                        RV32ZknhInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zks => {
                        RV32ZksInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zcb => {
                        RV32ZcbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zcmp => {
                        RV32ZcmpInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zcmop => {
                        RV32ZcmopInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zcd => {
                        RV32ZcdInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zcf => {
                        RV32ZcfInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvbb => {
                        RV32ZvbbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvbc => {
                        RV32ZvbcInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvkg => {
                        RV32ZvkgInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvks => {
                        RV32ZvksInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvkned => {
                        RV32ZvknedInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvknha => {
                        RV32ZvknhaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvfbfmin => {
                        RV32ZvfbfminInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zvfbfwma => {
                        RV32ZvfbfwmaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zfh => {
                        RV32ZfhInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zfbfmin => {
                        RV32ZfbfminInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zaamo => {
                        RV32ZaamoInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zalasr => {
                        RV32ZalasrInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zalrsc => {
                        RV32ZalrscInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zacas => {
                        RV32ZacasInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zabha => {
                        RV32ZabhaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zawrs => {
                        RV32ZawrsInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zimop => {
                        RV32ZimopInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Zilsd => {
                        RV32ZilsdInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Svinval => {
                        RV32SvinvalInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Smrnmi => {
                        RV32SmrnmiInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Smdbltrp => {
                        RV32SmdbltrpInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV32Extension::Sdext => {
                        RV32SdextInstructions::random_with_rng(&mut rng).to_string()
                    }
                },
                InstructionTestCase::RV64Extension(ext) => match ext {
                    RV64Extension::I => RV64IInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::M => RV64MInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::F => RV64FInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::D => RV64DInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::Q => RV64QInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::C => RV64CInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::B => RV64BInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::V => RV64VInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::H => RV64HInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::S => RV64SInstructions::random_with_rng(&mut rng).to_string(),
                    RV64Extension::Zicsr => {
                        RV64ZicsrInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zifencei => {
                        RV64ZifenceiInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zicond => {
                        RV64ZicondInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zicbom => {
                        RV64ZicbomInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zicboz => {
                        RV64ZicbozInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zicfilp => {
                        RV64ZicfilpInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zicfiss => {
                        RV64ZicfissInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zba => {
                        RV64ZbaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zbb => {
                        RV64ZbbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zbc => {
                        RV64ZbcInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zbs => {
                        RV64ZbsInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zbkb => {
                        RV64ZbkbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zbkx => {
                        RV64ZbkxInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zkn => {
                        RV64ZknInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zknd => {
                        RV64ZkndInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zkne => {
                        RV64ZkneInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zknh => {
                        RV64ZknhInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zks => {
                        RV64ZksInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zcb => {
                        RV64ZcbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zcmp => {
                        RV64ZcmpInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zcmop => {
                        RV64ZcmopInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zcd => {
                        RV64ZcdInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvbb => {
                        RV64ZvbbInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvbc => {
                        RV64ZvbcInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvkg => {
                        RV64ZvkgInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvks => {
                        RV64ZvksInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvkned => {
                        RV64ZvknedInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvknha => {
                        RV64ZvknhaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvfbfmin => {
                        RV64ZvfbfminInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zvfbfwma => {
                        RV64ZvfbfwmaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zfh => {
                        RV64ZfhInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zfbfmin => {
                        RV64ZfbfminInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zaamo => {
                        RV64ZaamoInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zalasr => {
                        RV64ZalasrInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zalrsc => {
                        RV64ZalrscInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zacas => {
                        RV64ZacasInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zabha => {
                        RV64ZabhaInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zawrs => {
                        RV64ZawrsInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zimop => {
                        RV64ZimopInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Zilsd => {
                        RV64ZilsdInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Svinval => {
                        RV64SvinvalInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Smrnmi => {
                        RV64SmrnmiInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Smdbltrp => {
                        RV64SmdbltrpInstructions::random_with_rng(&mut rng).to_string()
                    }
                    RV64Extension::Sdext => {
                        RV64SdextInstructions::random_with_rng(&mut rng).to_string()
                    }
                },
            })
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
        test_case: &InstructionTestCase,
        instructions: &[String],
        errors: &[(String, String)],
    ) -> std::io::Result<()> {
        // 创建错误日志目录
        std::fs::create_dir_all("error_logs")?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let log_filename = format!("error_logs/{}_{}_errors.log", test_case.name(), timestamp);

        // 在内存中构造完整的日志内容
        let mut log_content = String::new();

        log_content.push_str("=== 错误日志 ===\n");
        log_content.push_str(&format!("指令集类型: {}\n", test_case.name()));
        log_content.push_str(&format!("时间: {}\n", timestamp));
        log_content.push_str(&format!("指令数量: {}\n", instructions.len()));
        log_content.push_str("\n");

        for (march, error_info) in errors {
            log_content.push_str(&format!("=== -march={} 失败详情 ===\n", march));
            log_content.push_str(error_info);
            log_content.push_str("\n\n");
        }

        log_content.push_str("=== 生成的指令 ===\n");
        for (i, inst) in instructions.iter().enumerate() {
            log_content.push_str(&format!("{:4}: {}\n", i + 1, inst));
        }

        // 一次性写入完整日志内容
        std::fs::write(&log_filename, log_content)?;

        // 同时保存汇编文件到错误日志目录
        let asm_filename = format!("error_logs/{}_{}.S", test_case.name(), timestamp);
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

        // 创建完整的测试用例列表
        let mut test_cases = Vec::new();

        // RV32 所有扩展
        let all_rv32_extensions = [
            RV32Extension::I,
            RV32Extension::M,
            RV32Extension::F,
            RV32Extension::D,
            RV32Extension::Q,
            RV32Extension::C,
            RV32Extension::B,
            RV32Extension::V,
            RV32Extension::H,
            RV32Extension::S,
            RV32Extension::Zicsr,
            RV32Extension::Zifencei,
            RV32Extension::Zicond,
            RV32Extension::Zicbom,
            RV32Extension::Zicboz,
            RV32Extension::Zicfilp,
            RV32Extension::Zicfiss,
            RV32Extension::Zba,
            RV32Extension::Zbb,
            RV32Extension::Zbc,
            RV32Extension::Zbs,
            RV32Extension::Zbkb,
            RV32Extension::Zbkx,
            RV32Extension::Zknd,
            RV32Extension::Zkne,
            RV32Extension::Zknh,
            RV32Extension::Zks,
            RV32Extension::Zcb,
            RV32Extension::Zcmp,
            RV32Extension::Zcmop,
            RV32Extension::Zcd,
            RV32Extension::Zcf,
            RV32Extension::Zvbb,
            RV32Extension::Zvbc,
            RV32Extension::Zvkg,
            RV32Extension::Zvks,
            RV32Extension::Zvkned,
            RV32Extension::Zvknha,
            RV32Extension::Zvfbfmin,
            RV32Extension::Zvfbfwma,
            RV32Extension::Zfh,
            RV32Extension::Zfbfmin,
            RV32Extension::Zaamo,
            // riscv32-unknown-elf-as 现在不支持 zalasr
            // RV32Extension::Zalasr,
            RV32Extension::Zalrsc,
            RV32Extension::Zacas,
            RV32Extension::Zabha,
            RV32Extension::Zawrs,
            RV32Extension::Zimop,
            // riscv64-unknown-elf-as 现在不支持 zilsd
            // RV32Extension::Zilsd,
            RV32Extension::Svinval,
            // riscv64-unknown-elf-as 现在不支持 smrnmi
            // RV32Extension::Smrnmi,
            RV32Extension::Smdbltrp,
            RV32Extension::Sdext,
        ];

        for ext in all_rv32_extensions {
            test_cases.push(InstructionTestCase::RV32Extension(ext));
        }

        // RV64 所有扩展
        let all_rv64_extensions = [
            RV64Extension::I,
            RV64Extension::M,
            RV64Extension::F,
            RV64Extension::D,
            RV64Extension::Q,
            RV64Extension::C,
            RV64Extension::B,
            RV64Extension::V,
            RV64Extension::H,
            RV64Extension::S,
            RV64Extension::Zicsr,
            RV64Extension::Zifencei,
            RV64Extension::Zicond,
            RV64Extension::Zicbom,
            RV64Extension::Zicboz,
            RV64Extension::Zicfilp,
            RV64Extension::Zicfiss,
            RV64Extension::Zba,
            RV64Extension::Zbb,
            RV64Extension::Zbc,
            RV64Extension::Zbs,
            RV64Extension::Zbkb,
            RV64Extension::Zbkx,
            RV64Extension::Zkn,
            RV64Extension::Zknd,
            RV64Extension::Zkne,
            RV64Extension::Zknh,
            RV64Extension::Zks,
            RV64Extension::Zcb,
            RV64Extension::Zcmp,
            RV64Extension::Zcmop,
            RV64Extension::Zcd,
            RV64Extension::Zvbb,
            RV64Extension::Zvbc,
            RV64Extension::Zvkg,
            RV64Extension::Zvks,
            RV64Extension::Zvkned,
            RV64Extension::Zvknha,
            RV64Extension::Zvfbfmin,
            RV64Extension::Zvfbfwma,
            RV64Extension::Zfh,
            RV64Extension::Zfbfmin,
            RV64Extension::Zaamo,
            // riscv64-unknown-elf-as 现在不支持 zalasr
            // RV64Extension::Zalasr,
            RV64Extension::Zalrsc,
            RV64Extension::Zacas,
            RV64Extension::Zabha,
            RV64Extension::Zawrs,
            RV64Extension::Zimop,
            // riscv64-unknown-elf-as 现在不支持 zilsd
            // RV64Extension::Zilsd,
            RV64Extension::Svinval,
            // riscv64-unknown-elf-as 现在不支持 smrnmi
            // RV64Extension::Smrnmi,
            RV64Extension::Smdbltrp,
            RV64Extension::Sdext,
        ];

        for ext in all_rv64_extensions {
            test_cases.push(InstructionTestCase::RV64Extension(ext));
        }

        let mut total_success = 0;
        let mut total_tests = 0;
        let mut failed_cases = Vec::new();

        for test_case in test_cases {
            println!("测试 {}:", test_case.name());

            let instructions = generate_instructions(&test_case, 10000);
            let filename = format!("test_{}.S", test_case.name());

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

            let mut case_success = 0;
            let march_flags = test_case.march_flags();
            let case_total = march_flags.len();
            let mut case_errors = Vec::new();

            for march in march_flags {
                print!("  测试 -march={} ... ", march);
                let (success, error_info) = test_assembly(&filename, &march);
                if success {
                    println!("✓ 成功");
                    case_success += 1;
                } else {
                    println!("✗ 失败");
                    if !error_info.is_empty() {
                        println!("    === 详细错误信息 ===");
                        for line in error_info.lines() {
                            println!("    {}", line);
                        }
                        println!("    === 错误信息结束 ===");
                        case_errors.push((march, error_info));
                    }
                }
            }

            // 如果有失败的测试，保存错误日志
            if !case_errors.is_empty() {
                println!(
                    "  ⚠️  {} 存在失败的测试用例，详细信息已输出到控制台",
                    test_case.name()
                );
                if let Err(e) = create_error_log(&test_case, &instructions, &case_errors) {
                    println!("    警告: 无法创建错误日志: {}", e);
                }
                failed_cases.push(test_case.name());
            }

            println!(
                "  {}: {}/{} 测试通过\n",
                test_case.name(),
                case_success,
                case_total
            );

            total_success += case_success;
            total_tests += case_total;

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
