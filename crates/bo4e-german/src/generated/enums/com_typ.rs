#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ComTyp {
    #[serde(rename = "Adresse")]
    Address,
    #[serde(rename = "Geokoordinaten")]
    GeoCoordinates,
    #[serde(rename = "Katasteradresse")]
    CadastralAddress,
    #[serde(rename = "Preis")]
    Price,
    #[serde(rename = "Preisposition")]
    PricePosition,
    #[serde(rename = "Preisstaffel")]
    PriceTier,
    #[serde(rename = "Tarifpreis")]
    TariffPrice,
    #[serde(rename = "Tarifpreisposition")]
    TariffPricePosition,
    #[serde(rename = "RegionalePreisstaffel")]
    RegionalPriceTier,
    #[serde(rename = "Betrag")]
    Amount,
    #[serde(rename = "Menge")]
    Quantity,
    #[serde(rename = "Messwert")]
    MeasuredValue,
    #[serde(rename = "Steuerbetrag")]
    TaxAmount,
    #[serde(rename = "Zeitraum")]
    TimePeriod,
    #[serde(rename = "Zaehlwerk")]
    MeterRegister,
    #[serde(rename = "Zaehlzeitregister")]
    TimeOfUseRegister,
    #[serde(rename = "Fremdkostenblock")]
    ExternalCostBlock,
    #[serde(rename = "Fremdkostenposition")]
    ExternalCostPosition,
    #[serde(rename = "Kostenblock")]
    CostBlock,
    #[serde(rename = "Kostenposition")]
    CostPosition,
    #[serde(rename = "AufAbschlag")]
    Surcharge,
    #[serde(rename = "AufAbschlagProOrt")]
    SurchargePerLocation,
    #[serde(rename = "AufAbschlagRegional")]
    RegionalSurcharge,
    #[serde(rename = "PositionsAufAbschlag")]
    PositionSurcharge,
    #[serde(rename = "Tarifberechnungsparameter")]
    TariffCalculationParameter,
    #[serde(rename = "Tarifeinschraenkung")]
    TariffRestriction,
    #[serde(rename = "Vertragskonditionen")]
    ContractConditions,
    #[serde(rename = "Vertragsteil")]
    ContractPart,
    #[serde(rename = "Energieherkunft")]
    EnergySource,
    #[serde(rename = "Energiemix")]
    EnergyMix,
    #[serde(rename = "Rechnungsposition")]
    InvoicePosition,
    #[serde(rename = "Angebotsposition")]
    OfferPosition,
    #[serde(rename = "Angebotsteil")]
    OfferPart,
    #[serde(rename = "Angebotsvariante")]
    OfferVariant,
    #[serde(rename = "Kontaktweg")]
    ContactMethod,
    #[serde(rename = "Unterschrift")]
    Signature,
    #[serde(rename = "Zustaendigkeit")]
    Responsibility,
    #[serde(rename = "Preisgarantie")]
    PriceGuarantee,
    #[serde(rename = "Regionskriterium")]
    RegionCriterion,
    #[serde(rename = "Verbrauch")]
    Consumption,
}
impl From<bo4e_core::enums::ComType> for ComTyp {
    fn from(v: bo4e_core::enums::ComType) -> Self {
        match v {
            bo4e_core::enums::ComType::Address => ComTyp::Address,
            bo4e_core::enums::ComType::GeoCoordinates => ComTyp::GeoCoordinates,
            bo4e_core::enums::ComType::CadastralAddress => ComTyp::CadastralAddress,
            bo4e_core::enums::ComType::Price => ComTyp::Price,
            bo4e_core::enums::ComType::PricePosition => ComTyp::PricePosition,
            bo4e_core::enums::ComType::PriceTier => ComTyp::PriceTier,
            bo4e_core::enums::ComType::TariffPrice => ComTyp::TariffPrice,
            bo4e_core::enums::ComType::TariffPricePosition => ComTyp::TariffPricePosition,
            bo4e_core::enums::ComType::RegionalPriceTier => ComTyp::RegionalPriceTier,
            bo4e_core::enums::ComType::Amount => ComTyp::Amount,
            bo4e_core::enums::ComType::Quantity => ComTyp::Quantity,
            bo4e_core::enums::ComType::MeasuredValue => ComTyp::MeasuredValue,
            bo4e_core::enums::ComType::TaxAmount => ComTyp::TaxAmount,
            bo4e_core::enums::ComType::TimePeriod => ComTyp::TimePeriod,
            bo4e_core::enums::ComType::MeterRegister => ComTyp::MeterRegister,
            bo4e_core::enums::ComType::TimeOfUseRegister => ComTyp::TimeOfUseRegister,
            bo4e_core::enums::ComType::ExternalCostBlock => ComTyp::ExternalCostBlock,
            bo4e_core::enums::ComType::ExternalCostPosition => {
                ComTyp::ExternalCostPosition
            }
            bo4e_core::enums::ComType::CostBlock => ComTyp::CostBlock,
            bo4e_core::enums::ComType::CostPosition => ComTyp::CostPosition,
            bo4e_core::enums::ComType::Surcharge => ComTyp::Surcharge,
            bo4e_core::enums::ComType::SurchargePerLocation => {
                ComTyp::SurchargePerLocation
            }
            bo4e_core::enums::ComType::RegionalSurcharge => ComTyp::RegionalSurcharge,
            bo4e_core::enums::ComType::PositionSurcharge => ComTyp::PositionSurcharge,
            bo4e_core::enums::ComType::TariffCalculationParameter => {
                ComTyp::TariffCalculationParameter
            }
            bo4e_core::enums::ComType::TariffRestriction => ComTyp::TariffRestriction,
            bo4e_core::enums::ComType::ContractConditions => ComTyp::ContractConditions,
            bo4e_core::enums::ComType::ContractPart => ComTyp::ContractPart,
            bo4e_core::enums::ComType::EnergySource => ComTyp::EnergySource,
            bo4e_core::enums::ComType::EnergyMix => ComTyp::EnergyMix,
            bo4e_core::enums::ComType::InvoicePosition => ComTyp::InvoicePosition,
            bo4e_core::enums::ComType::OfferPosition => ComTyp::OfferPosition,
            bo4e_core::enums::ComType::OfferPart => ComTyp::OfferPart,
            bo4e_core::enums::ComType::OfferVariant => ComTyp::OfferVariant,
            bo4e_core::enums::ComType::ContactMethod => ComTyp::ContactMethod,
            bo4e_core::enums::ComType::Signature => ComTyp::Signature,
            bo4e_core::enums::ComType::Responsibility => ComTyp::Responsibility,
            bo4e_core::enums::ComType::PriceGuarantee => ComTyp::PriceGuarantee,
            bo4e_core::enums::ComType::RegionCriterion => ComTyp::RegionCriterion,
            bo4e_core::enums::ComType::Consumption => ComTyp::Consumption,
            _ => panic!("Unknown {} variant", stringify!(ComType)),
        }
    }
}
impl From<ComTyp> for bo4e_core::enums::ComType {
    fn from(v: ComTyp) -> Self {
        match v {
            ComTyp::Address => bo4e_core::enums::ComType::Address,
            ComTyp::GeoCoordinates => bo4e_core::enums::ComType::GeoCoordinates,
            ComTyp::CadastralAddress => bo4e_core::enums::ComType::CadastralAddress,
            ComTyp::Price => bo4e_core::enums::ComType::Price,
            ComTyp::PricePosition => bo4e_core::enums::ComType::PricePosition,
            ComTyp::PriceTier => bo4e_core::enums::ComType::PriceTier,
            ComTyp::TariffPrice => bo4e_core::enums::ComType::TariffPrice,
            ComTyp::TariffPricePosition => bo4e_core::enums::ComType::TariffPricePosition,
            ComTyp::RegionalPriceTier => bo4e_core::enums::ComType::RegionalPriceTier,
            ComTyp::Amount => bo4e_core::enums::ComType::Amount,
            ComTyp::Quantity => bo4e_core::enums::ComType::Quantity,
            ComTyp::MeasuredValue => bo4e_core::enums::ComType::MeasuredValue,
            ComTyp::TaxAmount => bo4e_core::enums::ComType::TaxAmount,
            ComTyp::TimePeriod => bo4e_core::enums::ComType::TimePeriod,
            ComTyp::MeterRegister => bo4e_core::enums::ComType::MeterRegister,
            ComTyp::TimeOfUseRegister => bo4e_core::enums::ComType::TimeOfUseRegister,
            ComTyp::ExternalCostBlock => bo4e_core::enums::ComType::ExternalCostBlock,
            ComTyp::ExternalCostPosition => {
                bo4e_core::enums::ComType::ExternalCostPosition
            }
            ComTyp::CostBlock => bo4e_core::enums::ComType::CostBlock,
            ComTyp::CostPosition => bo4e_core::enums::ComType::CostPosition,
            ComTyp::Surcharge => bo4e_core::enums::ComType::Surcharge,
            ComTyp::SurchargePerLocation => {
                bo4e_core::enums::ComType::SurchargePerLocation
            }
            ComTyp::RegionalSurcharge => bo4e_core::enums::ComType::RegionalSurcharge,
            ComTyp::PositionSurcharge => bo4e_core::enums::ComType::PositionSurcharge,
            ComTyp::TariffCalculationParameter => {
                bo4e_core::enums::ComType::TariffCalculationParameter
            }
            ComTyp::TariffRestriction => bo4e_core::enums::ComType::TariffRestriction,
            ComTyp::ContractConditions => bo4e_core::enums::ComType::ContractConditions,
            ComTyp::ContractPart => bo4e_core::enums::ComType::ContractPart,
            ComTyp::EnergySource => bo4e_core::enums::ComType::EnergySource,
            ComTyp::EnergyMix => bo4e_core::enums::ComType::EnergyMix,
            ComTyp::InvoicePosition => bo4e_core::enums::ComType::InvoicePosition,
            ComTyp::OfferPosition => bo4e_core::enums::ComType::OfferPosition,
            ComTyp::OfferPart => bo4e_core::enums::ComType::OfferPart,
            ComTyp::OfferVariant => bo4e_core::enums::ComType::OfferVariant,
            ComTyp::ContactMethod => bo4e_core::enums::ComType::ContactMethod,
            ComTyp::Signature => bo4e_core::enums::ComType::Signature,
            ComTyp::Responsibility => bo4e_core::enums::ComType::Responsibility,
            ComTyp::PriceGuarantee => bo4e_core::enums::ComType::PriceGuarantee,
            ComTyp::RegionCriterion => bo4e_core::enums::ComType::RegionCriterion,
            ComTyp::Consumption => bo4e_core::enums::ComType::Consumption,
            _ => panic!("Unknown {} variant", stringify!(ComTyp)),
        }
    }
}
