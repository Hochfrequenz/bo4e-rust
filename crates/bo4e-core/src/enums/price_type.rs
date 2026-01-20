//! Price type (Preistyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of price in tariffs.
///
/// Breakdown of price types in tariffs.
///
/// German: Preistyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Preistyp"))]
#[non_exhaustive]
pub enum PriceType {
    /// Base price (Grundpreis)
    #[serde(rename = "GRUNDPREIS")]
    BasePrice,

    /// Working price single tariff (Arbeitspreis Eintarif)
    #[serde(rename = "ARBEITSPREIS_EINTARIF")]
    WorkingPriceSingleTariff,

    /// Working price high tariff (Arbeitspreis HT)
    #[serde(rename = "ARBEITSPREIS_HT")]
    WorkingPriceHT,

    /// Working price low tariff (Arbeitspreis NT)
    #[serde(rename = "ARBEITSPREIS_NT")]
    WorkingPriceNT,

    /// Capacity price (Leistungspreis)
    #[serde(rename = "LEISTUNGSPREIS")]
    CapacityPrice,

    /// Metering price (Messpreis)
    #[serde(rename = "MESSPREIS")]
    MeteringPrice,

    /// Meter reading fee (Entgelt für Ablesung)
    #[serde(rename = "ENTGELT_ABLESUNG")]
    MeterReadingFee,

    /// Billing fee (Entgelt für Abrechnung)
    #[serde(rename = "ENTGELT_ABRECHNUNG")]
    BillingFee,

    /// Metering service operator fee (Entgelt für MSB)
    #[serde(rename = "ENTGELT_MSB")]
    MeteringServiceFee,

    /// Commission (Provision)
    #[serde(rename = "PROVISION")]
    Commission,
}

impl PriceType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::BasePrice => "Grundpreis",
            Self::WorkingPriceSingleTariff => "Arbeitspreis Eintarif",
            Self::WorkingPriceHT => "Arbeitspreis HT",
            Self::WorkingPriceNT => "Arbeitspreis NT",
            Self::CapacityPrice => "Leistungspreis",
            Self::MeteringPrice => "Messpreis",
            Self::MeterReadingFee => "Entgelt für Ablesung",
            Self::BillingFee => "Entgelt für Abrechnung",
            Self::MeteringServiceFee => "Entgelt für MSB",
            Self::Commission => "Provision",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&PriceType::BasePrice).unwrap(),
            r#""GRUNDPREIS""#
        );
        assert_eq!(
            serde_json::to_string(&PriceType::WorkingPriceSingleTariff).unwrap(),
            r#""ARBEITSPREIS_EINTARIF""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<PriceType>(r#""LEISTUNGSPREIS""#).unwrap(),
            PriceType::CapacityPrice
        );
    }

    #[test]
    fn test_roundtrip() {
        for price_type in [
            PriceType::BasePrice,
            PriceType::WorkingPriceSingleTariff,
            PriceType::WorkingPriceHT,
            PriceType::WorkingPriceNT,
            PriceType::CapacityPrice,
            PriceType::MeteringPrice,
            PriceType::MeterReadingFee,
            PriceType::BillingFee,
            PriceType::MeteringServiceFee,
            PriceType::Commission,
        ] {
            let json = serde_json::to_string(&price_type).unwrap();
            let parsed: PriceType = serde_json::from_str(&json).unwrap();
            assert_eq!(price_type, parsed);
        }
    }
}
