//! JSON serialization functions.

use serde::Serialize;

/// Serialize to a compact JSON string.
pub fn to_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serialize to a pretty-printed JSON string.
pub fn to_string_pretty<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

/// Serialize to a byte vector.
pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bo4e_core::bo::Meter;

    #[test]
    fn test_to_string() {
        let meter = Meter {
            meter_number: Some("TEST".to_string()),
            ..Default::default()
        };

        let json = to_string(&meter).unwrap();
        assert!(json.contains(r#""meterNumber":"TEST""#));
    }

    #[test]
    fn test_to_string_pretty() {
        let meter = Meter {
            meter_number: Some("TEST".to_string()),
            ..Default::default()
        };
        let json = to_string_pretty(&meter).unwrap();
        // Pretty-printed JSON with fields will have newlines
        assert!(json.contains('\n'));
        assert!(json.contains(r#""meterNumber""#));
    }
}
