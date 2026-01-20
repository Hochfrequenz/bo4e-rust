//! SIMD-accelerated JSON parsing.

use serde::de::DeserializeOwned;

/// Deserialize from a mutable byte slice using simd-json.
///
/// simd-json performs in-place parsing, which requires a mutable slice.
/// The input will be modified during parsing.
///
/// # Example
///
/// ```rust,ignore
/// use bo4e_serde::simd::from_slice;
/// use bo4e_core::bo::Meter;
///
/// let mut json = br#"{"meterNumber":"123"}"#.to_vec();
/// let meter: Meter = from_slice(&mut json)?;
/// ```
pub fn from_slice<T: DeserializeOwned>(json: &mut [u8]) -> Result<T, simd_json::Error> {
    simd_json::from_slice(json)
}

/// Deserialize from a string, converting to mutable bytes internally.
///
/// This is slightly less efficient than `from_slice` as it requires
/// copying the string to a mutable buffer.
pub fn from_str<T: DeserializeOwned>(json: &str) -> Result<T, simd_json::Error> {
    let mut bytes = json.as_bytes().to_vec();
    from_slice(&mut bytes)
}

/// Deserialize from owned bytes (Vec<u8>).
///
/// Takes ownership of the vector, avoiding copies.
pub fn from_vec<T: DeserializeOwned>(mut json: Vec<u8>) -> Result<T, simd_json::Error> {
    from_slice(&mut json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bo4e_core::bo::Meter;

    #[test]
    fn test_from_slice() {
        let mut json = br#"{"meterNumber":"TEST123"}"#.to_vec();
        let meter: Meter = from_slice(&mut json).unwrap();
        assert_eq!(meter.meter_number, Some("TEST123".to_string()));
    }

    #[test]
    fn test_from_str() {
        let json = r#"{"meterNumber":"TEST456"}"#;
        let meter: Meter = from_str(json).unwrap();
        assert_eq!(meter.meter_number, Some("TEST456".to_string()));
    }

    #[test]
    fn test_from_vec() {
        let json = br#"{"meterNumber":"TEST789"}"#.to_vec();
        let meter: Meter = from_vec(json).unwrap();
        assert_eq!(meter.meter_number, Some("TEST789".to_string()));
    }
}
