//! Code generator for German BO4E types.
//!
//! Parses bo4e-core source files, extracts German names from serde/schemars attributes,
//! and generates German versions of all types with From/Into conversions.

mod codegen;
mod parser;
mod writer;

use std::collections::HashMap;
use std::path::PathBuf;

use codegen::{generate_german_enum, generate_german_struct, TypeResolver};
use parser::parse_crate;
use writer::{write_generated_files, GeneratedFile};

/// Mapping structure matching mapping.toml format.
#[derive(serde::Deserialize)]
struct Mapping {
    bo: HashMap<String, String>,
    com: HashMap<String, String>,
    enums: HashMap<String, String>,
}

fn main() {
    let toml_str = include_str!("../mapping.toml");
    let mapping: Mapping = toml::from_str(toml_str).expect("Failed to parse mapping.toml");

    println!(
        "Loaded mappings: {} BOs, {} COMs, {} enums",
        mapping.bo.len(),
        mapping.com.len(),
        mapping.enums.len()
    );

    // Build the combined type map for the resolver
    let mut all_types: HashMap<String, String> = HashMap::new();
    all_types.extend(mapping.bo.iter().map(|(k, v)| (k.clone(), v.clone())));
    all_types.extend(mapping.com.iter().map(|(k, v)| (k.clone(), v.clone())));
    all_types.extend(mapping.enums.iter().map(|(k, v)| (k.clone(), v.clone())));

    let resolver = TypeResolver::new(all_types);

    // Parse bo4e-core source files
    let core_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("bo4e-core");

    println!("Parsing bo4e-core at: {}", core_path.display());
    let parsed = parse_crate(&core_path);

    println!(
        "Parsed {} structs, {} enums",
        parsed.structs.len(),
        parsed.enums.len()
    );

    // Generate German code
    let mut bo_files = Vec::new();
    let mut com_files = Vec::new();
    let mut enum_files = Vec::new();

    // Generate structs
    for info in &parsed.structs {
        let german_name = match info.module_path.as_str() {
            "bo" => mapping.bo.get(&info.name),
            "com" => mapping.com.get(&info.name),
            _ => None,
        };

        let german_name = match german_name {
            Some(name) => name.clone(),
            None => {
                eprintln!(
                    "Warning: No German mapping for struct {} in {}",
                    info.name, info.module_path
                );
                continue;
            }
        };

        let tokens = generate_german_struct(info, &german_name, &resolver);
        let file = GeneratedFile {
            type_name: german_name,
            tokens,
        };

        match info.module_path.as_str() {
            "bo" => bo_files.push(file),
            "com" => com_files.push(file),
            _ => {}
        }
    }

    // Generate enums
    for info in &parsed.enums {
        let german_name = match mapping.enums.get(&info.name) {
            Some(name) => name.clone(),
            None => {
                eprintln!("Warning: No German mapping for enum {}", info.name);
                continue;
            }
        };

        let tokens = generate_german_enum(info, &german_name);
        enum_files.push(GeneratedFile {
            type_name: german_name,
            tokens,
        });
    }

    println!(
        "Generated: {} BOs, {} COMs, {} enums",
        bo_files.len(),
        com_files.len(),
        enum_files.len()
    );

    // Write output
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("bo4e-german")
        .join("src")
        .join("generated");

    println!("Writing to: {}", output_dir.display());
    write_generated_files(&output_dir, bo_files, com_files, enum_files);

    println!("Done!");
}
