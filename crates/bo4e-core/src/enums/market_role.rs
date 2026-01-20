//! Market role (Marktrolle) enumeration.

use serde::{Deserialize, Serialize};

/// Role of a market participant in the energy market.
///
/// German: Marktrolle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Marktrolle"))]
#[non_exhaustive]
pub enum MarketRole {
    /// Technical resource operator (Betreiber einer technischen Ressource)
    #[serde(rename = "BTR")]
    TechnicalResourceOperator,

    /// Balance coordinator / market area manager (Bilanzkoordinator / Marktgebietsverantwortlicher)
    #[serde(rename = "BIKO")]
    BalanceCoordinator,

    /// Balance responsible party (Bilanzkreisverantwortlicher)
    #[serde(rename = "BKV")]
    BalanceResponsibleParty,

    /// Data provider
    #[serde(rename = "DP")]
    DataProvider,

    /// Deployment responsible (Einsatzverantwortlicher)
    #[serde(rename = "EIV")]
    DeploymentResponsible,

    /// Energy service provider of connection user (Energieserviceanbieter des Anschlussnutzers)
    #[serde(rename = "ESA")]
    EnergyServiceProvider,

    /// Capacity user (Kapazitaetsnutzer)
    #[serde(rename = "KN")]
    CapacityUser,

    /// Supplier (Lieferant)
    #[serde(rename = "LF")]
    Supplier,

    /// Market area manager (Marktgebietsverantwortlicher)
    #[serde(rename = "MGV")]
    MarketAreaManager,

    /// Metering point operator (Messstellenbetreiber)
    #[serde(rename = "MSB")]
    MeteringPointOperator,

    /// Network operator (Netzbetreiber)
    #[serde(rename = "NB")]
    NetworkOperator,

    /// Register operator (Registerbetreiber)
    #[serde(rename = "RB")]
    RegisterOperator,

    /// Transmission system operator (Uebertragungsnetzbetreiber)
    #[serde(rename = "UENB")]
    TransmissionSystemOperator,
}

impl MarketRole {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::TechnicalResourceOperator => "Betreiber einer technischen Ressource",
            Self::BalanceCoordinator => "Bilanzkoordinator",
            Self::BalanceResponsibleParty => "Bilanzkreisverantwortlicher",
            Self::DataProvider => "Data Provider",
            Self::DeploymentResponsible => "Einsatzverantwortlicher",
            Self::EnergyServiceProvider => "Energieserviceanbieter des Anschlussnutzers",
            Self::CapacityUser => "Kapazitaetsnutzer",
            Self::Supplier => "Lieferant",
            Self::MarketAreaManager => "Marktgebietsverantwortlicher",
            Self::MeteringPointOperator => "Messstellenbetreiber",
            Self::NetworkOperator => "Netzbetreiber",
            Self::RegisterOperator => "Registerbetreiber",
            Self::TransmissionSystemOperator => "Uebertragungsnetzbetreiber",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MarketRole::NetworkOperator).unwrap(),
            r#""NB""#
        );
        assert_eq!(
            serde_json::to_string(&MarketRole::Supplier).unwrap(),
            r#""LF""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<MarketRole>(r#""MSB""#).unwrap(),
            MarketRole::MeteringPointOperator
        );
        assert_eq!(
            serde_json::from_str::<MarketRole>(r#""BKV""#).unwrap(),
            MarketRole::BalanceResponsibleParty
        );
    }

    #[test]
    fn test_roundtrip() {
        for role in [
            MarketRole::TechnicalResourceOperator,
            MarketRole::BalanceCoordinator,
            MarketRole::BalanceResponsibleParty,
            MarketRole::DataProvider,
            MarketRole::DeploymentResponsible,
            MarketRole::EnergyServiceProvider,
            MarketRole::CapacityUser,
            MarketRole::Supplier,
            MarketRole::MarketAreaManager,
            MarketRole::MeteringPointOperator,
            MarketRole::NetworkOperator,
            MarketRole::RegisterOperator,
            MarketRole::TransmissionSystemOperator,
        ] {
            let json = serde_json::to_string(&role).unwrap();
            let parsed: MarketRole = serde_json::from_str(&json).unwrap();
            assert_eq!(role, parsed);
        }
    }
}
