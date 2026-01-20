//! Component type discriminator.

use serde::{Deserialize, Serialize};

/// Type discriminator for Components.
///
/// Used in the `_typ` field to identify the concrete type of a BO4E component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ComType {
    // Address related
    #[serde(rename = "Adresse")]
    Address,
    #[serde(rename = "Geokoordinaten")]
    GeoCoordinates,
    #[serde(rename = "Katasteradresse")]
    CadastralAddress,

    // Pricing
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

    // Quantities
    #[serde(rename = "Betrag")]
    Amount,
    #[serde(rename = "Menge")]
    Quantity,
    #[serde(rename = "Messwert")]
    MeasuredValue,
    #[serde(rename = "Steuerbetrag")]
    TaxAmount,

    // Time related
    #[serde(rename = "Zeitraum")]
    TimePeriod,
    #[serde(rename = "Zaehlwerk")]
    MeterRegister,
    #[serde(rename = "Zaehlzeitregister")]
    TimeOfUseRegister,

    // Costs
    #[serde(rename = "Fremdkostenblock")]
    ExternalCostBlock,
    #[serde(rename = "Fremdkostenposition")]
    ExternalCostPosition,
    #[serde(rename = "Kostenblock")]
    CostBlock,
    #[serde(rename = "Kostenposition")]
    CostPosition,

    // Tariff
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

    // Contract
    #[serde(rename = "Vertragskonditionen")]
    ContractConditions,
    #[serde(rename = "Vertragsteil")]
    ContractPart,

    // Energy
    #[serde(rename = "Energieherkunft")]
    EnergySource,
    #[serde(rename = "Energiemix")]
    EnergyMix,

    // Invoice
    #[serde(rename = "Rechnungsposition")]
    InvoicePosition,

    // Offer
    #[serde(rename = "Angebotsposition")]
    OfferPosition,
    #[serde(rename = "Angebotsteil")]
    OfferPart,
    #[serde(rename = "Angebotsvariante")]
    OfferVariant,

    // Contact
    #[serde(rename = "Kontaktweg")]
    ContactMethod,
    #[serde(rename = "Unterschrift")]
    Signature,
    #[serde(rename = "Zustaendigkeit")]
    Responsibility,

    // Other
    #[serde(rename = "Preisgarantie")]
    PriceGuarantee,
    #[serde(rename = "Regionskriterium")]
    RegionCriterion,
    #[serde(rename = "Verbrauch")]
    Consumption,
}

impl ComType {
    /// Returns the German type name.
    pub fn german_name(&self) -> &'static str {
        // The serde rename value is already the German name
        match self {
            Self::Address => "Adresse",
            Self::GeoCoordinates => "Geokoordinaten",
            Self::CadastralAddress => "Katasteradresse",
            Self::Price => "Preis",
            Self::PricePosition => "Preisposition",
            Self::PriceTier => "Preisstaffel",
            Self::TariffPrice => "Tarifpreis",
            Self::TariffPricePosition => "Tarifpreisposition",
            Self::RegionalPriceTier => "RegionalePreisstaffel",
            Self::Amount => "Betrag",
            Self::Quantity => "Menge",
            Self::MeasuredValue => "Messwert",
            Self::TaxAmount => "Steuerbetrag",
            Self::TimePeriod => "Zeitraum",
            Self::MeterRegister => "Zaehlwerk",
            Self::TimeOfUseRegister => "Zaehlzeitregister",
            Self::ExternalCostBlock => "Fremdkostenblock",
            Self::ExternalCostPosition => "Fremdkostenposition",
            Self::CostBlock => "Kostenblock",
            Self::CostPosition => "Kostenposition",
            Self::Surcharge => "AufAbschlag",
            Self::SurchargePerLocation => "AufAbschlagProOrt",
            Self::RegionalSurcharge => "AufAbschlagRegional",
            Self::PositionSurcharge => "PositionsAufAbschlag",
            Self::TariffCalculationParameter => "Tarifberechnungsparameter",
            Self::TariffRestriction => "Tarifeinschraenkung",
            Self::ContractConditions => "Vertragskonditionen",
            Self::ContractPart => "Vertragsteil",
            Self::EnergySource => "Energieherkunft",
            Self::EnergyMix => "Energiemix",
            Self::InvoicePosition => "Rechnungsposition",
            Self::OfferPosition => "Angebotsposition",
            Self::OfferPart => "Angebotsteil",
            Self::OfferVariant => "Angebotsvariante",
            Self::ContactMethod => "Kontaktweg",
            Self::Signature => "Unterschrift",
            Self::Responsibility => "Zustaendigkeit",
            Self::PriceGuarantee => "Preisgarantie",
            Self::RegionCriterion => "Regionskriterium",
            Self::Consumption => "Verbrauch",
        }
    }

    /// Returns the English type name.
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Address => "Address",
            Self::GeoCoordinates => "GeoCoordinates",
            Self::CadastralAddress => "CadastralAddress",
            Self::Price => "Price",
            Self::PricePosition => "PricePosition",
            Self::PriceTier => "PriceTier",
            Self::TariffPrice => "TariffPrice",
            Self::TariffPricePosition => "TariffPricePosition",
            Self::RegionalPriceTier => "RegionalPriceTier",
            Self::Amount => "Amount",
            Self::Quantity => "Quantity",
            Self::MeasuredValue => "MeasuredValue",
            Self::TaxAmount => "TaxAmount",
            Self::TimePeriod => "TimePeriod",
            Self::MeterRegister => "MeterRegister",
            Self::TimeOfUseRegister => "TimeOfUseRegister",
            Self::ExternalCostBlock => "ExternalCostBlock",
            Self::ExternalCostPosition => "ExternalCostPosition",
            Self::CostBlock => "CostBlock",
            Self::CostPosition => "CostPosition",
            Self::Surcharge => "Surcharge",
            Self::SurchargePerLocation => "SurchargePerLocation",
            Self::RegionalSurcharge => "RegionalSurcharge",
            Self::PositionSurcharge => "PositionSurcharge",
            Self::TariffCalculationParameter => "TariffCalculationParameter",
            Self::TariffRestriction => "TariffRestriction",
            Self::ContractConditions => "ContractConditions",
            Self::ContractPart => "ContractPart",
            Self::EnergySource => "EnergySource",
            Self::EnergyMix => "EnergyMix",
            Self::InvoicePosition => "InvoicePosition",
            Self::OfferPosition => "OfferPosition",
            Self::OfferPart => "OfferPart",
            Self::OfferVariant => "OfferVariant",
            Self::ContactMethod => "ContactMethod",
            Self::Signature => "Signature",
            Self::Responsibility => "Responsibility",
            Self::PriceGuarantee => "PriceGuarantee",
            Self::RegionCriterion => "RegionCriterion",
            Self::Consumption => "Consumption",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_type_serialize() {
        let typ = ComType::Address;
        let json = serde_json::to_string(&typ).unwrap();
        assert_eq!(json, r#""Adresse""#);
    }

    #[test]
    fn test_com_type_deserialize() {
        let typ: ComType = serde_json::from_str(r#""Adresse""#).unwrap();
        assert_eq!(typ, ComType::Address);
    }

    #[test]
    fn test_com_type_names() {
        let typ = ComType::PriceGuarantee;
        assert_eq!(typ.german_name(), "Preisgarantie");
        assert_eq!(typ.english_name(), "PriceGuarantee");
    }
}
