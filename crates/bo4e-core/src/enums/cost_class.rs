//! Cost class (Kostenklasse) enumeration.

use serde::{Deserialize, Serialize};

/// Cost class categorization.
///
/// Cost classes form the top level of different costs.
/// Typically, total costs of a cost class are calculated in an application.
///
/// German: Kostenklasse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kostenklasse"))]
#[non_exhaustive]
pub enum CostClass {
    /// External costs (Fremdkosten)
    #[serde(rename = "FREMDKOSTEN")]
    ExternalCosts,

    /// Procurement costs (Beschaffung)
    #[serde(rename = "BESCHAFFUNG")]
    Procurement,

    /// Internal costs (Selbstkosten)
    #[serde(rename = "SELBSTKOSTEN")]
    InternalCosts,

    /// Margins (Margen)
    #[serde(rename = "MARGEN")]
    Margins,

    /// Energy supply costs (Energieversorgungskosten)
    #[serde(rename = "ENERGIEVERSORGUNGSKOSTEN")]
    EnergySupplyCosts,
}

impl CostClass {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ExternalCosts => "Fremdkosten",
            Self::Procurement => "Beschaffung",
            Self::InternalCosts => "Selbstkosten",
            Self::Margins => "Margen",
            Self::EnergySupplyCosts => "Energieversorgungskosten",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&CostClass::ExternalCosts).unwrap(),
            r#""FREMDKOSTEN""#
        );
        assert_eq!(
            serde_json::to_string(&CostClass::Procurement).unwrap(),
            r#""BESCHAFFUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for cost_class in [
            CostClass::ExternalCosts,
            CostClass::Procurement,
            CostClass::InternalCosts,
            CostClass::Margins,
            CostClass::EnergySupplyCosts,
        ] {
            let json = serde_json::to_string(&cost_class).unwrap();
            let parsed: CostClass = serde_json::from_str(&json).unwrap();
            assert_eq!(cost_class, parsed);
        }
    }
}
