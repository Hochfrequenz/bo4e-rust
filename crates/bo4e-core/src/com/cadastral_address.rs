//! Cadastral address (Katasteradresse) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Address based on cadastral/land registry information.
///
/// Used for addressing via real estate/property information.
///
/// German: Katasteradresse
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::CadastralAddress;
///
/// let cadastral = CadastralAddress {
///     gemarkung_flur: Some("Flur 4".to_string()),
///     flurstueck: Some("123/45".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Katasteradresse"))]
#[serde(rename_all = "camelCase")]
pub struct CadastralAddress {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Cadastral district and parcel (Gemarkung/Flur)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gemarkungFlur"))]
    pub gemarkung_flur: Option<String>,

    /// Plot/parcel number (Flurstück)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "flurstueck"))]
    pub flurstueck: Option<String>,
}

impl Bo4eObject for CadastralAddress {
    fn type_name_german() -> &'static str {
        "Katasteradresse"
    }

    fn type_name_english() -> &'static str {
        "CadastralAddress"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadastral_address_default() {
        let addr = CadastralAddress::default();
        assert!(addr.gemarkung_flur.is_none());
        assert!(addr.flurstueck.is_none());
    }

    #[test]
    fn test_cadastral_address_serialize() {
        let addr = CadastralAddress {
            gemarkung_flur: Some("Gemarkung Musterstadt, Flur 3".to_string()),
            flurstueck: Some("123/4".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&addr).unwrap();
        assert!(json.contains(r#""gemarkungFlur":"#));
        assert!(json.contains(r#""flurstueck":"123/4""#));
    }

    #[test]
    fn test_cadastral_address_roundtrip() {
        let addr = CadastralAddress {
            meta: Bo4eMeta::with_type("Katasteradresse"),
            gemarkung_flur: Some("Flur 5".to_string()),
            flurstueck: Some("789/10".to_string()),
        };

        let json = serde_json::to_string(&addr).unwrap();
        let parsed: CadastralAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(addr, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(CadastralAddress::type_name_german(), "Katasteradresse");
        assert_eq!(CadastralAddress::type_name_english(), "CadastralAddress");
    }
}
