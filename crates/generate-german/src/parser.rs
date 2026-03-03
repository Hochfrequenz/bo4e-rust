//! Parser for bo4e-core source files.
//!
//! Extracts struct, enum, and field information including German names
//! from serde attributes and doc comments.

use std::fs;
use std::path::Path;
use syn::{Attribute, Expr, Fields, Item, Lit, Meta, Type};
use walkdir::WalkDir;

/// Information about a struct field.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FieldInfo {
    /// English field name (snake_case)
    pub name: String,
    /// German name from serde(alias) or schemars(rename)
    pub german_name: Option<String>,
    /// Full type as string
    pub type_str: String,
    /// Whether the type is Vec<T>
    pub is_vec: bool,
    /// Whether the type is Option<T>
    pub is_option: bool,
    /// Whether the type is Box<T>
    pub is_box: bool,
    /// Inner BO4E type name (e.g., "Address" from Option<Address>)
    pub inner_type: Option<String>,
    /// Whether this field has #[serde(flatten)]
    pub is_flatten: bool,
    /// Value from #[serde(rename = "...")]
    pub serde_rename: Option<String>,
}

/// Information about an enum variant.
#[derive(Debug, Clone)]
pub struct VariantInfo {
    /// English variant name (PascalCase)
    pub name: String,
    /// German name from doc comment pattern "English (German)"
    pub german_name: Option<String>,
    /// Value from #[serde(rename = "...")]
    pub serde_rename: Option<String>,
}

/// Information about a parsed struct.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StructInfo {
    /// English struct name (PascalCase)
    pub name: String,
    /// German type name from schemars(rename)
    pub german_type_name: Option<String>,
    /// Fields in the struct
    pub fields: Vec<FieldInfo>,
    /// Source file name
    pub source_file: String,
    /// Module path ("bo" or "com")
    pub module_path: String,
}

/// Information about a parsed enum.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnumInfo {
    /// English enum name (PascalCase)
    pub name: String,
    /// German type name from schemars(rename)
    pub german_type_name: Option<String>,
    /// Variants in the enum
    pub variants: Vec<VariantInfo>,
    /// Source file name
    pub source_file: String,
    /// Module path ("enums")
    pub module_path: String,
    /// Whether the enum derives Copy
    pub has_copy: bool,
}

/// All parsed data from the crate.
#[derive(Debug)]
pub struct ParsedCrate {
    pub structs: Vec<StructInfo>,
    pub enums: Vec<EnumInfo>,
}

/// Parse all source files in bo4e-core.
pub fn parse_crate(core_path: &Path) -> ParsedCrate {
    let mut structs = Vec::new();
    let mut enums = Vec::new();

    // Parse bo/ directory (business objects)
    parse_directory(&core_path.join("src/bo"), "bo", &mut structs, &mut enums);
    // Parse com/ directory (components)
    parse_directory(&core_path.join("src/com"), "com", &mut structs, &mut enums);
    // Parse enums/ directory
    parse_directory(
        &core_path.join("src/enums"),
        "enums",
        &mut structs,
        &mut enums,
    );

    ParsedCrate { structs, enums }
}

fn parse_directory(
    dir: &Path,
    module_path: &str,
    structs: &mut Vec<StructInfo>,
    enums: &mut Vec<EnumInfo>,
) {
    for entry in WalkDir::new(dir).max_depth(1).into_iter().flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "rs") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            // Skip mod.rs
            if file_name == "mod" {
                continue;
            }
            parse_file(path, file_name, module_path, structs, enums);
        }
    }
}

fn parse_file(
    path: &Path,
    file_name: &str,
    module_path: &str,
    structs: &mut Vec<StructInfo>,
    enums: &mut Vec<EnumInfo>,
) {
    let source = fs::read_to_string(path).expect("Failed to read file");
    let syntax = syn::parse_file(&source).expect("Failed to parse file");

    for item in &syntax.items {
        match item {
            Item::Struct(s) => {
                if let Fields::Named(fields) = &s.fields {
                    let german_type_name = extract_schemars_rename_from_attrs(&s.attrs);
                    let mut field_infos = Vec::new();

                    for field in &fields.named {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        let type_info = analyze_type(&field.ty);
                        let german_name = extract_german_field_name(&field.attrs);
                        let is_flatten = has_serde_flatten(&field.attrs);
                        let serde_rename = extract_serde_rename(&field.attrs);

                        field_infos.push(FieldInfo {
                            name: field_name,
                            german_name,
                            type_str: type_info.full_type,
                            is_vec: type_info.is_vec,
                            is_option: type_info.is_option,
                            is_box: type_info.is_box,
                            inner_type: type_info.inner_type,
                            is_flatten,
                            serde_rename,
                        });
                    }

                    structs.push(StructInfo {
                        name: s.ident.to_string(),
                        german_type_name,
                        fields: field_infos,
                        source_file: file_name.to_string(),
                        module_path: module_path.to_string(),
                    });
                }
            }
            Item::Enum(e) => {
                let german_type_name = extract_schemars_rename_from_attrs(&e.attrs);
                let has_copy = has_derive_copy(&e.attrs);
                let mut variant_infos = Vec::new();

                for variant in &e.variants {
                    let name = variant.ident.to_string();
                    let german_name = extract_german_name_from_doc(&variant.attrs);
                    let serde_rename = extract_serde_rename(&variant.attrs);

                    variant_infos.push(VariantInfo {
                        name,
                        german_name,
                        serde_rename,
                    });
                }

                enums.push(EnumInfo {
                    name: e.ident.to_string(),
                    german_type_name,
                    variants: variant_infos,
                    source_file: file_name.to_string(),
                    module_path: module_path.to_string(),
                    has_copy,
                });
            }
            _ => {}
        }
    }
}

