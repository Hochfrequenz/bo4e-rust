//! Measurement price type (Messpreistyp) enumeration.

use serde::{Deserialize, Serialize};

/// Measurement pricing type.
///
/// Specifies which type of measurement is priced.
///
/// German: Messpreistyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MeasurementPriceType {
    /// Measurement price G2.5
    #[serde(rename = "MESSPREIS_G2_5")]
    MeasurementPriceG2_5,

    /// Measurement price G4
    #[serde(rename = "MESSPREIS_G4")]
    MeasurementPriceG4,

    /// Measurement price G6
    #[serde(rename = "MESSPREIS_G6")]
    MeasurementPriceG6,

    /// Measurement price G10
    #[serde(rename = "MESSPREIS_G10")]
    MeasurementPriceG10,

    /// Measurement price G16
    #[serde(rename = "MESSPREIS_G16")]
    MeasurementPriceG16,

    /// Measurement price G25
    #[serde(rename = "MESSPREIS_G25")]
    MeasurementPriceG25,

    /// Measurement price G40
    #[serde(rename = "MESSPREIS_G40")]
    MeasurementPriceG40,

    /// Electronic attachment
    #[serde(rename = "ELEKTRONISCHER_AUFSATZ")]
    ElectronicAttachment,

    /// Smart meter measurement price G2.5
    #[serde(rename = "SMART_METER_MESSPREIS_G2_5")]
    SmartMeterMeasurementPriceG2_5,

    /// Smart meter measurement price G4
    #[serde(rename = "SMART_METER_MESSPREIS_G4")]
    SmartMeterMeasurementPriceG4,

    /// Smart meter measurement price G6
    #[serde(rename = "SMART_METER_MESSPREIS_G6")]
    SmartMeterMeasurementPriceG6,

    /// Smart meter measurement price G10
    #[serde(rename = "SMART_METER_MESSPREIS_G10")]
    SmartMeterMeasurementPriceG10,

    /// Smart meter measurement price G16
    #[serde(rename = "SMART_METER_MESSPREIS_G16")]
    SmartMeterMeasurementPriceG16,

    /// Smart meter measurement price G25
    #[serde(rename = "SMART_METER_MESSPREIS_G25")]
    SmartMeterMeasurementPriceG25,

    /// Smart meter measurement price G40
    #[serde(rename = "SMART_METER_MESSPREIS_G40")]
    SmartMeterMeasurementPriceG40,

    /// Settlement price single tariff change
    #[serde(rename = "VERRECHNUNGSPREIS_ET_WECHSEL")]
    SettlementPriceSingleTariffChange,

    /// Settlement price single tariff rotation
    #[serde(rename = "VERRECHNUNGSPREIS_ET_DREH")]
    SettlementPriceSingleTariffRotation,

    /// Settlement price dual tariff change
    #[serde(rename = "VERRECHNUNGSPREIS_ZT_WECHSEL")]
    SettlementPriceDualTariffChange,

    /// Settlement price dual tariff rotation
    #[serde(rename = "VERRECHNUNGSPREIS_ZT_DREH")]
    SettlementPriceDualTariffRotation,

    /// Settlement price load profile single tariff
    #[serde(rename = "VERRECHNUNGSPREIS_L_ET")]
    SettlementPriceLoadProfileSingleTariff,

    /// Settlement price load profile dual tariff
    #[serde(rename = "VERRECHNUNGSPREIS_L_ZT")]
    SettlementPriceLoadProfileDualTariff,

    /// Settlement price smart meter
    #[serde(rename = "VERRECHNUNGSPREIS_SM")]
    SettlementPriceSmartMeter,

    /// Surcharge for transformer
    #[serde(rename = "AUFSCHLAG_WANDLER")]
    SurchargeTransformer,

    /// Surcharge for tariff switching
    #[serde(rename = "AUFSCHLAG_TARIFSCHALTUNG")]
    SurchargeTariffSwitching,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeasurementPriceType::MeasurementPriceG4).unwrap(),
            r#""MESSPREIS_G4""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for ptype in [
            MeasurementPriceType::MeasurementPriceG2_5,
            MeasurementPriceType::MeasurementPriceG4,
            MeasurementPriceType::ElectronicAttachment,
            MeasurementPriceType::SmartMeterMeasurementPriceG4,
            MeasurementPriceType::SettlementPriceSmartMeter,
        ] {
            let json = serde_json::to_string(&ptype).unwrap();
            let parsed: MeasurementPriceType = serde_json::from_str(&json).unwrap();
            assert_eq!(ptype, parsed);
        }
    }
}
