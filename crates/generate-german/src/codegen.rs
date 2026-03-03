//! Code generation for German BO4E types.
//!
//! Generates German versions of structs and enums with From/Into conversions.

use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::parser::{EnumInfo, FieldInfo, StructInfo};

/// Resolves English BO4E type names to German equivalents.
pub struct TypeResolver {
    /// English name -> German name mapping
    type_map: HashMap<String, String>,
}

impl TypeResolver {
    pub fn new(type_map: HashMap<String, String>) -> Self {
        Self { type_map }
    }

    /// Check if a type name is a BO4E type that has a German mapping.
    pub fn is_bo4e_type(&self, name: &str) -> bool {
        self.type_map.contains_key(name)
    }

    /// Get the German name for an English type name.
    pub fn german_name(&self, english_name: &str) -> Option<&str> {
        self.type_map.get(english_name).map(|s| s.as_str())
    }
}

/// Convert camelCase to snake_case.
pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                // Check if previous char is lowercase or next char is lowercase
                let prev_lower = s.chars().nth(i - 1).is_some_and(|p| p.is_lowercase());
                let next_lower = s.chars().nth(i + 1).is_some_and(|n| n.is_lowercase());
                if prev_lower || next_lower {
                    result.push('_');
                }
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert snake_case to camelCase.
pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert PascalCase to snake_case (for file names).
pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Generate a German enum with From/Into conversions.
pub fn generate_german_enum(info: &EnumInfo, german_name: &str) -> TokenStream {
    let german_ident = Ident::new(german_name, Span::call_site());
    let english_ident = Ident::new(&info.name, Span::call_site());

    // Build the enum variants, deduplicating by German variant name.
    // When multiple English variants map to the same German name, only the first
    // becomes a variant definition; the rest just get From arms pointing to it.
    let mut variant_defs = Vec::new();
    let mut from_english_arms = Vec::new();
    let mut from_german_arms = Vec::new();
    let mut seen_german_variants: HashSet<String> = HashSet::new();

    for variant in &info.variants {
        let english_variant = Ident::new(&variant.name, Span::call_site());

        // German variant name: use extracted German name or fall back to English
        let german_variant_name = variant.german_name.as_deref().unwrap_or(&variant.name);
        // Clean up the German variant name to be a valid Rust identifier
        let sanitized = sanitize_variant_name(german_variant_name);
        let german_variant_ident = Ident::new(&sanitized, Span::call_site());

        // Only emit the variant definition once
        if seen_german_variants.insert(sanitized.clone()) {
            // Preserve serde(rename) if present
            let serde_attr = if let Some(ref rename) = variant.serde_rename {
                quote! { #[serde(rename = #rename)] }
            } else {
                quote! {}
            };

            variant_defs.push(quote! {
                #serde_attr
                #german_variant_ident
            });

            // Only emit the German->English arm once (first English variant wins)
            from_german_arms.push(quote! {
                #german_ident::#german_variant_ident => bo4e_core::enums::#english_ident::#english_variant
            });
        }

        // Every English variant maps to its German variant (many-to-one is fine here)
        from_english_arms.push(quote! {
            bo4e_core::enums::#english_ident::#english_variant => #german_ident::#german_variant_ident
        });
    }

    // Add wildcard arm for #[non_exhaustive] enums (all our enums are)
    // We need a fallback for forward compatibility
    from_english_arms.push(quote! {
        _ => panic!("Unknown {} variant", stringify!(#english_ident))
    });

    let derive_copy = if info.has_copy {
        quote! { Copy, }
    } else {
        quote! {}
    };

    quote! {
        #[derive(Debug, Clone, #derive_copy PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[non_exhaustive]
        #[allow(non_camel_case_types)]
        pub enum #german_ident {
            #(#variant_defs,)*
        }

        impl From<bo4e_core::enums::#english_ident> for #german_ident {
            fn from(v: bo4e_core::enums::#english_ident) -> Self {
                #[allow(unreachable_patterns)]
                match v {
                    #(#from_english_arms,)*
                }
            }
        }

        impl From<#german_ident> for bo4e_core::enums::#english_ident {
            fn from(v: #german_ident) -> Self {
                #[allow(unreachable_patterns)]
                match v {
                    #(#from_german_arms,)*
                }
            }
        }
    }
}

