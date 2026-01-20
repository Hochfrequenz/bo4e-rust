//! Region criterion type (Regionskriteriumtyp) enumeration.

use serde::{Deserialize, Serialize};

/// Classification of criteria for regional delimitation.
///
/// German: Regionskriteriumtyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Regionskriteriumtyp"))]
#[non_exhaustive]
pub enum RegionCriterionType {
    /// Official federal state code (Bundeslandkennziffer)
    #[serde(rename = "BUNDESLANDKENNZIFFER")]
    FederalStateCode,

    /// Federal state name (Bundesland Name)
    #[serde(rename = "BUNDESLAND_NAME")]
    FederalStateName,

    /// Official market area code number (Marktgebiet-Codenummer)
    #[serde(rename = "MARKTGEBIET_NUMMER")]
    MarketAreaNumber,

    /// Market area name (Marktgebiet Name)
    #[serde(rename = "MARKTGEBIET_NAME")]
    MarketAreaName,

    /// Official control area number (Regelgebiet Nummer)
    #[serde(rename = "REGELGEBIET_NUMMER")]
    ControlAreaNumber,

    /// Control area name (Regelgebiet Name)
    #[serde(rename = "REGELGEBIET_NAME")]
    ControlAreaName,

    /// Electricity network identification (Netz Strom)
    #[serde(rename = "NETZ_STROM")]
    ElectricityNetwork,

    /// Gas network identification (Netz Gas)
    #[serde(rename = "NETZ_GAS")]
    GasNetwork,

    /// Official electricity network operator code (Netzbetreiber-Codenummer Strom)
    #[serde(rename = "NETZBETREIBER_NUMMER_STROM")]
    ElectricityNetworkOperatorNumber,

    /// Official gas network operator code (Netzbetreiber-Codenummer Gas)
    #[serde(rename = "NETZBETREIBER_NUMMER_GAS")]
    GasNetworkOperatorNumber,

    /// Electricity network operator name (Netzbetreiber Name Strom)
    #[serde(rename = "NETZBETREIBER_NAME_STROM")]
    ElectricityNetworkOperatorName,

    /// Gas network operator name (Netzbetreiber Name Gas)
    #[serde(rename = "NETZBETREIBER_NAME_GAS")]
    GasNetworkOperatorName,

    /// Balancing area number (Electricity: Bilanzierungsgebietsnummer, Gas: Netzkontonummer)
    #[serde(rename = "BILANZIERUNGS_GEBIET_NUMMER")]
    BalancingAreaNumber,

    /// Official metering service operator code (MSB-Codenummer)
    #[serde(rename = "MSB_NUMMER")]
    MeteringServiceOperatorNumber,

    /// Metering service operator name (MSB Name)
    #[serde(rename = "MSB_NAME")]
    MeteringServiceOperatorName,

    /// Official supplier code number (Lieferanten-Codenummer)
    #[serde(rename = "VERSORGER_NUMMER")]
    SupplierNumber,

    /// Supplier name (Versorger Name)
    #[serde(rename = "VERSORGER_NAME")]
    SupplierName,

    /// Official electricity basic supplier code (Strom-Grundversorger Codenummer)
    #[serde(rename = "GRUNDVERSORGER_NUMMER_STROM")]
    ElectricityBasicSupplierNumber,

    /// Electricity basic supplier name (Strom-Grundversorger Name)
    #[serde(rename = "GRUNDVERSORGER_NAME_STROM")]
    ElectricityBasicSupplierName,

    /// Official gas basic supplier code (Gas-Grundversorger Codenummer)
    #[serde(rename = "GRUNDVERSORGER_NUMMER_GAS")]
    GasBasicSupplierNumber,

    /// Gas basic supplier name (Gas-Grundversorger Name)
    #[serde(rename = "GRUNDVERSORGER_NAME_GAS")]
    GasBasicSupplierName,

    /// District name (Kreis)
    #[serde(rename = "KREIS_NAME")]
    DistrictName,

    /// Official district code (Kreiskennziffer)
    #[serde(rename = "KREISKENNZIFFER")]
    DistrictCode,

    /// Municipality name (Gemeinde)
    #[serde(rename = "GEMEINDE_NAME")]
    MunicipalityName,

    /// Official municipality code (Gemeindekennziffer)
    #[serde(rename = "GEMEINDEKENNZIFFER")]
    MunicipalityCode,

    /// Postal code (Postleitzahl)
    #[serde(rename = "POSTLEITZAHL")]
    PostalCode,

    /// City/Town (Ort)
    #[serde(rename = "ORT")]
    City,

    /// Combination of postal code and city (Postleitzahl und Ort)
    #[serde(rename = "POSTORT")]
    PostalCity,

    /// Municipality population (Einwohnerzahl Gemeinde)
    #[serde(rename = "EINWOHNERZAHL_GEMEINDE")]
    MunicipalityPopulation,

    /// City population (Einwohnerzahl Ort)
    #[serde(rename = "EINWOHNERZAHL_ORT")]
    CityPopulation,

