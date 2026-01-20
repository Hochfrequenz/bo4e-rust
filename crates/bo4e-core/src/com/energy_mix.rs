//! Energy mix (Energiemix) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Division, EcoCertificate, EcoLabel};
use crate::traits::{Bo4eMeta, Bo4eObject};

use super::EnergySource;

/// The composition of energy sources for a supplier's energy mix.
///
/// German: Energiemix
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::{EnergyMix, EnergySource};
/// use bo4e_core::enums::GenerationType;
///
/// let mix = EnergyMix {
///     energy_mix_number: Some(1),
///     designation: Some("Green Energy Mix".to_string()),
///     valid_year: Some(2024),
///     sources: vec![
///         EnergySource {
///             generation_type: Some(GenerationType::Solar),
///             percentage_share: Some(40.0),
///             ..Default::default()
///         },
///         EnergySource {
///             generation_type: Some(GenerationType::Wind),
///             percentage_share: Some(60.0),
///             ..Default::default()
///         },
///     ],
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyMix {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Unique identifier for the energy mix (Energiemixnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_mix_number: Option<i32>,

    /// Energy type/division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Name/designation of the energy mix (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designation: Option<String>,

    /// Year for which this mix applies (Gültigkeitsjahr)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_year: Option<i32>,

    /// Individual energy sources and their shares (Anteil)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<EnergySource>,

    /// Notes about the mix (Bemerkung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// CO₂ emissions in g/kWh (CO2-Emission)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_emission: Option<f64>,

    /// Nuclear waste in g/kWh (Atommüll)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuclear_waste: Option<f64>,

    /// Environmental certificates (Ökozertifikate)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub eco_certificates: Vec<EcoCertificate>,

    /// Eco-labels (Ökolabel)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub eco_labels: Vec<EcoLabel>,

    /// Whether provider is in eco top ten (Ist in Öko Top Ten)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_eco_top_ten: Option<bool>,

    /// Website for published energy mix data (Website)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl Bo4eObject for EnergyMix {
    fn type_name_german() -> &'static str {
        "Energiemix"
    }

    fn type_name_english() -> &'static str {
        "EnergyMix"
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
    use crate::enums::GenerationType;

    #[test]
    fn test_green_energy_mix() {
        let mix = EnergyMix {
            energy_mix_number: Some(1),
            designation: Some("100% Renewable".to_string()),
            valid_year: Some(2024),
            sources: vec![
                EnergySource {
                    generation_type: Some(GenerationType::Solar),
                    percentage_share: Some(40.0),
                    ..Default::default()
                },
                EnergySource {
                    generation_type: Some(GenerationType::Wind),
                    percentage_share: Some(60.0),
                    ..Default::default()
                },
            ],
            co2_emission: Some(0.0),
            nuclear_waste: Some(0.0),
            in_eco_top_ten: Some(true),
            ..Default::default()
        };

        assert_eq!(mix.sources.len(), 2);
        assert_eq!(mix.co2_emission, Some(0.0));
        assert_eq!(mix.in_eco_top_ten, Some(true));
    }

    #[test]
    fn test_default() {
        let mix = EnergyMix::default();
        assert!(mix.energy_mix_number.is_none());
        assert!(mix.designation.is_none());
        assert!(mix.sources.is_empty());
    }

    #[test]
    fn test_serialize() {
        let mix = EnergyMix {
            energy_mix_number: Some(42),
            designation: Some("Test Mix".to_string()),
            valid_year: Some(2024),
            co2_emission: Some(150.5),
            ..Default::default()
        };

        let json = serde_json::to_string(&mix).unwrap();
        assert!(json.contains(r#""energyMixNumber":42"#));
        assert!(json.contains(r#""designation":"Test Mix""#));
        assert!(json.contains(r#""co2Emission":150.5"#));
    }

    #[test]
    fn test_roundtrip() {
        let mix = EnergyMix {
            energy_mix_number: Some(1),
            division: Some(Division::Electricity),
            designation: Some("Ökostrom".to_string()),
            valid_year: Some(2024),
            sources: vec![EnergySource {
                generation_type: Some(GenerationType::Hydro),
                percentage_share: Some(100.0),
                ..Default::default()
            }],
            co2_emission: Some(0.0),
            eco_labels: vec![EcoLabel::GruenerStrom],
            in_eco_top_ten: Some(true),
            website: Some("https://example.com/energiemix".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&mix).unwrap();
        let parsed: EnergyMix = serde_json::from_str(&json).unwrap();
        assert_eq!(mix, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(EnergyMix::type_name_german(), "Energiemix");
        assert_eq!(EnergyMix::type_name_english(), "EnergyMix");
    }
}
