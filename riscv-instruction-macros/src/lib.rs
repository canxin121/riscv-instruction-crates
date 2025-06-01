extern crate proc_macro;

mod code_gen;
mod instruction_display;
mod instruction_types;
mod random;
mod validated_value;

use std::{env, fs, path::PathBuf};

use code_gen::CodeGenerator;
use instruction_display::impl_instruction_display;
use instruction_types::RiscvInstructionDef;
use random::impl_random_derive;
use validated_value::impl_validated_value;

use syn::parse_macro_input;

/// Implements the `InstructionDisplay` derive macro.
/// Requires the `#[asm("...")]` or `#[asm_code(...)]` attribute on enum variants.
#[proc_macro_derive(DeriveInstructionDisplay, attributes(asm, asm_code))]
pub fn instruction_display_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a DeriveInput AST.
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // Call the implementation function from the `assembly` module.
    let output = impl_instruction_display(&ast);

    // Convert the resulting TokenStream back to proc_macro::TokenStream.
    proc_macro::TokenStream::from(output)
}

/// Implements the `Random` derive macro.
/// Supports enums and structs (assuming structs implement `ValidatedValue`).
#[proc_macro_derive(DeriveRandom)]
pub fn random_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a DeriveInput AST.
    let input_ast = parse_macro_input!(input as syn::DeriveInput);

    // Call the implementation function from the `random` module.
    let output_tokens = impl_random_derive(&input_ast);

    // Convert the resulting proc_macro2::TokenStream back to proc_macro::TokenStream.
    proc_macro::TokenStream::from(output_tokens)
}

/// Implements the `DeriveValidatedValue` derive macro.
/// Requires the `#[validated(...)]` attribute on newtype structs.
#[proc_macro_derive(DeriveValidatedValue, attributes(validated))]
pub fn derive_validated_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a DeriveInput AST.
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // Call the implementation function from the `validated_value` module.
    let output_stream = impl_validated_value(&ast);

    // Convert the resulting TokenStream back to proc_macro::TokenStream.
    proc_macro::TokenStream::from(output_stream)
}

#[proc_macro]
pub fn generate_riscv_instructions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 1. 解析传入的路径参数
    let input_str = input.to_string();
    let json_path = input_str.trim_matches('"'); // 移除引号

    // 2. 构建完整路径
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let mut path = PathBuf::from(manifest_dir);
    path.push(json_path);

    // 3. 读取和解析 JSON
    let file_content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
    let instructions: RiscvInstructionDef =
        serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // 4. 创建代码生成器
    let code_generator = CodeGenerator::new(
        instructions.rvc_instructions,
        instructions.standard_instructions,
    );

    // 5. 生成代码 (TokenStream)
    let generated_code = code_generator.generate_instruction_enum();

    // 6. 返回生成的代码
    proc_macro::TokenStream::from(generated_code)
}