/// Information about a field type after analysis.
struct TypeAnalysis {
    full_type: String,
    is_option: bool,
    is_vec: bool,
    is_box: bool,
    inner_type: Option<String>,
}

/// Analyze a type to extract Option/Vec/Box wrappers and the inner type.
fn analyze_type(ty: &Type) -> TypeAnalysis {
    let type_str = quote::quote!(#ty).to_string();
    // Clean up spacing from quote
    let type_str = type_str
        .replace(" < ", "<")
        .replace(" > ", ">")
        .replace("< ", "<")
        .replace(" >", ">");

    let mut is_option = false;
    let mut is_vec = false;
    let mut is_box = false;
    let mut inner_type = None;

    // Peel off layers
    let innermost = peel_type(ty, &mut is_option, &mut is_vec, &mut is_box);

    // Extract the innermost type name (for BO4E type resolution)
    if let Some(name) = extract_type_name(innermost) {
        // Filter out primitive types - only BO4E types matter
        if !is_primitive_type(&name) {
            inner_type = Some(name);
        }
    }

    TypeAnalysis {
        full_type: type_str,
        is_option,
        is_vec,
        is_box,
        inner_type,
    }
}

/// Recursively peel Option<>, Vec<>, Box<> wrappers from a type.
fn peel_type<'a>(
    ty: &'a Type,
    is_option: &mut bool,
    is_vec: &mut bool,
    is_box: &mut bool,
) -> &'a Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = segment.ident.to_string();
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if args.args.len() == 1 {
                    if let syn::GenericArgument::Type(inner) = &args.args[0] {
                        match ident.as_str() {
                            "Option" => {
                                *is_option = true;
                                return peel_type(inner, is_option, is_vec, is_box);
                            }
                            "Vec" => {
                                *is_vec = true;
                                return peel_type(inner, is_option, is_vec, is_box);
                            }
                            "Box" => {
                                *is_box = true;
                                return peel_type(inner, is_option, is_vec, is_box);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    ty
}

/// Extract the simple type name from a Type, handling `super::Foo` paths.
fn extract_type_name(ty: &Type) -> Option<String> {
    if let Type::Path(type_path) = ty {
        // Get the last segment (handles super::Foo, crate::Foo, etc.)
        if let Some(last) = type_path.path.segments.last() {
            let name = last.ident.to_string();
            // Skip generic types like DateTime<Utc> - we want just the name
            return Some(name);
        }
    }
    None
}

/// Check if a type name is a primitive/standard library type.
fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        "String"
            | "str"
            | "bool"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "f32"
            | "f64"
            | "usize"
            | "isize"
            | "DateTime"
            | "Utc"
            | "Bo4eMeta"
            | "AdditionalAttribute"
    )
}

/// Extract German field name from serde(alias = "...") or schemars(rename = "...").
///
/// Priority: serde alias first, then schemars rename (some fields only have one).
fn extract_german_field_name(attrs: &[Attribute]) -> Option<String> {
    // Try serde alias first
    if let Some(alias) = extract_serde_alias(attrs) {
        return Some(alias);
    }
    // Fall back to schemars rename (inside cfg_attr)
    extract_schemars_rename_from_attrs(attrs)
}

/// Extract alias value from #[serde(..., alias = "value", ...)]
fn extract_serde_alias(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        // Parse the attribute as a list of nested metas
        let nested = attr
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated);
        if let Ok(nested) = nested {
            for meta in &nested {
                if let Meta::NameValue(nv) = meta {
                    if nv.path.is_ident("alias") {
                        if let Expr::Lit(lit) = &nv.value {
                            if let Lit::Str(s) = &lit.lit {
                                return Some(s.value());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract rename value from #[serde(rename = "value")]
fn extract_serde_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let nested = attr
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated);
        if let Ok(nested) = nested {
            for meta in &nested {
                if let Meta::NameValue(nv) = meta {
                    if nv.path.is_ident("rename") {
                        if let Expr::Lit(lit) = &nv.value {
                            if let Lit::Str(s) = &lit.lit {
                                return Some(s.value());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Check if field has #[serde(flatten)].
fn has_serde_flatten(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let nested = attr
            .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated);
        if let Ok(nested) = nested {
            for meta in &nested {
                if let Meta::Path(path) = meta {
                    if path.is_ident("flatten") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Extract rename from #[cfg_attr(feature = "json-schema", schemars(rename = "..."))]
/// or from #[schemars(rename = "...")].
fn extract_schemars_rename_from_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        // Direct schemars attribute
        if attr.path().is_ident("schemars") {
            let nested = attr.parse_args_with(
                syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
            );
            if let Ok(nested) = nested {
                for meta in &nested {
                    if let Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("rename") {
                            if let Expr::Lit(lit) = &nv.value {
                                if let Lit::Str(s) = &lit.lit {
                                    return Some(s.value());
                                }
                            }
                        }
                    }
                }
            }
            continue;
        }

        // cfg_attr wrapping schemars
        if !attr.path().is_ident("cfg_attr") {
            continue;
        }

        // Parse cfg_attr content: we need to find schemars(rename = "...") inside
        // The attribute looks like: #[cfg_attr(feature = "json-schema", schemars(rename = "Zaehler"))]
        // We'll parse the raw token stream to find the schemars part
        let tokens = match &attr.meta {
            Meta::List(list) => &list.tokens,
            _ => continue,
        };

        let token_str = tokens.to_string();
        // Quick check if this cfg_attr contains schemars rename
        if !token_str.contains("schemars") || !token_str.contains("rename") {
            continue;
        }

        // Parse the tokens more carefully
        // Format: feature = "json-schema" , schemars ( rename = "Value" )
        if let Some(rename_val) = extract_schemars_rename_from_cfg_attr_tokens(&token_str) {
            return Some(rename_val);
        }
    }
    None
}

/// Parse the inner tokens of a cfg_attr to find schemars(rename = "...").
fn extract_schemars_rename_from_cfg_attr_tokens(token_str: &str) -> Option<String> {
    // Look for pattern: schemars(rename = "...")
    // or: schemars ( rename = "..." )
    let schemars_pos = token_str.find("schemars")?;
    let after_schemars = &token_str[schemars_pos..];

    // Find the opening paren
    let paren_start = after_schemars.find('(')?;
    let inner = &after_schemars[paren_start + 1..];

    // Find rename = "..."
    let rename_pos = inner.find("rename")?;
    let after_rename = &inner[rename_pos..];

    // Find the = sign
    let eq_pos = after_rename.find('=')?;
    let after_eq = after_rename[eq_pos + 1..].trim();

    // Extract the string value between quotes
    let quote_start = after_eq.find('"')?;
    let rest = &after_eq[quote_start + 1..];
    let quote_end = rest.find('"')?;

    Some(rest[..quote_end].to_string())
}

/// Extract German name from doc comment pattern: `/// English (German)`.
fn extract_german_name_from_doc(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let Meta::NameValue(nv) = &attr.meta {
            if let Expr::Lit(lit) = &nv.value {
                if let Lit::Str(s) = &lit.lit {
                    let doc = s.value();
                    let doc = doc.trim();
                    // Pattern: "English text (German)" or "Some description (German)"
                    if let Some(paren_start) = doc.rfind('(') {
                        if let Some(paren_end) = doc.rfind(')') {
                            if paren_end > paren_start && paren_end == doc.len() - 1 {
                                let german = doc[paren_start + 1..paren_end].trim();
                                if !german.is_empty() && is_valid_german_name(german) {
                                    return Some(german.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Check if a string extracted from a doc comment is a valid German name
/// (not a technical description like "10^18" or "3 months").
fn is_valid_german_name(s: &str) -> bool {
    // Must start with a letter
    if s.chars().next().is_some_and(|c| !c.is_alphabetic()) {
        return false;
    }
    // Must not contain special chars that indicate technical notation
    if s.contains('^') || s.contains('§') || s.contains('/') || s.contains('%') {
        return false;
    }
    true
}

/// Check if the enum derives Copy (from #[derive(..., Copy, ...)]).
fn has_derive_copy(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        let nested = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        );
        if let Ok(paths) = nested {
            for path in paths {
                if path.is_ident("Copy") {
                    return true;
                }
            }
        }
    }
    false
}
