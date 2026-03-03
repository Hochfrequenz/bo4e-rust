//! File writer for generated German BO4E code.
//!
//! Writes generated Rust source files and mod.rs files with proper
//! module declarations and re-exports.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use proc_macro2::TokenStream;

use crate::codegen::pascal_to_snake;

/// A single generated file.
pub struct GeneratedFile {
    /// German type name (PascalCase, e.g., "Zaehler")
    pub type_name: String,
    /// Generated token stream
    pub tokens: TokenStream,
}

/// Write all generated files to the output directory.
///
/// Creates separate subdirectories for bo, com, and enums,
/// each with their own mod.rs.
pub fn write_generated_files(
    output_dir: &Path,
    bo_files: Vec<GeneratedFile>,
    com_files: Vec<GeneratedFile>,
    enum_files: Vec<GeneratedFile>,
) {
    // Clean and recreate the output directory
    if output_dir.exists() {
        fs::remove_dir_all(output_dir).expect("Failed to clean output directory");
    }
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Write each category
    write_category(output_dir, "bo", bo_files);
    write_category(output_dir, "com", com_files);
    write_category(output_dir, "enums", enum_files);

    // Write the top-level mod.rs
    write_top_level_mod(output_dir);
}

/// Write files for one category (bo, com, or enums).
fn write_category(output_dir: &Path, category: &str, files: Vec<GeneratedFile>) {
    let cat_dir = output_dir.join(category);
    fs::create_dir_all(&cat_dir).expect("Failed to create category directory");

    // Use BTreeMap for sorted output
    let mut modules: BTreeMap<String, String> = BTreeMap::new();

    for file in &files {
        let file_name = pascal_to_snake(&file.type_name);
        let file_path = cat_dir.join(format!("{}.rs", file_name));

        // Format the generated code using prettyplease
        let syntax_tree = syn::parse2(file.tokens.clone()).unwrap_or_else(|e| {
            panic!(
                "Failed to parse generated code for {}: {}\nTokens: {}",
                file.type_name, e, file.tokens
            );
        });
        let formatted = prettyplease::unparse(&syntax_tree);

        fs::write(&file_path, formatted)
            .unwrap_or_else(|e| panic!("Failed to write {}: {}", file_path.display(), e));

        modules.insert(file_name, file.type_name.clone());
    }

    // Write mod.rs for this category
    let mut mod_content = String::new();
    mod_content.push_str("// @generated — do not edit by hand.\n\n");

    for (file_name, type_name) in &modules {
        mod_content.push_str(&format!("mod {};\n", file_name));
        mod_content.push_str(&format!("pub use {}::{};\n\n", file_name, type_name));
    }

    let mod_path = cat_dir.join("mod.rs");
    fs::write(&mod_path, mod_content)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", mod_path.display(), e));
}

/// Write the top-level generated/mod.rs that re-exports all categories.
fn write_top_level_mod(output_dir: &Path) {
    let content = "// @generated — do not edit by hand.\n\n\
        pub mod bo;\n\
        pub mod com;\n\
        pub mod enums;\n";

    let mod_path = output_dir.join("mod.rs");
    fs::write(&mod_path, content)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", mod_path.display(), e));
}
