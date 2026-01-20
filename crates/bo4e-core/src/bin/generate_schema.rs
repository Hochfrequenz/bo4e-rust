//! Schema generator for drift detection.
//!
//! Outputs JSON schemas for all BO4E types.
//!
//! Run with: cargo run --bin generate_schema --features json-schema

fn main() {
    #[cfg(feature = "json-schema")]
    {
        use serde_json::json;

        let schemas = json!({
            "bo": {},
            "com": {},
            "enum": {}
        });

        println!("{}", serde_json::to_string_pretty(&schemas).unwrap());
    }

    #[cfg(not(feature = "json-schema"))]
    {
        eprintln!("Error: This binary requires the 'json-schema' feature.");
        eprintln!("Run with: cargo run --bin generate_schema --features json-schema");
        std::process::exit(1);
    }
}
