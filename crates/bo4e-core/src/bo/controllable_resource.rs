//! Controllable resource (SteuerbareRessource) business object.
//!
//! Represents a resource that can be remotely controlled for demand response.

use serde::{Deserialize, Serialize};

use crate::com::Address;
use crate::enums::{ControllableResourceType, Division, EnergyDirection};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A controllable resource for demand response.
///
/// German: SteuerbareRessource
///
/// Controllable resources can be remotely controlled to
/// participate in demand response programs.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::ControllableResource;
/// use bo4e_core::enums::{Division, ControllableResourceType};
///
/// let resource = ControllableResource {
///     controllable_resource_id: Some("SR001".to_string()),
///     division: Some(Division::Electricity),
///     resource_type: Some(ControllableResourceType::OnOff),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllableResource {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Controllable resource ID (SteuerbareRessource-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controllable_resource_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Resource type (Ressourcentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ControllableResourceType>,

    /// Energy direction (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_direction: Option<EnergyDirection>,

    /// Location address (Standort)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Controllable power in kW (Steuerbare Leistung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controllable_power: Option<f64>,

    /// Minimum activation time in minutes (Mindestaktivierungszeit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_activation_time: Option<i32>,

    /// Maximum activation time in minutes (Maximalaktivierungszeit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_activation_time: Option<i32>,

    /// Ramp up time in seconds (Hochlaufzeit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramp_up_time: Option<i32>,

    /// Ramp down time in seconds (Herunterlaufzeit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramp_down_time: Option<i32>,

    /// Associated technical resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_resource_id: Option<String>,

    /// Associated market location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location_id: Option<String>,

    /// Is currently active/available (Ist aktiv)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl Bo4eObject for ControllableResource {
    fn type_name_german() -> &'static str {
        "SteuerbareRessource"
    }

    fn type_name_english() -> &'static str {
        "ControllableResource"
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
    fn test_controllable_resource_creation() {
        let resource = ControllableResource {
            controllable_resource_id: Some("SR001".to_string()),
            division: Some(Division::Electricity),
            resource_type: Some(ControllableResourceType::OnOff),
            ..Default::default()
        };

        assert_eq!(resource.controllable_resource_id, Some("SR001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let resource = ControllableResource {
            meta: Bo4eMeta::with_type("SteuerbareRessource"),
            controllable_resource_id: Some("SR001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&resource).unwrap();
        assert!(json.contains(r#""_typ":"SteuerbareRessource""#));
    }

    #[test]
    fn test_roundtrip() {
        let resource = ControllableResource {
            meta: Bo4eMeta::with_type("SteuerbareRessource"),
            controllable_resource_id: Some("SR001".to_string()),
            division: Some(Division::Electricity),
            resource_type: Some(ControllableResourceType::OnOff),
            controllable_power: Some(50.0),
            min_activation_time: Some(15),
            is_active: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&resource).unwrap();
        let parsed: ControllableResource = serde_json::from_str(&json).unwrap();
        assert_eq!(resource, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            ControllableResource::type_name_german(),
            "SteuerbareRessource"
        );
        assert_eq!(
            ControllableResource::type_name_english(),
            "ControllableResource"
        );
    }
}
