//! Location properties (Standorteigenschaften) business object.
//!
//! Represents properties and characteristics of a physical location.

use serde::{Deserialize, Serialize};

use crate::com::{Address, GeoCoordinates};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// Properties of a physical location.
///
/// German: Standorteigenschaften
///
/// Location properties describe characteristics of a physical
/// site, such as address, coordinates, and site-specific details.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::LocationProperties;
///
/// let props = LocationProperties {
///     location_properties_id: Some("LOC001".to_string()),
///     building_type: Some("Residential".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Standorteigenschaften"))]
#[serde(rename_all = "camelCase")]
pub struct LocationProperties {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Location properties ID (Standorteigenschaften-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "standorteigenschaftenId"))]
    pub location_properties_id: Option<String>,

    /// Location address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "adresse"))]
    pub address: Option<Address>,

    /// Geographic coordinates (Geokoordinaten)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geokoordinaten"))]
    pub coordinates: Option<GeoCoordinates>,

    /// Building type (Gebaeudeart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "gebaeudeart"))]
    pub building_type: Option<String>,

    /// Construction year (Baujahr)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "baujahr"))]
    pub construction_year: Option<i32>,

    /// Floor area in square meters (Flaeche)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "flaeche"))]
    pub floor_area: Option<f64>,

    /// Number of floors (Anzahl Etagen)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "anzahlEtagen"))]
    pub number_of_floors: Option<i32>,

    /// Number of residential units (Anzahl Wohneinheiten)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "anzahlWohneinheiten"))]
    pub number_of_units: Option<i32>,

    /// Heating type (Heizungsart)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "heizungsart"))]
    pub heating_type: Option<String>,

    /// Energy efficiency class (Energieeffizienzklasse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "energieeffizienzklasse"))]
    pub energy_efficiency_class: Option<String>,

    /// Has solar installation (Hat Solaranlage)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "hatSolaranlage"))]
    pub has_solar: Option<bool>,

    /// Has electric vehicle charging (Hat E-Ladestation)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "hatELadestation"))]
    pub has_ev_charging: Option<bool>,

    /// Has heat pump (Hat Waermepumpe)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "hatWaermepumpe"))]
    pub has_heat_pump: Option<bool>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,
}

impl Bo4eObject for LocationProperties {
    fn type_name_german() -> &'static str {
        "Standorteigenschaften"
    }

    fn type_name_english() -> &'static str {
        "LocationProperties"
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
    fn test_location_properties_creation() {
        let props = LocationProperties {
            location_properties_id: Some("LOC001".to_string()),
            building_type: Some("Residential".to_string()),
            ..Default::default()
        };

        assert_eq!(props.location_properties_id, Some("LOC001".to_string()));
    }

    #[test]
    fn test_serialize() {
        let props = LocationProperties {
            meta: Bo4eMeta::with_type("Standorteigenschaften"),
            location_properties_id: Some("LOC001".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&props).unwrap();
        assert!(json.contains(r#""_typ":"Standorteigenschaften""#));
    }

    #[test]
    fn test_roundtrip() {
        let props = LocationProperties {
            meta: Bo4eMeta::with_type("Standorteigenschaften"),
            location_properties_id: Some("LOC001".to_string()),
            building_type: Some("Residential".to_string()),
            construction_year: Some(2010),
            floor_area: Some(150.0),
            number_of_floors: Some(2),
            has_solar: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&props).unwrap();
        let parsed: LocationProperties = serde_json::from_str(&json).unwrap();
        assert_eq!(props, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            LocationProperties::type_name_german(),
            "Standorteigenschaften"
        );
        assert_eq!(
            LocationProperties::type_name_english(),
            "LocationProperties"
        );
    }
}
