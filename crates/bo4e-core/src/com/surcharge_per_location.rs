//! Surcharge per location (AufAbschlagProOrt) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

use super::Surcharge;

/// A surcharge or discount that applies to a specific location.
///
/// German: AufAbschlagProOrt
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::SurchargePerLocation;
///
/// let surcharge = SurchargePerLocation {
///     postal_code: Some("50667".to_string()),
///     municipality: Some("Köln".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "AufAbschlagProOrt"))]
#[serde(rename_all = "camelCase")]
pub struct SurchargePerLocation {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Postal code (Postleitzahl)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "postleitzahl"))]
    pub postal_code: Option<String>,

    /// Municipality/city (Ort)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ort"))]
    pub municipality: Option<String>,

    /// Network area code (Netznummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netznummer"))]
    pub network_area_code: Option<String>,

    /// Surcharges applicable to this location (AufAbschläge)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "aufAbschlaege"))]
    pub surcharges: Vec<Surcharge>,
}

impl Bo4eObject for SurchargePerLocation {
    fn type_name_german() -> &'static str {
        "AufAbschlagProOrt"
    }

    fn type_name_english() -> &'static str {
        "SurchargePerLocation"
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
    fn test_location_surcharge() {
        let surcharge = SurchargePerLocation {
            postal_code: Some("50667".to_string()),
            municipality: Some("Köln".to_string()),
            surcharges: vec![Surcharge {
                description: Some("Konzessionsabgabe".to_string()),
                value: Some(1.59),
                ..Default::default()
            }],
            ..Default::default()
        };

        assert_eq!(surcharge.postal_code, Some("50667".to_string()));
        assert_eq!(surcharge.surcharges.len(), 1);
    }

    #[test]
    fn test_default() {
        let surcharge = SurchargePerLocation::default();
        assert!(surcharge.postal_code.is_none());
        assert!(surcharge.surcharges.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let surcharge = SurchargePerLocation {
            postal_code: Some("80331".to_string()),
            municipality: Some("München".to_string()),
            network_area_code: Some("9900001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&surcharge).unwrap();
        let parsed: SurchargePerLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(surcharge, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            SurchargePerLocation::type_name_german(),
            "AufAbschlagProOrt"
        );
        assert_eq!(
            SurchargePerLocation::type_name_english(),
            "SurchargePerLocation"
        );
    }
}
