//! Technical resource (TechnischeRessource) business object.
//!
//! Represents a technical resource in the energy infrastructure.

use serde::{Deserialize, Serialize};

use crate::com::Address;
use crate::enums::{Division, EnergyDirection, TechnicalResourceUsage};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A technical resource in the energy infrastructure.
///
/// German: TechnischeRessource
///
/// Technical resources are physical components that produce,
/// consume, or store energy.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::TechnicalResource;
/// use bo4e_core::enums::{Division, TechnicalResourceUsage};
///
/// let resource = TechnicalResource {
///     technical_resource_id: Some("TR001".to_string()),
///     division: Some(Division::Electricity),
///     usage: Some(TechnicalResourceUsage::ElectricityGenerationType),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "TechnischeRessource"))]
#[serde(rename_all = "camelCase")]
pub struct TechnicalResource {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Technical resource ID (TechnischeRessource-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "technischeRessourceId"))]
    pub technical_resource_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Usage type (Verwendungszweck)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "verwendungszweck"))]
    pub usage: Option<TechnicalResourceUsage>,

    /// Energy direction (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energierichtung"))]
    pub energy_direction: Option<EnergyDirection>,

    /// Location address (Standort)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "standort"))]
    pub address: Option<Address>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Nominal power in kW (Nennleistung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "nennleistung"))]
    pub nominal_power: Option<f64>,

    /// Maximum power in kW (Maximalleistung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "maximalleistung"))]
    pub max_power: Option<f64>,

    /// Minimum power in kW (Minimalleistung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "minimalleistung"))]
    pub min_power: Option<f64>,

    /// Energy capacity in kWh (Speicherkapazitaet)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "speicherkapazitaet"))]
    pub energy_capacity: Option<f64>,

    /// Associated metering location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messlokationsId"))]
    pub metering_location_id: Option<String>,

    /// Associated market location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktlokationsId"))]
    pub market_location_id: Option<String>,

    /// Commissioning date (Inbetriebnahmedatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "inbetriebnahmedatum"))]
    pub commissioning_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Decommissioning date (Stilllegungsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "stilllegungsdatum"))]
    pub decommissioning_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl Bo4eObject for TechnicalResource {
    fn type_name_german() -> &'static str {
        "TechnischeRessource"
    }

    fn type_name_english() -> &'static str {
        "TechnicalResource"
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
    fn test_technical_resource_creation() {
        let resource = TechnicalResource {
            technical_resource_id: Some("TR001".to_string()),
            division: Some(Division::Electricity),
            usage: Some(TechnicalResourceUsage::ElectricityGenerationType),
            ..Default::default()
        };

        assert_eq!(resource.technical_resource_id, Some("TR001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let resource = TechnicalResource {
            meta: Bo4eMeta::with_type("TechnischeRessource"),
            technical_resource_id: Some("TR001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&resource).unwrap();
        assert!(json.contains(r#""_typ":"TechnischeRessource""#));
    }

    #[test]
    fn test_roundtrip() {
        let resource = TechnicalResource {
            meta: Bo4eMeta::with_type("TechnischeRessource"),
            technical_resource_id: Some("TR001".to_string()),
            division: Some(Division::Electricity),
            usage: Some(TechnicalResourceUsage::ElectricityGenerationType),
            nominal_power: Some(100.0),
            max_power: Some(120.0),
            ..Default::default()
        };

        let json = serde_json::to_string(&resource).unwrap();
        let parsed: TechnicalResource = serde_json::from_str(&json).unwrap();
        assert_eq!(resource, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TechnicalResource::type_name_german(), "TechnischeRessource");
        assert_eq!(TechnicalResource::type_name_english(), "TechnicalResource");
    }
}
