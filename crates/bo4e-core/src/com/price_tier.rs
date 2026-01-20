//! Price tier (Preisstaffel) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price tier based on consumption brackets.
///
/// German: Preisstaffel
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::PriceTier;
///
/// let tier = PriceTier {
///     lower_limit: Some(0.0),
///     upper_limit: Some(1000.0),
///     unit_price: Some(0.30),
///     tier_number: Some(1),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTier {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Lower consumption limit inclusive (Staffelgrenze von)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<f64>,

    /// Upper consumption limit exclusive (Staffelgrenze bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<f64>,

    /// Unit price for this tier (Einheitspreis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,

    /// Tier number/sequence (Staffelnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_number: Option<i32>,

    /// Article ID reference (Artikel-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
}

impl Bo4eObject for PriceTier {
    fn type_name_german() -> &'static str {
        "Preisstaffel"
    }

    fn type_name_english() -> &'static str {
        "PriceTier"
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
    fn test_consumption_tiers() {
        let tier1 = PriceTier {
            lower_limit: Some(0.0),
            upper_limit: Some(1000.0),
            unit_price: Some(0.30),
            tier_number: Some(1),
            ..Default::default()
        };

        let tier2 = PriceTier {
            lower_limit: Some(1000.0),
            upper_limit: Some(5000.0),
            unit_price: Some(0.25),
            tier_number: Some(2),
            ..Default::default()
        };

        assert!(tier1.unit_price.unwrap() > tier2.unit_price.unwrap());
    }

    #[test]
    fn test_default() {
        let tier = PriceTier::default();
        assert!(tier.lower_limit.is_none());
        assert!(tier.upper_limit.is_none());
        assert!(tier.unit_price.is_none());
    }

    #[test]
    fn test_serialize() {
        let tier = PriceTier {
            lower_limit: Some(0.0),
            upper_limit: Some(1000.0),
            unit_price: Some(0.30),
            tier_number: Some(1),
            ..Default::default()
        };

        let json = serde_json::to_string(&tier).unwrap();
        assert!(json.contains(r#""lowerLimit":0"#));
        assert!(json.contains(r#""upperLimit":1000"#));
        assert!(json.contains(r#""unitPrice":0.3"#));
        assert!(json.contains(r#""tierNumber":1"#));
    }

    #[test]
    fn test_roundtrip() {
        let tier = PriceTier {
            lower_limit: Some(100.0),
            upper_limit: Some(500.0),
            unit_price: Some(0.28),
            tier_number: Some(2),
            article_id: Some("BDEW-123".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&tier).unwrap();
        let parsed: PriceTier = serde_json::from_str(&json).unwrap();
        assert_eq!(tier, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(PriceTier::type_name_german(), "Preisstaffel");
        assert_eq!(PriceTier::type_name_english(), "PriceTier");
    }
}
