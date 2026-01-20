//! Energy source (Energieherkunft) component.

use serde::{Deserialize, Serialize};

use crate::enums::GenerationType;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An energy source with its generation type and percentage share.
///
/// German: Energieherkunft
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::EnergySource;
/// use bo4e_core::enums::GenerationType;
///
/// let source = EnergySource {
///     generation_type: Some(GenerationType::Solar),
///     percentage_share: Some(35.0),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergySource {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of energy generation (Erzeugungsart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_type: Option<GenerationType>,

    /// Percentage share of this generation type (Anteil in Prozent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_share: Option<f64>,
}

impl Bo4eObject for EnergySource {
    fn type_name_german() -> &'static str {
        "Energieherkunft"
    }

    fn type_name_english() -> &'static str {
        "EnergySource"
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
    fn test_solar_source() {
        let source = EnergySource {
            generation_type: Some(GenerationType::Solar),
            percentage_share: Some(35.0),
            ..Default::default()
        };

        assert_eq!(source.generation_type, Some(GenerationType::Solar));
        assert_eq!(source.percentage_share, Some(35.0));
    }

    #[test]
    fn test_wind_source() {
        let source = EnergySource {
            generation_type: Some(GenerationType::Wind),
            percentage_share: Some(45.0),
            ..Default::default()
        };

        assert_eq!(source.generation_type, Some(GenerationType::Wind));
    }

    #[test]
    fn test_default() {
        let source = EnergySource::default();
        assert!(source.generation_type.is_none());
        assert!(source.percentage_share.is_none());
    }

    #[test]
    fn test_serialize() {
        let source = EnergySource {
            generation_type: Some(GenerationType::Solar),
            percentage_share: Some(25.5),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains(r#""generationType":"SOLAR""#));
        assert!(json.contains(r#""percentageShare":25.5"#));
    }

    #[test]
    fn test_roundtrip() {
        let source = EnergySource {
            generation_type: Some(GenerationType::Wind),
            percentage_share: Some(60.0),
            ..Default::default()
        };

        let json = serde_json::to_string(&source).unwrap();
        let parsed: EnergySource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(EnergySource::type_name_german(), "Energieherkunft");
        assert_eq!(EnergySource::type_name_english(), "EnergySource");
    }
}
