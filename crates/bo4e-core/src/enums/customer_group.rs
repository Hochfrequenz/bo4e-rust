//! Customer group (Kundengruppe) enumeration.

use serde::{Deserialize, Serialize};

/// Customer group for a market location (based on standard load profiles).
///
/// German: Kundengruppe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Kundengruppe"))]
#[non_exhaustive]
pub enum CustomerGroup {
    // RLM profiles (metered)
    /// Customer with registering power measurement (no SLP)
    #[serde(rename = "RLM")]
    Rlm,

    /// Municipal consumption point with power measurement
    #[serde(rename = "RLM_KOMMUNAL")]
    RlmMunicipal,

    /// Municipal consumption point without power measurement
    #[serde(rename = "SLP_KOMMUNAL")]
    SlpMunicipal,

    // Electricity SLP profiles (G - commercial)
    /// General commercial (G0)
    #[serde(rename = "SLP_S_G0")]
    SlpElectricityG0,

    /// Weekdays (G1)
    #[serde(rename = "SLP_S_G1")]
    SlpElectricityG1,

    /// Evening consumption (G2)
    #[serde(rename = "SLP_S_G2")]
    SlpElectricityG2,

    /// Continuous commercial (G3)
    #[serde(rename = "SLP_S_G3")]
    SlpElectricityG3,

    /// Shop, hairdresser (G4)
    #[serde(rename = "SLP_S_G4")]
    SlpElectricityG4,

    /// Bakery with bakehouse (G5)
    #[serde(rename = "SLP_S_G5")]
    SlpElectricityG5,

    /// Weekend operation (G6)
    #[serde(rename = "SLP_S_G6")]
    SlpElectricityG6,

    /// Mobile phone base station (G7)
    #[serde(rename = "SLP_S_G7")]
    SlpElectricityG7,

    // Electricity SLP profiles (L - agriculture)
    /// Agriculture general (L0)
    #[serde(rename = "SLP_S_L0")]
    SlpElectricityL0,

    /// Agriculture with dairy/sideline animal breeding (L1)
    #[serde(rename = "SLP_S_L1")]
    SlpElectricityL1,

    /// Other agricultural businesses (L2)
    #[serde(rename = "SLP_S_L2")]
    SlpElectricityL2,

    // Electricity SLP profiles (H - household)
    /// Household general (H0)
    #[serde(rename = "SLP_S_H0")]
    SlpElectricityH0,

    // Electricity SLP profiles (special)
    /// Street lighting (SB)
    #[serde(rename = "SLP_S_SB")]
    SlpElectricitySb,

    /// Night storage heating (HZ)
    #[serde(rename = "SLP_S_HZ")]
    SlpElectricityHz,

    /// Heat pump (WP)
    #[serde(rename = "SLP_S_WP")]
    SlpElectricityWp,

    /// Electric mobility (EM)
    #[serde(rename = "SLP_S_EM")]
    SlpElectricityEm,

    /// Night storage heating common measurement (HZ_GEM)
    #[serde(rename = "SLP_S_HZ_GEM")]
    SlpElectricityHzGem,

    // Gas SLP profiles
    /// Territorial authorities, credit institutions, insurance, non-profit organizations & public facilities
    #[serde(rename = "SLP_G_GKO")]
    SlpGasGko,

    /// Standard customer group for gas
    #[serde(rename = "SLP_G_STANDARD")]
    SlpGasStandard,

    /// Retail, wholesale
    #[serde(rename = "SLP_G_GHA")]
    SlpGasGha,

    /// Metal, automotive
    #[serde(rename = "SLP_G_GMK")]
    SlpGasGmk,

    /// Other operational services
    #[serde(rename = "SLP_G_GBD")]
    SlpGasGbd,

    /// Accommodation
    #[serde(rename = "SLP_G_GGA")]
    SlpGasGga,

    /// Restaurants
    #[serde(rename = "SLP_G_GBH")]
    SlpGasGbh,

    /// Bakeries
    #[serde(rename = "SLP_G_GBA")]
    SlpGasGba,

    /// Laundries
    #[serde(rename = "SLP_G_GWA")]
    SlpGasGwa,

