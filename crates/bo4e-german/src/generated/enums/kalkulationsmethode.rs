#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Kalkulationsmethode {
    #[serde(rename = "STUFEN")]
    Steps,
    #[serde(rename = "ZONEN")]
    Zones,
    #[serde(rename = "VORZONEN_GP")]
    Vorzonengrundpreis,
    #[serde(rename = "SIGMOID")]
    Sigmoidfunktion,
    #[serde(rename = "BLINDARBEIT_GT_50_PROZENT")]
    ReactivePowerAbove50Percent,
    #[serde(rename = "BLINDARBEIT_GT_40_PROZENT")]
    ReactivePowerAbove40Percent,
    #[serde(rename = "BLINDARBEIT_MIT_FREIMENGE")]
    DefinedByCosPhiOrPercentage,
    #[serde(rename = "AP_GP_ZONEN")]
    WorkingAndBasePriceZoned,
    #[serde(rename = "LP_INSTALL_LEISTUNG")]
    CapacityChargeInstalledCapacity,
    #[serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ")]
    WorkingPriceTransportOrDistribution,
    #[serde(rename = "AP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")]
    WorkingPriceTransportOrDistributionLocalSigmoid,
    #[serde(rename = "LP_JAHRESVERBRAUCH")]
    CapacityChargeAnnualConsumption,
    #[serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ")]
    CapacityPriceTransportOrDistribution,
    #[serde(rename = "LP_TRANSPORT_ODER_VERTEILNETZ_ORTSVERTEILNETZ_SIGMOID")]
    CapacityPriceTransportOrDistributionLocalSigmoid,
    #[serde(rename = "FUNKTIONEN")]
    Functions,
    #[serde(
        rename = "VERBRAUCH_UEBER_SLP_GRENZE_FUNKTIONSBEZOGEN_WEITERE_BERECHNUNG_ALS_LGK"
    )]
    ConsumptionAboveSLPThresholdFunctionBasedLGK,
}
impl From<bo4e_core::enums::CalculationMethod> for Kalkulationsmethode {
    fn from(v: bo4e_core::enums::CalculationMethod) -> Self {
        match v {
            bo4e_core::enums::CalculationMethod::Steps => Kalkulationsmethode::Steps,
            bo4e_core::enums::CalculationMethod::Zones => Kalkulationsmethode::Zones,
            bo4e_core::enums::CalculationMethod::PreZoneBasePrice => {
                Kalkulationsmethode::Vorzonengrundpreis
            }
            bo4e_core::enums::CalculationMethod::Sigmoid => {
                Kalkulationsmethode::Sigmoidfunktion
            }
            bo4e_core::enums::CalculationMethod::ReactivePowerAbove50Percent => {
                Kalkulationsmethode::ReactivePowerAbove50Percent
            }
            bo4e_core::enums::CalculationMethod::ReactivePowerAbove40Percent => {
                Kalkulationsmethode::ReactivePowerAbove40Percent
            }
            bo4e_core::enums::CalculationMethod::ReactivePowerWithFreeAllowance => {
                Kalkulationsmethode::DefinedByCosPhiOrPercentage
            }
            bo4e_core::enums::CalculationMethod::WorkingAndBasePriceZoned => {
                Kalkulationsmethode::WorkingAndBasePriceZoned
            }
            bo4e_core::enums::CalculationMethod::CapacityChargeInstalledCapacity => {
                Kalkulationsmethode::CapacityChargeInstalledCapacity
            }
            bo4e_core::enums::CalculationMethod::WorkingPriceTransportOrDistribution => {
                Kalkulationsmethode::WorkingPriceTransportOrDistribution
            }
            bo4e_core::enums::CalculationMethod::WorkingPriceTransportOrDistributionLocalSigmoid => {
                Kalkulationsmethode::WorkingPriceTransportOrDistributionLocalSigmoid
            }
            bo4e_core::enums::CalculationMethod::CapacityChargeAnnualConsumption => {
                Kalkulationsmethode::CapacityChargeAnnualConsumption
            }
            bo4e_core::enums::CalculationMethod::CapacityPriceTransportOrDistribution => {
                Kalkulationsmethode::CapacityPriceTransportOrDistribution
            }
            bo4e_core::enums::CalculationMethod::CapacityPriceTransportOrDistributionLocalSigmoid => {
                Kalkulationsmethode::CapacityPriceTransportOrDistributionLocalSigmoid
            }
            bo4e_core::enums::CalculationMethod::Functions => {
                Kalkulationsmethode::Functions
            }
            bo4e_core::enums::CalculationMethod::ConsumptionAboveSLPThresholdFunctionBasedLGK => {
                Kalkulationsmethode::ConsumptionAboveSLPThresholdFunctionBasedLGK
            }
            _ => panic!("Unknown {} variant", stringify!(CalculationMethod)),
        }
    }
}
impl From<Kalkulationsmethode> for bo4e_core::enums::CalculationMethod {
    fn from(v: Kalkulationsmethode) -> Self {
        match v {
            Kalkulationsmethode::Steps => bo4e_core::enums::CalculationMethod::Steps,
            Kalkulationsmethode::Zones => bo4e_core::enums::CalculationMethod::Zones,
            Kalkulationsmethode::Vorzonengrundpreis => {
                bo4e_core::enums::CalculationMethod::PreZoneBasePrice
            }
            Kalkulationsmethode::Sigmoidfunktion => {
                bo4e_core::enums::CalculationMethod::Sigmoid
            }
            Kalkulationsmethode::ReactivePowerAbove50Percent => {
                bo4e_core::enums::CalculationMethod::ReactivePowerAbove50Percent
            }
            Kalkulationsmethode::ReactivePowerAbove40Percent => {
                bo4e_core::enums::CalculationMethod::ReactivePowerAbove40Percent
            }
            Kalkulationsmethode::DefinedByCosPhiOrPercentage => {
                bo4e_core::enums::CalculationMethod::ReactivePowerWithFreeAllowance
            }
            Kalkulationsmethode::WorkingAndBasePriceZoned => {
                bo4e_core::enums::CalculationMethod::WorkingAndBasePriceZoned
            }
            Kalkulationsmethode::CapacityChargeInstalledCapacity => {
                bo4e_core::enums::CalculationMethod::CapacityChargeInstalledCapacity
            }
            Kalkulationsmethode::WorkingPriceTransportOrDistribution => {
                bo4e_core::enums::CalculationMethod::WorkingPriceTransportOrDistribution
            }
            Kalkulationsmethode::WorkingPriceTransportOrDistributionLocalSigmoid => {
                bo4e_core::enums::CalculationMethod::WorkingPriceTransportOrDistributionLocalSigmoid
            }
            Kalkulationsmethode::CapacityChargeAnnualConsumption => {
                bo4e_core::enums::CalculationMethod::CapacityChargeAnnualConsumption
            }
            Kalkulationsmethode::CapacityPriceTransportOrDistribution => {
                bo4e_core::enums::CalculationMethod::CapacityPriceTransportOrDistribution
            }
            Kalkulationsmethode::CapacityPriceTransportOrDistributionLocalSigmoid => {
                bo4e_core::enums::CalculationMethod::CapacityPriceTransportOrDistributionLocalSigmoid
            }
            Kalkulationsmethode::Functions => {
                bo4e_core::enums::CalculationMethod::Functions
            }
            Kalkulationsmethode::ConsumptionAboveSLPThresholdFunctionBasedLGK => {
                bo4e_core::enums::CalculationMethod::ConsumptionAboveSLPThresholdFunctionBasedLGK
            }
            _ => panic!("Unknown {} variant", stringify!(Kalkulationsmethode)),
        }
    }
}
