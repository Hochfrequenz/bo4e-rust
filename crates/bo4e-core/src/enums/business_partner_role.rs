//! Business partner role (Geschaeftspartnerrolle) enumeration.

use serde::{Deserialize, Serialize};

/// Role of a business partner in the energy market.
///
/// These roles define the capacity in which a business partner operates.
///
/// German: Geschaeftspartnerrolle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Geschaeftspartnerrolle"))]
#[non_exhaustive]
pub enum BusinessPartnerRole {
    /// Supplier (Lieferant)
    #[serde(rename = "LIEFERANT")]
    Supplier,

    /// Service provider (Dienstleister)
    #[serde(rename = "DIENSTLEISTER")]
    ServiceProvider,

    /// Customer (Kunde)
    #[serde(rename = "KUNDE")]
    Customer,

    /// Interested party (Interessent)
    #[serde(rename = "INTERESSENT")]
    InterestedParty,

    /// Market partner (Marktpartner)
    #[serde(rename = "MARKTPARTNER")]
    MarketPartner,

    /// Network operator (Netzbetreiber)
    #[serde(rename = "NETZBETREIBER")]
    NetworkOperator,
}

impl BusinessPartnerRole {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Supplier => "Lieferant",
            Self::ServiceProvider => "Dienstleister",
            Self::Customer => "Kunde",
            Self::InterestedParty => "Interessent",
            Self::MarketPartner => "Marktpartner",
            Self::NetworkOperator => "Netzbetreiber",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&BusinessPartnerRole::Supplier).unwrap(),
            r#""LIEFERANT""#
        );
        assert_eq!(
            serde_json::to_string(&BusinessPartnerRole::Customer).unwrap(),
            r#""KUNDE""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<BusinessPartnerRole>(r#""LIEFERANT""#).unwrap(),
            BusinessPartnerRole::Supplier
        );
        assert_eq!(
            serde_json::from_str::<BusinessPartnerRole>(r#""DIENSTLEISTER""#).unwrap(),
            BusinessPartnerRole::ServiceProvider
        );
    }

    #[test]
    fn test_roundtrip() {
        for role in [
            BusinessPartnerRole::Supplier,
            BusinessPartnerRole::ServiceProvider,
            BusinessPartnerRole::Customer,
            BusinessPartnerRole::InterestedParty,
            BusinessPartnerRole::MarketPartner,
            BusinessPartnerRole::NetworkOperator,
        ] {
            let json = serde_json::to_string(&role).unwrap();
            let parsed: BusinessPartnerRole = serde_json::from_str(&json).unwrap();
            assert_eq!(role, parsed);
        }
    }
}
