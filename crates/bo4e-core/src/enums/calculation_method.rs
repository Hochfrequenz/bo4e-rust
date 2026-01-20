//! Calculation method (Kalkulationsmethode) enumeration.

use serde::{Deserialize, Serialize};

/// Calculation method for price sheets.
///
/// List of different calculation methods for a price sheet.
///
/// German: Kalkulationsmethode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kalkulationsmethode"))]
#[non_exhaustive]
pub enum CalculationMethod {
    /// Step model - total quantity is assigned to one step and the price applies to entire quantity
    #[serde(rename = "STUFEN")]
    Steps,

    /// Zone model - total quantity is distributed across zones with respective prices
    #[serde(rename = "ZONEN")]
    Zones,

    /// Pre-zone base price (Vorzonengrundpreis)
    #[serde(rename = "VORZONEN_GP")]
    PreZoneBasePrice,

    /// Sigmoid function (Sigmoidfunktion)
    #[serde(rename = "SIGMOID")]
    Sigmoid,

    /// Reactive power above 50% of active power
    #[serde(rename = "BLINDARBEIT_GT_50_PROZENT")]
    ReactivePowerAbove50Percent,

    /// Reactive power above 40% of active power
    #[serde(rename = "BLINDARBEIT_GT_40_PROZENT")]
    ReactivePowerAbove40Percent,

    /// Reactive power with free allowance (defined by cos phi or percentage)
    #[serde(rename = "BLINDARBEIT_MIT_FREIMENGE")]
    ReactivePowerWithFreeAllowance,

    /// Working and base price zoned
    #[serde(rename = "AP_GP_ZONEN")]
    WorkingAndBasePriceZoned,

    /// Capacity charge based on installed capacity
    #[serde(rename = "LP_INSTALL_LEISTUNG")]
    CapacityChargeInstalledCapacity,

    /// Working price based on transport or distribution network
    #[serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ")]
    WorkingPriceTransportOrDistribution,

    /// Working price based on transport/distribution network, local network via sigmoid
    #[serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")]
    WorkingPriceTransportOrDistributionLocalSigmoid,

    /// Capacity charge based on annual consumption
    #[serde(rename = "LP_JAHRESVERBRAUCH")]
    CapacityChargeAnnualConsumption,

    /// Capacity price based on transport or distribution network
    #[serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ")]
    CapacityPriceTransportOrDistribution,

    /// Capacity price based on transport/distribution network, local network via sigmoid
    #[serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")]
    CapacityPriceTransportOrDistributionLocalSigmoid,

    /// Function-based capacity determination for consumption above SLP threshold
    #[serde(rename = "FUNKTIONEN")]
    Functions,

    /// Above SLP threshold, function-based calculation as LGK
    #[serde(rename = "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK")]
    ConsumptionAboveSLPThresholdFunctionBasedLGK,
}

impl CalculationMethod {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Steps => "Stufen",
            Self::Zones => "Zonen",
            Self::PreZoneBasePrice => "Vorzonengrundpreis",
            Self::Sigmoid => "Sigmoid",
            Self::ReactivePowerAbove50Percent => "Blindarbeit oberhalb 50% der Wirkarbeit",
            Self::ReactivePowerAbove40Percent => "Blindarbeit oberhalb 40% der Wirkarbeit",
            Self::ReactivePowerWithFreeAllowance => "Blindarbeit mit Freimenge",
            Self::WorkingAndBasePriceZoned => "Arbeits- und Grundpreis gezont",
            Self::CapacityChargeInstalledCapacity => {
                "Leistungsentgelt auf Grundlage der installierten Leistung"
            }
            Self::WorkingPriceTransportOrDistribution => {
                "AP auf Grundlage Transport- oder Verteilnetz"
            }
            Self::WorkingPriceTransportOrDistributionLocalSigmoid => {
                "AP auf Grundlage Transport- oder Verteilnetz, Ortsverteilnetz über Sigmoid"
            }
            Self::CapacityChargeAnnualConsumption => {
                "Leistungsentgelt auf Grundlage des Jahresverbrauchs"
            }
            Self::CapacityPriceTransportOrDistribution => {
                "LP auf Grundlage Transport- oder Verteilnetz"
            }
            Self::CapacityPriceTransportOrDistributionLocalSigmoid => {
                "LP auf Grundlage Transport- oder Verteilnetz, Ortsverteilnetz über Sigmoid"
            }
            Self::Functions => "Funktionen",
            Self::ConsumptionAboveSLPThresholdFunctionBasedLGK => {
                "Verbrauch über SLP-Grenze funktionsbezogen als LGK"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&CalculationMethod::Steps).unwrap(),
            r#""STUFEN""#
        );
        assert_eq!(
            serde_json::to_string(&CalculationMethod::Zones).unwrap(),
            r#""ZONEN""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<CalculationMethod>(r#""SIGMOID""#).unwrap(),
            CalculationMethod::Sigmoid
        );
    }

    #[test]
    fn test_roundtrip() {
        for method in [
            CalculationMethod::Steps,
            CalculationMethod::Zones,
            CalculationMethod::PreZoneBasePrice,
            CalculationMethod::Sigmoid,
            CalculationMethod::ReactivePowerAbove50Percent,
            CalculationMethod::ReactivePowerAbove40Percent,
            CalculationMethod::ReactivePowerWithFreeAllowance,
            CalculationMethod::WorkingAndBasePriceZoned,
            CalculationMethod::CapacityChargeInstalledCapacity,
            CalculationMethod::WorkingPriceTransportOrDistribution,
            CalculationMethod::WorkingPriceTransportOrDistributionLocalSigmoid,
            CalculationMethod::CapacityChargeAnnualConsumption,
            CalculationMethod::CapacityPriceTransportOrDistribution,
            CalculationMethod::CapacityPriceTransportOrDistributionLocalSigmoid,
            CalculationMethod::Functions,
            CalculationMethod::ConsumptionAboveSLPThresholdFunctionBasedLGK,
        ] {
            let json = serde_json::to_string(&method).unwrap();
            let parsed: CalculationMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(method, parsed);
        }
    }
}