    /// Radius in kilometers (km Umkreis)
    #[serde(rename = "KM_UMKREIS")]
    RadiusKm,

    /// Nationwide consideration (Bundesweit)
    #[serde(rename = "BUNDESWEIT")]
    Nationwide,

    /// Postal code range (Postleitzahlenbereich)
    #[serde(rename = "PLZ_BEREICH")]
    PostalCodeRange,
}

impl RegionCriterionType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::FederalStateCode => "Bundeslandkennziffer",
            Self::FederalStateName => "Bundesland Name",
            Self::MarketAreaNumber => "Marktgebiet-Codenummer",
            Self::MarketAreaName => "Marktgebiet Name",
            Self::ControlAreaNumber => "Regelgebiet Nummer",
            Self::ControlAreaName => "Regelgebiet Name",
            Self::ElectricityNetwork => "Netz Strom",
            Self::GasNetwork => "Netz Gas",
            Self::ElectricityNetworkOperatorNumber => "Netzbetreiber-Codenummer Strom",
            Self::GasNetworkOperatorNumber => "Netzbetreiber-Codenummer Gas",
            Self::ElectricityNetworkOperatorName => "Netzbetreiber Name Strom",
            Self::GasNetworkOperatorName => "Netzbetreiber Name Gas",
            Self::BalancingAreaNumber => "Bilanzierungsgebietsnummer",
            Self::MeteringServiceOperatorNumber => "MSB-Codenummer",
            Self::MeteringServiceOperatorName => "MSB Name",
            Self::SupplierNumber => "Lieferanten-Codenummer",
            Self::SupplierName => "Versorger Name",
            Self::ElectricityBasicSupplierNumber => "Strom-Grundversorger Codenummer",
            Self::ElectricityBasicSupplierName => "Strom-Grundversorger Name",
            Self::GasBasicSupplierNumber => "Gas-Grundversorger Codenummer",
            Self::GasBasicSupplierName => "Gas-Grundversorger Name",
            Self::DistrictName => "Kreis",
            Self::DistrictCode => "Kreiskennziffer",
            Self::MunicipalityName => "Gemeinde",
            Self::MunicipalityCode => "Gemeindekennziffer",
            Self::PostalCode => "Postleitzahl",
            Self::City => "Ort",
            Self::PostalCity => "Postleitzahl und Ort",
            Self::MunicipalityPopulation => "Einwohnerzahl Gemeinde",
            Self::CityPopulation => "Einwohnerzahl Ort",
            Self::RadiusKm => "km Umkreis",
            Self::Nationwide => "Bundesweit",
            Self::PostalCodeRange => "Postleitzahlenbereich",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&RegionCriterionType::PostalCode).unwrap(),
            r#""POSTLEITZAHL""#
        );
        assert_eq!(
            serde_json::to_string(&RegionCriterionType::Nationwide).unwrap(),
            r#""BUNDESWEIT""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<RegionCriterionType>(r#""ORT""#).unwrap(),
            RegionCriterionType::City
        );
    }

    #[test]
    fn test_roundtrip() {
        for criterion in [
            RegionCriterionType::FederalStateCode,
            RegionCriterionType::FederalStateName,
            RegionCriterionType::MarketAreaNumber,
            RegionCriterionType::MarketAreaName,
            RegionCriterionType::ControlAreaNumber,
            RegionCriterionType::ControlAreaName,
            RegionCriterionType::ElectricityNetwork,
            RegionCriterionType::GasNetwork,
            RegionCriterionType::ElectricityNetworkOperatorNumber,
            RegionCriterionType::GasNetworkOperatorNumber,
            RegionCriterionType::ElectricityNetworkOperatorName,
            RegionCriterionType::GasNetworkOperatorName,
            RegionCriterionType::BalancingAreaNumber,
            RegionCriterionType::MeteringServiceOperatorNumber,
            RegionCriterionType::MeteringServiceOperatorName,
            RegionCriterionType::SupplierNumber,
            RegionCriterionType::SupplierName,
            RegionCriterionType::ElectricityBasicSupplierNumber,
            RegionCriterionType::ElectricityBasicSupplierName,
            RegionCriterionType::GasBasicSupplierNumber,
            RegionCriterionType::GasBasicSupplierName,
            RegionCriterionType::DistrictName,
            RegionCriterionType::DistrictCode,
            RegionCriterionType::MunicipalityName,
            RegionCriterionType::MunicipalityCode,
            RegionCriterionType::PostalCode,
            RegionCriterionType::City,
            RegionCriterionType::PostalCity,
            RegionCriterionType::MunicipalityPopulation,
            RegionCriterionType::CityPopulation,
            RegionCriterionType::RadiusKm,
            RegionCriterionType::Nationwide,
            RegionCriterionType::PostalCodeRange,
        ] {
            let json = serde_json::to_string(&criterion).unwrap();
            let parsed: RegionCriterionType = serde_json::from_str(&json).unwrap();
            assert_eq!(criterion, parsed);
        }
    }
}
