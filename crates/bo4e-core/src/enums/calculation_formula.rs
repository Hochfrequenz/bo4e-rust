//! Calculation formula (Berechnungsformel) enumeration.

use serde::{Deserialize, Serialize};

/// Calculation formula type.
///
/// Defines standard calculation formulas used in energy billing.
///
/// German: Berechnungsformel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Berechnungsformel"))]
#[non_exhaustive]
pub enum CalculationFormula {
    /// Highest of maximum values (Höchstwert der Maximalwerte)
    #[serde(rename = "HOECHSTWERT")]
    HighestValue,

    /// Minimum value (Minimalwert)
    #[serde(rename = "MINIMALWERT")]
    MinimumValue,

    /// Average value (Mittelwert)
    #[serde(rename = "MITTELWERT")]
    AverageValue,

    /// Sum (Summenwert)
    #[serde(rename = "SUMMENWERT")]
    SumValue,
}

impl CalculationFormula {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::HighestValue => "Höchstwert",
            Self::MinimumValue => "Minimalwert",
            Self::AverageValue => "Mittelwert",
            Self::SumValue => "Summenwert",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&CalculationFormula::HighestValue).unwrap(),
            r#""HOECHSTWERT""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for formula in [
            CalculationFormula::HighestValue,
            CalculationFormula::MinimumValue,
            CalculationFormula::AverageValue,
            CalculationFormula::SumValue,
        ] {
            let json = serde_json::to_string(&formula).unwrap();
            let parsed: CalculationFormula = serde_json::from_str(&json).unwrap();
            assert_eq!(formula, parsed);
        }
    }
}