    /// Horticulture
    #[serde(rename = "SLP_G_GGB")]
    SlpGasGgb,

    /// Paper and printing
    #[serde(rename = "SLP_G_GPD")]
    SlpGasGpd,

    /// Household-like commercial enterprises
    #[serde(rename = "SLP_G_GMF")]
    SlpGasGmf,

    /// Single-family household
    #[serde(rename = "SLP_G_HEF")]
    SlpGasHef,

    /// Multi-family household
    #[serde(rename = "SLP_G_HMF")]
    SlpGasHmf,

    /// Cooking gas
    #[serde(rename = "SLP_G_HKO")]
    SlpGasHko,
}

impl CustomerGroup {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Rlm => "RLM",
            Self::RlmMunicipal => "RLM Kommunal",
            Self::SlpMunicipal => "SLP Kommunal",
            Self::SlpElectricityG0 => "Gewerbe allgemein",
            Self::SlpElectricityG1 => "Werktags",
            Self::SlpElectricityG2 => "Verbrauch in Abendstunden",
            Self::SlpElectricityG3 => "Gewerbe durchlaufend",
            Self::SlpElectricityG4 => "Laden, Friseur",
            Self::SlpElectricityG5 => "Baeckerei mit Backstube",
            Self::SlpElectricityG6 => "Wochenendbetrieb",
            Self::SlpElectricityG7 => "Mobilfunksendestation",
            Self::SlpElectricityL0 => "Landwirtschaft allgemein",
            Self::SlpElectricityL1 => "Landwirtschaft mit Milchwirtschaft",
            Self::SlpElectricityL2 => "Uebrige Landwirtschaftsbetriebe",
            Self::SlpElectricityH0 => "Haushalt allgemein",
            Self::SlpElectricitySb => "Strassenbeleuchtung",
            Self::SlpElectricityHz => "Nachtspeicherheizung",
            Self::SlpElectricityWp => "Waermepumpe",
            Self::SlpElectricityEm => "Elektromobilitaet",
            Self::SlpElectricityHzGem => "Nachtspeicherheizung gemeinsame Messung",
            Self::SlpGasGko => "Gebietskoerperschaften, Kreditinstitute, Versicherungen",
            Self::SlpGasStandard => "Standardkundengruppe Gas",
            Self::SlpGasGha => "Einzelhandel, Grosshandel",
            Self::SlpGasGmk => "Metall, Kfz",
            Self::SlpGasGbd => "sonst. betr. Dienstleistungen",
            Self::SlpGasGga => "Beherbergung",
            Self::SlpGasGbh => "Gaststaetten",
            Self::SlpGasGba => "Baeckereien",
            Self::SlpGasGwa => "Waeschereien",
            Self::SlpGasGgb => "Gartenbau",
            Self::SlpGasGpd => "Papier und Druck",
            Self::SlpGasGmf => "haushaltsaehnliche Gewerbebetriebe",
            Self::SlpGasHef => "Einfamilienhaushalt",
            Self::SlpGasHmf => "Mehrfamilienhaushalt",
            Self::SlpGasHko => "Kochgas",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&CustomerGroup::Rlm).unwrap(),
            r#""RLM""#
        );
        assert_eq!(
            serde_json::to_string(&CustomerGroup::SlpElectricityH0).unwrap(),
            r#""SLP_S_H0""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<CustomerGroup>(r#""RLM""#).unwrap(),
            CustomerGroup::Rlm
        );
        assert_eq!(
            serde_json::from_str::<CustomerGroup>(r#""SLP_G_HEF""#).unwrap(),
            CustomerGroup::SlpGasHef
        );
    }

    #[test]
    fn test_roundtrip() {
        for group in [
            CustomerGroup::Rlm,
            CustomerGroup::RlmMunicipal,
            CustomerGroup::SlpMunicipal,
            CustomerGroup::SlpElectricityG0,
            CustomerGroup::SlpElectricityH0,
            CustomerGroup::SlpGasStandard,
            CustomerGroup::SlpGasHef,
        ] {
            let json = serde_json::to_string(&group).unwrap();
            let parsed: CustomerGroup = serde_json::from_str(&json).unwrap();
            assert_eq!(group, parsed);
        }
    }
}
