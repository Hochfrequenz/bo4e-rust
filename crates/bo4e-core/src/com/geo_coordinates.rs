//! Geographic coordinates (Geokoordinaten) component.

use serde::{Deserialize, Serialize};

use crate::traits::{Bo4eMeta, Bo4eObject};

/// Geographic coordinates (latitude/longitude).
///
/// German: Geokoordinaten
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::GeoCoordinates;
///
/// // Coordinates for Cologne Cathedral
/// let coords = GeoCoordinates {
///     latitude: Some(50.9413),
///     longitude: Some(6.9583),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Geokoordinaten"))]
#[serde(rename_all = "camelCase")]
pub struct GeoCoordinates {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Latitude in decimal degrees (Breitengrad)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "breitengrad"))]
    pub latitude: Option<f64>,

    /// Longitude in decimal degrees (Laengengrad)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "laengengrad"))]
    pub longitude: Option<f64>,
}

impl Bo4eObject for GeoCoordinates {
    fn type_name_german() -> &'static str {
        "Geokoordinaten"
    }

    fn type_name_english() -> &'static str {
        "GeoCoordinates"
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
    fn test_geo_coordinates_default() {
        let coords = GeoCoordinates::default();
        assert!(coords.latitude.is_none());
        assert!(coords.longitude.is_none());
    }

    #[test]
    fn test_cologne_coordinates() {
        let coords = GeoCoordinates {
            latitude: Some(50.9375),
            longitude: Some(6.9603),
            ..Default::default()
        };

        let json = serde_json::to_string(&coords).unwrap();
        assert!(json.contains("50.9375"));
        assert!(json.contains("6.9603"));

        let parsed: GeoCoordinates = serde_json::from_str(&json).unwrap();
        assert_eq!(coords, parsed);
    }

    #[test]
    fn test_geo_coordinates_roundtrip() {
        let coords = GeoCoordinates {
            meta: Bo4eMeta::with_type("Geokoordinaten"),
            latitude: Some(52.520008),
            longitude: Some(13.404954),
        };

        let json = serde_json::to_string(&coords).unwrap();
        let parsed: GeoCoordinates = serde_json::from_str(&json).unwrap();
        assert_eq!(coords, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(GeoCoordinates::type_name_german(), "Geokoordinaten");
        assert_eq!(GeoCoordinates::type_name_english(), "GeoCoordinates");
    }
}
