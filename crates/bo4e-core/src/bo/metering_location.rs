//! Metering location (Messlokation) business object.
//!
//! Represents the physical location where energy measurement takes place.

use serde::{Deserialize, Serialize};

use crate::com::{Address, GeoCoordinates, Hardware};
use crate::enums::Division;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A metering location (MeLo) - where measurement takes place.
///
/// German: Messlokation
///
/// A metering location represents the physical point where energy
/// is measured. It has a 33-character identifier.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::MeteringLocation;
/// use bo4e_core::enums::Division;
///
/// let melo = MeteringLocation {
///     metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Messlokation"))]
#[serde(rename_all = "camelCase")]
pub struct MeteringLocation {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Metering location ID - 33 characters (Messlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "messlokationsId"))]
    pub metering_location_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Location address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "adresse"))]
    pub address: Option<Address>,

    /// Geographic coordinates (Geokoordinaten)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geokoordinaten"))]
    pub coordinates: Option<GeoCoordinates>,

    /// Metering point operator code (Messstellenbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(
        feature = "json-schema",
        schemars(rename = "messstellenbetreiberCodenummer")
    )]
    pub metering_operator_code: Option<String>,

    /// Network operator code (Netzbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "netzbetreiberCodenummer"))]
    pub network_operator_code: Option<String>,

    /// Grid area (Regelzone)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "regelzone"))]
    pub grid_area: Option<String>,

    /// Description of the metering location (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,

    /// Hardware at this metering location (Geraete)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geraete"))]
    pub hardware: Vec<Hardware>,

    /// Associated meter IDs (Zaehler)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "zaehler"))]
    pub meter_ids: Vec<String>,

    /// Associated market location IDs (Marktlokationen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktlokationen"))]
    pub market_location_ids: Vec<String>,
}

impl Bo4eObject for MeteringLocation {
    fn type_name_german() -> &'static str {
        "Messlokation"
    }

    fn type_name_english() -> &'static str {
        "MeteringLocation"
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
    fn test_melo_creation() {
        let melo = MeteringLocation {
            metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert!(melo.metering_location_id.is_some());
    }

    #[test]
    fn test_serialize() {
        let melo = MeteringLocation {
            meta: Bo4eMeta::with_type("Messlokation"),
            metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&melo).unwrap();
        assert!(json.contains(r#""_typ":"Messlokation""#));
        assert!(json.contains("DE00012345678901234567890123456789"));
    }

    #[test]
    fn test_roundtrip() {
        let melo = MeteringLocation {
            meta: Bo4eMeta::with_type("Messlokation"),
            metering_location_id: Some("DE00012345678901234567890123456789".to_string()),
            division: Some(Division::Electricity),
            metering_operator_code: Some("9900001234".to_string()),
            meter_ids: vec!["METER001".to_string()],
            ..Default::default()
        };

        let json = serde_json::to_string(&melo).unwrap();
        let parsed: MeteringLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(melo, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MeteringLocation::type_name_german(), "Messlokation");
        assert_eq!(MeteringLocation::type_name_english(), "MeteringLocation");
    }
}
