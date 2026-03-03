#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Messpreistyp {
    #[serde(rename = "MESSPREIS_G2_5")]
    Measurementpriceg25,
    #[serde(rename = "MESSPREIS_G4")]
    MeasurementPriceG4,
    #[serde(rename = "MESSPREIS_G6")]
    MeasurementPriceG6,
    #[serde(rename = "MESSPREIS_G10")]
    MeasurementPriceG10,
    #[serde(rename = "MESSPREIS_G16")]
    MeasurementPriceG16,
    #[serde(rename = "MESSPREIS_G25")]
    MeasurementPriceG25,
    #[serde(rename = "MESSPREIS_G40")]
    MeasurementPriceG40,
    #[serde(rename = "ELEKTRONISCHER_AUFSATZ")]
    ElectronicAttachment,
    #[serde(rename = "SMART_METER_MESSPREIS_G2_5")]
    Smartmetermeasurementpriceg25,
    #[serde(rename = "SMART_METER_MESSPREIS_G4")]
    SmartMeterMeasurementPriceG4,
    #[serde(rename = "SMART_METER_MESSPREIS_G6")]
    SmartMeterMeasurementPriceG6,
    #[serde(rename = "SMART_METER_MESSPREIS_G10")]
    SmartMeterMeasurementPriceG10,
    #[serde(rename = "SMART_METER_MESSPREIS_G16")]
    SmartMeterMeasurementPriceG16,
    #[serde(rename = "SMART_METER_MESSPREIS_G25")]
    SmartMeterMeasurementPriceG25,
    #[serde(rename = "SMART_METER_MESSPREIS_G40")]
    SmartMeterMeasurementPriceG40,
    #[serde(rename = "VERRECHNUNGSPREIS_ET_WECHSEL")]
    SettlementPriceSingleTariffChange,
    #[serde(rename = "VERRECHNUNGSPREIS_ET_DREH")]
    SettlementPriceSingleTariffRotation,
    #[serde(rename = "VERRECHNUNGSPREIS_ZT_WECHSEL")]
    SettlementPriceDualTariffChange,
    #[serde(rename = "VERRECHNUNGSPREIS_ZT_DREH")]
    SettlementPriceDualTariffRotation,
    #[serde(rename = "VERRECHNUNGSPREIS_L_ET")]
    SettlementPriceLoadProfileSingleTariff,
    #[serde(rename = "VERRECHNUNGSPREIS_L_ZT")]
    SettlementPriceLoadProfileDualTariff,
    #[serde(rename = "VERRECHNUNGSPREIS_SM")]
    SettlementPriceSmartMeter,
    #[serde(rename = "AUFSCHLAG_WANDLER")]
    SurchargeTransformer,
    #[serde(rename = "AUFSCHLAG_TARIFSCHALTUNG")]
    SurchargeTariffSwitching,
}
impl From<bo4e_core::enums::MeasurementPriceType> for Messpreistyp {
    fn from(v: bo4e_core::enums::MeasurementPriceType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG2_5 => {
                Messpreistyp::Measurementpriceg25
            }
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG4 => {
                Messpreistyp::MeasurementPriceG4
            }
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG6 => {
                Messpreistyp::MeasurementPriceG6
            }
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG10 => {
                Messpreistyp::MeasurementPriceG10
            }
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG16 => {
                Messpreistyp::MeasurementPriceG16
            }
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG25 => {
                Messpreistyp::MeasurementPriceG25
            }
            bo4e_core::enums::MeasurementPriceType::MeasurementPriceG40 => {
                Messpreistyp::MeasurementPriceG40
            }
            bo4e_core::enums::MeasurementPriceType::ElectronicAttachment => {
                Messpreistyp::ElectronicAttachment
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG2_5 => {
                Messpreistyp::Smartmetermeasurementpriceg25
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG4 => {
                Messpreistyp::SmartMeterMeasurementPriceG4
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG6 => {
                Messpreistyp::SmartMeterMeasurementPriceG6
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG10 => {
                Messpreistyp::SmartMeterMeasurementPriceG10
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG16 => {
                Messpreistyp::SmartMeterMeasurementPriceG16
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG25 => {
                Messpreistyp::SmartMeterMeasurementPriceG25
            }
            bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG40 => {
                Messpreistyp::SmartMeterMeasurementPriceG40
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceSingleTariffChange => {
                Messpreistyp::SettlementPriceSingleTariffChange
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceSingleTariffRotation => {
                Messpreistyp::SettlementPriceSingleTariffRotation
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceDualTariffChange => {
                Messpreistyp::SettlementPriceDualTariffChange
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceDualTariffRotation => {
                Messpreistyp::SettlementPriceDualTariffRotation
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceLoadProfileSingleTariff => {
                Messpreistyp::SettlementPriceLoadProfileSingleTariff
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceLoadProfileDualTariff => {
                Messpreistyp::SettlementPriceLoadProfileDualTariff
            }
            bo4e_core::enums::MeasurementPriceType::SettlementPriceSmartMeter => {
                Messpreistyp::SettlementPriceSmartMeter
            }
            bo4e_core::enums::MeasurementPriceType::SurchargeTransformer => {
                Messpreistyp::SurchargeTransformer
            }
            bo4e_core::enums::MeasurementPriceType::SurchargeTariffSwitching => {
                Messpreistyp::SurchargeTariffSwitching
            }
            _ => panic!("Unknown {} variant", stringify!(MeasurementPriceType)),
        }
    }
}
impl From<Messpreistyp> for bo4e_core::enums::MeasurementPriceType {
    fn from(v: Messpreistyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Messpreistyp::Measurementpriceg25 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG2_5
            }
            Messpreistyp::MeasurementPriceG4 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG4
            }
            Messpreistyp::MeasurementPriceG6 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG6
            }
            Messpreistyp::MeasurementPriceG10 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG10
            }
            Messpreistyp::MeasurementPriceG16 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG16
            }
            Messpreistyp::MeasurementPriceG25 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG25
            }
            Messpreistyp::MeasurementPriceG40 => {
                bo4e_core::enums::MeasurementPriceType::MeasurementPriceG40
            }
            Messpreistyp::ElectronicAttachment => {
                bo4e_core::enums::MeasurementPriceType::ElectronicAttachment
            }
            Messpreistyp::Smartmetermeasurementpriceg25 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG2_5
            }
            Messpreistyp::SmartMeterMeasurementPriceG4 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG4
            }
            Messpreistyp::SmartMeterMeasurementPriceG6 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG6
            }
            Messpreistyp::SmartMeterMeasurementPriceG10 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG10
            }
            Messpreistyp::SmartMeterMeasurementPriceG16 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG16
            }
            Messpreistyp::SmartMeterMeasurementPriceG25 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG25
            }
            Messpreistyp::SmartMeterMeasurementPriceG40 => {
                bo4e_core::enums::MeasurementPriceType::SmartMeterMeasurementPriceG40
            }
            Messpreistyp::SettlementPriceSingleTariffChange => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceSingleTariffChange
            }
            Messpreistyp::SettlementPriceSingleTariffRotation => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceSingleTariffRotation
            }
            Messpreistyp::SettlementPriceDualTariffChange => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceDualTariffChange
            }
            Messpreistyp::SettlementPriceDualTariffRotation => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceDualTariffRotation
            }
            Messpreistyp::SettlementPriceLoadProfileSingleTariff => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceLoadProfileSingleTariff
            }
            Messpreistyp::SettlementPriceLoadProfileDualTariff => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceLoadProfileDualTariff
            }
            Messpreistyp::SettlementPriceSmartMeter => {
                bo4e_core::enums::MeasurementPriceType::SettlementPriceSmartMeter
            }
            Messpreistyp::SurchargeTransformer => {
                bo4e_core::enums::MeasurementPriceType::SurchargeTransformer
            }
            Messpreistyp::SurchargeTariffSwitching => {
                bo4e_core::enums::MeasurementPriceType::SurchargeTariffSwitching
            }
        }
    }
}
