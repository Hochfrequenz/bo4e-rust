//! Network location (Netzlokation) business object.
//!
//! Represents a location in the electricity or gas network.

use serde::{Deserialize, Serialize};

use crate::com::Address;
use crate::enums::{Division, NetworkLevel};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A network location - a point in the electricity or gas network.
///
/// German: Netzlokation
///
/// A network location represents a physical point in the network
/// infrastructure where energy flows.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::NetworkLocation;
/// use bo4e_core::enums::Division;
///
/// let nelo = NetworkLocation {
///     network_location_id: Some("NELO12345".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Netzlokation"))]
#[serde(rename_all = "camelCase")]
pub struct NetworkLocation {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Network location ID (Netzlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzlokationsId"))]
    pub network_location_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Network level (Netzebene)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzebene"))]
    pub network_level: Option<NetworkLevel>,

    /// Location address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "adresse"))]
    pub address: Option<Address>,

    /// Network operator code (Netzbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzbetreiberCodenummer"))]
    pub network_operator_code: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Associated metering location IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messlokationsIds"))]
    pub metering_location_ids: Vec<String>,

    /// Associated technical resource IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "technischeRessourceIds"))]
    pub technical_resource_ids: Vec<String>,
}

impl Bo4eObject for NetworkLocation {
    fn type_name_german() -> &'static str {
        "Netzlokation"
    }

    fn type_name_english() -> &'static str {
        "NetworkLocation"
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
    fn test_network_location_creation() {
        let nelo = NetworkLocation {
            network_location_id: Some("NELO12345".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(nelo.network_location_id, Some("NELO12345".to_string()));
    }

    #[test]
    fn test_serialize() {
        let nelo = NetworkLocation {
            meta: Bo4eMeta::with_type("Netzlokation"),
            network_location_id: Some("NELO12345".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&nelo).unwrap();
        assert!(json.contains(r#""_typ":"Netzlokation""#));
    }

    #[test]
    fn test_roundtrip() {
        let nelo = NetworkLocation {
            meta: Bo4eMeta::with_type("Netzlokation"),
            network_location_id: Some("NELO12345".to_string()),
            division: Some(Division::Electricity),
            network_level: Some(NetworkLevel::LowVoltage),
            ..Default::default()
        };

        let json = serde_json::to_string(&nelo).unwrap();
        let parsed: NetworkLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(nelo, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(NetworkLocation::type_name_german(), "Netzlokation");
        assert_eq!(NetworkLocation::type_name_english(), "NetworkLocation");
    }
}
