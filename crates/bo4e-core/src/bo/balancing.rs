//! Balancing (Bilanzierung) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::TimePeriod;
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Balance group data for energy market balancing.
///
/// German: Bilanzierung
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Balancing;
/// use bo4e_core::enums::Division;
///
/// let balancing = Balancing {
///     balance_group_id: Some("11XBALANCEGROUP".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Bilanzierung"))]
#[serde(rename_all = "camelCase")]
pub struct Balancing {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Balance group identifier (Bilanzkreis-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bilanzkreisId"))]
    pub balance_group_id: Option<String>,

    /// Balance group name (Bilanzkreisname)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bilanzkreisname"))]
    pub balance_group_name: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Market area (Marktgebiet)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktgebiet"))]
    pub market_area: Option<String>,

    /// Balance responsible party (Bilanzkreisverantwortlicher)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "json-schema",
        schemars(rename = "bilanzkreisverantwortlicher")
    )]
    pub balance_responsible_party: Option<Box<super::MarketParticipant>>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gueltigkeitszeitraum"))]
    pub validity_period: Option<TimePeriod>,

    /// Start date of balancing (Startdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "startdatum"))]
    pub start_date: Option<DateTime<Utc>>,

    /// End date of balancing (Enddatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "enddatum"))]
    pub end_date: Option<DateTime<Utc>>,
}

impl Bo4eObject for Balancing {
    fn type_name_german() -> &'static str {
        "Bilanzierung"
    }

    fn type_name_english() -> &'static str {
        "Balancing"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balancing_creation() {
        let balancing = Balancing {
            balance_group_id: Some("11XBALANCEGROUP".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(
            balancing.balance_group_id,
            Some("11XBALANCEGROUP".to_string())
        );
        assert_eq!(balancing.division, Some(Division::Electricity));
    }

    #[test]
    fn test_gas_balancing() {
        let balancing = Balancing {
            balance_group_id: Some("GASBALANCE001".to_string()),
            balance_group_name: Some("Gas Balance Group 1".to_string()),
            division: Some(Division::Gas),
            market_area: Some("NCG".to_string()),
            ..Default::default()
        };

        assert_eq!(balancing.division, Some(Division::Gas));
        assert_eq!(balancing.market_area, Some("NCG".to_string()));
    }

    #[test]
    fn test_serialize() {
        let balancing = Balancing {
            meta: Bo4eMeta::with_type("Bilanzierung"),
            balance_group_id: Some("11XTEST".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&balancing).unwrap();
        assert!(json.contains(r#""balanceGroupId":"11XTEST""#));
    }

    #[test]
    fn test_roundtrip() {
        let balancing = Balancing {
            meta: Bo4eMeta::with_type("Bilanzierung"),
            balance_group_id: Some("11XTEST".to_string()),
            balance_group_name: Some("Test Balance Group".to_string()),
            description: Some("Test description".to_string()),
            division: Some(Division::Electricity),
            market_area: Some("DE".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&balancing).unwrap();
        let parsed: Balancing = serde_json::from_str(&json).unwrap();
        assert_eq!(balancing, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Balancing::type_name_german(), "Bilanzierung");
        assert_eq!(Balancing::type_name_english(), "Balancing");
    }
}
