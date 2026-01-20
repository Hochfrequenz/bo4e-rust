//! Price guarantee (Preisgarantie) component.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::PriceGuaranteeType;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price guarantee specifying which price components are fixed.
///
/// German: Preisgarantie
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::PriceGuarantee;
/// use bo4e_core::enums::PriceGuaranteeType;
///
/// let guarantee = PriceGuarantee {
///     guarantee_type: Some(PriceGuaranteeType::AllComponentsGross),
///     description: Some("12-month price guarantee".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Preisgarantie"))]
#[serde(rename_all = "camelCase")]
pub struct PriceGuarantee {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of price guarantee (Preisgarantietyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "preisgarantietyp"))]
    pub guarantee_type: Option<PriceGuaranteeType>,

    /// Start of validity period (Zeitliche Gültigkeit - Von)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitlicheGueltigkeit"))]
    pub valid_from: Option<DateTime<Utc>>,

    /// End of validity period (Zeitliche Gültigkeit - Bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zeitlicheGueltigkeitBis"))]
    pub valid_until: Option<DateTime<Utc>>,

    /// Description of the guarantee (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,
}

impl Bo4eObject for PriceGuarantee {
    fn type_name_german() -> &'static str {
        "Preisgarantie"
    }

    fn type_name_english() -> &'static str {
        "PriceGuarantee"
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
    fn test_complete_price_guarantee() {
        let guarantee = PriceGuarantee {
            guarantee_type: Some(PriceGuaranteeType::AllComponentsGross),
            description: Some("Full price guarantee for 12 months".to_string()),
            ..Default::default()
        };

        assert_eq!(
            guarantee.guarantee_type,
            Some(PriceGuaranteeType::AllComponentsGross)
        );
    }

    #[test]
    fn test_energy_price_guarantee() {
        let guarantee = PriceGuarantee {
            guarantee_type: Some(PriceGuaranteeType::EnergyPriceOnly),
            description: Some("Energy price guaranteed".to_string()),
            ..Default::default()
        };

        assert_eq!(
            guarantee.guarantee_type,
            Some(PriceGuaranteeType::EnergyPriceOnly)
        );
    }

    #[test]
    fn test_default() {
        let guarantee = PriceGuarantee::default();
        assert!(guarantee.guarantee_type.is_none());
        assert!(guarantee.valid_from.is_none());
        assert!(guarantee.valid_until.is_none());
        assert!(guarantee.description.is_none());
    }

    #[test]
    fn test_serialize() {
        let guarantee = PriceGuarantee {
            guarantee_type: Some(PriceGuaranteeType::AllComponentsGross),
            description: Some("Test guarantee".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&guarantee).unwrap();
        assert!(json.contains(r#""guaranteeType":"ALLE_PREISBESTANDTEILE_BRUTTO""#));
        assert!(json.contains(r#""description":"Test guarantee""#));
    }

    #[test]
    fn test_roundtrip() {
        let guarantee = PriceGuarantee {
            guarantee_type: Some(PriceGuaranteeType::EnergyPriceOnly),
            description: Some("Energy price fixed".to_string()),
            valid_from: Some(
                DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                    .unwrap()
                    .into(),
            ),
            valid_until: Some(
                DateTime::parse_from_rfc3339("2024-12-31T23:59:59Z")
                    .unwrap()
                    .into(),
            ),
            ..Default::default()
        };

        let json = serde_json::to_string(&guarantee).unwrap();
        let parsed: PriceGuarantee = serde_json::from_str(&json).unwrap();
        assert_eq!(guarantee, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(PriceGuarantee::type_name_german(), "Preisgarantie");
        assert_eq!(PriceGuarantee::type_name_english(), "PriceGuarantee");
    }
}
