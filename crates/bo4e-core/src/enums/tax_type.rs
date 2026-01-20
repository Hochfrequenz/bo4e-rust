//! Tax type (Steuerart) enumeration.

use serde::{Deserialize, Serialize};

/// Type of tax.
///
/// Used to identify different types of taxes.
///
/// German: Steuerart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Steuerart"))]
#[non_exhaustive]
pub enum TaxType {
    /// Reverse charge procedure (Umkehrung der Steuerpflicht)
    #[serde(rename = "RCV")]
    ReverseCharge,

    /// Value added tax / sales tax (Umsatzsteuer)
    #[serde(rename = "UST")]
    ValueAddedTax,

    /// Input tax / deductible VAT (Vorsteuer)
    #[serde(rename = "VST")]
    InputTax,
}

impl TaxType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ReverseCharge => "Reverse Charge Verfahren",
            Self::ValueAddedTax => "Umsatzsteuer",
            Self::InputTax => "Vorsteuer",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&TaxType::ReverseCharge).unwrap(),
            r#""RCV""#
        );
        assert_eq!(
            serde_json::to_string(&TaxType::ValueAddedTax).unwrap(),
            r#""UST""#
        );
        assert_eq!(
            serde_json::to_string(&TaxType::InputTax).unwrap(),
            r#""VST""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for tax_type in [
            TaxType::ReverseCharge,
            TaxType::ValueAddedTax,
            TaxType::InputTax,
        ] {
            let json = serde_json::to_string(&tax_type).unwrap();
            let parsed: TaxType = serde_json::from_str(&json).unwrap();
            assert_eq!(tax_type, parsed);
        }
    }
}