/// Sanitize a German name for use as a Rust variant identifier.
/// Removes spaces, hyphens, and special characters, converts to PascalCase.
fn sanitize_variant_name(name: &str) -> String {
    // Check if it's already a valid PascalCase identifier (no underscores, starts with uppercase)
    if name.chars().all(|c| c.is_ascii_alphanumeric())
        && !name.is_empty()
        && name.chars().next().unwrap().is_uppercase()
    {
        return name.to_string();
    }

    // Split by underscores, spaces, hyphens, and other non-alphanumeric chars,
    // then join as PascalCase.
    name.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    let rest: String = chars.as_str().to_lowercase();
                    upper + &rest
                }
            }
        })
        .collect()
}

/// Generate a German struct with From/Into conversions.
pub fn generate_german_struct(
    info: &StructInfo,
    german_name: &str,
    resolver: &TypeResolver,
) -> TokenStream {
    let german_ident = Ident::new(german_name, Span::call_site());

    // Build the module path for the English type
    let english_type_path = match info.module_path.as_str() {
        "bo" => {
            let english_ident = Ident::new(&info.name, Span::call_site());
            quote! { bo4e_core::bo::#english_ident }
        }
        "com" => {
            let english_ident = Ident::new(&info.name, Span::call_site());
            quote! { bo4e_core::com::#english_ident }
        }
        _ => unreachable!("Structs should only be in bo or com modules"),
    };

    let mut field_defs = Vec::new();
    let mut from_english_fields = Vec::new();
    let mut from_german_fields = Vec::new();

    for field in &info.fields {
        if field.is_flatten {
            // The meta field stays as bo4e_core::Bo4eMeta
            let field_ident = Ident::new(&field.name, Span::call_site());
            field_defs.push(quote! {
                #[serde(flatten)]
                pub #field_ident: bo4e_core::Bo4eMeta
            });
            from_english_fields.push(quote! { #field_ident: v.#field_ident });
            from_german_fields.push(quote! { #field_ident: v.#field_ident });
            continue;
        }

        // Determine German field name
        let german_field_name = if let Some(ref gn) = field.german_name {
            camel_to_snake(gn)
        } else {
            // No German name available, keep English name
            field.name.clone()
        };

        let german_field_ident = format_ident!("{}", german_field_name);
        let english_field_ident = Ident::new(&field.name, Span::call_site());

        // Compute the German type
        let german_type = resolve_field_type(field, resolver);
        let german_type_tokens: TokenStream = german_type.parse().unwrap_or_else(|_| {
            panic!(
                "Failed to parse type: {} (from field {} in {})",
                german_type, field.name, info.name
            )
        });

        // Determine serde attributes for the German field
        let serde_attrs = generate_field_serde_attrs(field, &german_field_name);

        field_defs.push(quote! {
            #serde_attrs
            pub #german_field_ident: #german_type_tokens
        });

        // Generate conversion expressions
        let to_german_expr = generate_conversion_expr(&english_field_ident, field, resolver, true);
        let to_english_expr = generate_conversion_expr(&german_field_ident, field, resolver, false);

        from_english_fields.push(quote! { #german_field_ident: #to_german_expr });
        from_german_fields.push(quote! { #english_field_ident: #to_english_expr });
    }

    quote! {
        #[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #german_ident {
            #(#field_defs,)*
        }

        impl From<#english_type_path> for #german_ident {
            fn from(v: #english_type_path) -> Self {
                Self {
                    #(#from_english_fields,)*
                }
            }
        }

        impl From<#german_ident> for #english_type_path {
            fn from(v: #german_ident) -> Self {
                Self {
                    #(#from_german_fields,)*
                }
            }
        }
    }
}

/// Resolve the full type for a German field, replacing BO4E types with German equivalents.
fn resolve_field_type(field: &FieldInfo, resolver: &TypeResolver) -> String {
    // If there's no inner BO4E type, keep the original type (but qualify it)
    let inner = match &field.inner_type {
        Some(inner) => inner,
        None => return qualify_type_string(&field.type_str),
    };

    // Check if the inner type has a German mapping
    let german_inner = match resolver.german_name(inner) {
        Some(g) => g.to_string(),
        None => return qualify_type_string(&field.type_str),
    };

    // Rebuild the type with the German name
    // We need to handle Option<Box<super::Type>> -> Option<Box<GermanType>>
    // and Option<Type> -> Option<GermanType>
    // and Vec<Type> -> Vec<GermanType>
    build_german_type(field, &german_inner)
}

/// Ensure type strings use fully-qualified paths for chrono types.
fn qualify_type_string(type_str: &str) -> String {
    let mut s = type_str.to_string();
    // Replace bare DateTime<Utc> with chrono::DateTime<chrono::Utc>
    // But don't double-qualify chrono::DateTime<chrono::Utc>
    if s.contains("DateTime")
        && !s.contains("chrono :: DateTime")
        && !s.contains("chrono::DateTime")
    {
        s = s.replace("DateTime", "chrono::DateTime");
    }
    if s.contains("<Utc>") && !s.contains("chrono :: Utc") && !s.contains("chrono::Utc") {
        s = s.replace("<Utc>", "<chrono::Utc>");
    }
    // Replace bare NaiveDate with chrono::NaiveDate
    if s.contains("NaiveDate")
        && !s.contains("chrono :: NaiveDate")
        && !s.contains("chrono::NaiveDate")
    {
        s = s.replace("NaiveDate", "chrono::NaiveDate");
    }
    s
}

/// Build the German version of a field type.
fn build_german_type(field: &FieldInfo, german_inner: &str) -> String {
    let inner = format!("crate::{}", german_inner);

    if field.is_option && field.is_box {
        format!("Option<Box<{}>>", inner)
    } else if field.is_option && field.is_vec {
        format!("Option<Vec<{}>>", inner)
    } else if field.is_vec && field.is_box {
        format!("Vec<Box<{}>>", inner)
    } else if field.is_option {
        format!("Option<{}>", inner)
    } else if field.is_vec {
        format!("Vec<{}>", inner)
    } else if field.is_box {
        format!("Box<{}>", inner)
    } else {
        inner
    }
}

/// Generate serde attributes for a German field.
fn generate_field_serde_attrs(field: &FieldInfo, german_field_name: &str) -> TokenStream {
    // The English camelCase name serves as an alias
    let english_camel = snake_to_camel(&field.name);

    if field.is_vec {
        // Vec fields use default + skip_serializing_if
        // Only add alias if English camelCase differs from German field name camelCase
        let german_camel = snake_to_camel(german_field_name);
        if english_camel != german_camel {
            quote! {
                #[serde(default, skip_serializing_if = "Vec::is_empty", alias = #english_camel)]
            }
        } else {
            quote! {
                #[serde(default, skip_serializing_if = "Vec::is_empty")]
            }
        }
    } else {
        // Option fields use skip_serializing_if
        let german_camel = snake_to_camel(german_field_name);
        if english_camel != german_camel {
            quote! {
                #[serde(skip_serializing_if = "Option::is_none", alias = #english_camel)]
            }
        } else {
            quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
            }
        }
    }
}

/// Generate conversion expression for a field in From impl.
fn generate_conversion_expr(
    source_field: &Ident,
    field: &FieldInfo,
    resolver: &TypeResolver,
    _to_german: bool,
) -> TokenStream {
    let needs_conversion = field
        .inner_type
        .as_ref()
        .is_some_and(|t| resolver.is_bo4e_type(t));

    if !needs_conversion {
        // Direct copy for primitives, String, DateTime, etc.
        return quote! { v.#source_field };
    }

    // BO4E types need .into() conversion
    if field.is_option && field.is_box {
        quote! { v.#source_field.map(|b| Box::new((*b).into())) }
    } else if field.is_option {
        quote! { v.#source_field.map(Into::into) }
    } else if field.is_vec && field.is_box {
        quote! { v.#source_field.into_iter().map(|b| Box::new((*b).into())).collect() }
    } else if field.is_vec {
        quote! { v.#source_field.into_iter().map(Into::into).collect() }
    } else if field.is_box {
        quote! { Box::new((*v.#source_field).into()) }
    } else {
        quote! { v.#source_field.into() }
    }
}
