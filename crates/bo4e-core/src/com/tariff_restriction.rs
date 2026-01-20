//! Tariff restriction (Tarifeinschraenkung) component.

use serde::{Deserialize, Serialize};

use crate::enums::{CustomerType, Division, TariffFeature};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Restrictions that apply to a tariff.
///
/// German: Tarifeinschraenkung
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TariffRestriction;
/// use bo4e_core::enums::CustomerType;
///
/// let restriction = TariffRestriction {
///     customer_types: vec![CustomerType::Private],
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Tarifeinschraenkung"))]
#[serde(rename_all = "camelCase")]
pub struct TariffRestriction {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Applicable customer types (Kundentypen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kundentypen"))]
    pub customer_types: Vec<CustomerType>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Required tariff features (Tarifmerkmale)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "tarifmerkmale"))]
    pub required_features: Vec<TariffFeature>,

    /// Excluded tariff features (Ausgeschlossene Tarifmerkmale)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "ausgeschlosseneTarifmerkmale"))]
    pub excluded_features: Vec<TariffFeature>,

    /// Minimum annual consumption (Mindestjahresverbrauch)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "mindestjahresverbrauch"))]
    pub min_annual_consumption: Option<f64>,

    /// Maximum annual consumption (Höchstjahresverbrauch)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "hoechstjahresverbrauch"))]
    pub max_annual_consumption: Option<f64>,

    /// Additional notes (Bemerkung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bemerkung"))]
    pub notes: Option<String>,
}

impl Bo4eObject for TariffRestriction {
    fn type_name_german() -> &'static str {
        "Tarifeinschraenkung"
    }

    fn type_name_english() -> &'static str {
        "TariffRestriction"
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
    fn test_private_customer_tariff() {
        let restriction = TariffRestriction {
            customer_types: vec![CustomerType::Private],
            division: Some(Division::Electricity),
            max_annual_consumption: Some(10000.0),
            ..Default::default()
        };

        assert_eq!(restriction.customer_types, vec![CustomerType::Private]);
    }

    #[test]
    fn test_business_tariff_with_features() {
        let restriction = TariffRestriction {
            customer_types: vec![CustomerType::Commercial],
            required_features: vec![TariffFeature::Online],
            min_annual_consumption: Some(10000.0),
            ..Default::default()
        };

        assert_eq!(restriction.required_features.len(), 1);
    }

    #[test]
    fn test_default() {
        let restriction = TariffRestriction::default();
        assert!(restriction.customer_types.is_empty());
        assert!(restriction.division.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let restriction = TariffRestriction {
            customer_types: vec![CustomerType::Private, CustomerType::Commercial],
            division: Some(Division::Gas),
            notes: Some("Available in selected areas only".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&restriction).unwrap();
        let parsed: TariffRestriction = serde_json::from_str(&json).unwrap();
        assert_eq!(restriction, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TariffRestriction::type_name_german(), "Tarifeinschraenkung");
        assert_eq!(TariffRestriction::type_name_english(), "TariffRestriction");
    }
}
