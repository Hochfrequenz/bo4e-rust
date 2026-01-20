//! Country code (Landescode) enumeration.

use serde::{Deserialize, Serialize};

/// ISO 3166-1 alpha-2 country codes.
///
/// This enum contains the most commonly used country codes in the German energy market.
/// Additional codes can be added as needed.
///
/// German: Landescode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Landescode"))]
#[non_exhaustive]
pub enum Country {
    /// Germany (Deutschland)
    #[serde(rename = "DE")]
    Germany,

    /// Austria (Oesterreich)
    #[serde(rename = "AT")]
    Austria,

    /// Switzerland (Schweiz)
    #[serde(rename = "CH")]
    Switzerland,

    /// Netherlands (Niederlande)
    #[serde(rename = "NL")]
    Netherlands,

    /// Belgium (Belgien)
    #[serde(rename = "BE")]
    Belgium,

    /// France (Frankreich)
    #[serde(rename = "FR")]
    France,

    /// Luxembourg (Luxemburg)
    #[serde(rename = "LU")]
    Luxembourg,

    /// Poland (Polen)
    #[serde(rename = "PL")]
    Poland,

    /// Czech Republic (Tschechien)
    #[serde(rename = "CZ")]
    CzechRepublic,

    /// Denmark (Daenemark)
    #[serde(rename = "DK")]
    Denmark,

    /// Italy (Italien)
    #[serde(rename = "IT")]
    Italy,

    /// Spain (Spanien)
    #[serde(rename = "ES")]
    Spain,

    /// United Kingdom (Vereinigtes Koenigreich)
    #[serde(rename = "GB")]
    UnitedKingdom,

    /// Sweden (Schweden)
    #[serde(rename = "SE")]
    Sweden,

    /// Norway (Norwegen)
    #[serde(rename = "NO")]
    Norway,

    /// Finland (Finnland)
    #[serde(rename = "FI")]
    Finland,

    /// Portugal
    #[serde(rename = "PT")]
    Portugal,

    /// Greece (Griechenland)
    #[serde(rename = "GR")]
    Greece,

    /// Ireland (Irland)
    #[serde(rename = "IE")]
    Ireland,

    /// Hungary (Ungarn)
    #[serde(rename = "HU")]
    Hungary,

    /// Slovakia (Slowakei)
    #[serde(rename = "SK")]
    Slovakia,

    /// Slovenia (Slowenien)
    #[serde(rename = "SI")]
    Slovenia,

    /// Croatia (Kroatien)
    #[serde(rename = "HR")]
    Croatia,

    /// Romania (Rumaenien)
    #[serde(rename = "RO")]
    Romania,

    /// Bulgaria (Bulgarien)
    #[serde(rename = "BG")]
    Bulgaria,

    /// Estonia (Estland)
    #[serde(rename = "EE")]
    Estonia,

    /// Latvia (Lettland)
    #[serde(rename = "LV")]
    Latvia,

    /// Lithuania (Litauen)
    #[serde(rename = "LT")]
    Lithuania,

    /// Cyprus (Zypern)
    #[serde(rename = "CY")]
    Cyprus,

    /// Malta
    #[serde(rename = "MT")]
    Malta,

    /// Liechtenstein
    #[serde(rename = "LI")]
    Liechtenstein,

    /// Iceland (Island)
    #[serde(rename = "IS")]
    Iceland,
}

impl Country {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Germany => "Deutschland",
            Self::Austria => "Oesterreich",
            Self::Switzerland => "Schweiz",
            Self::Netherlands => "Niederlande",
            Self::Belgium => "Belgien",
            Self::France => "Frankreich",
            Self::Luxembourg => "Luxemburg",
            Self::Poland => "Polen",
            Self::CzechRepublic => "Tschechien",
            Self::Denmark => "Daenemark",
            Self::Italy => "Italien",
            Self::Spain => "Spanien",
            Self::UnitedKingdom => "Vereinigtes Koenigreich",
            Self::Sweden => "Schweden",
            Self::Norway => "Norwegen",
            Self::Finland => "Finnland",
            Self::Portugal => "Portugal",
            Self::Greece => "Griechenland",
            Self::Ireland => "Irland",
            Self::Hungary => "Ungarn",
            Self::Slovakia => "Slowakei",
            Self::Slovenia => "Slowenien",
            Self::Croatia => "Kroatien",
            Self::Romania => "Rumaenien",
            Self::Bulgaria => "Bulgarien",
            Self::Estonia => "Estland",
            Self::Latvia => "Lettland",
            Self::Lithuania => "Litauen",
            Self::Cyprus => "Zypern",
            Self::Malta => "Malta",
            Self::Liechtenstein => "Liechtenstein",
            Self::Iceland => "Island",
        }
    }

    /// Returns the ISO 3166-1 alpha-2 code as a string.
    pub fn alpha2_code(&self) -> &'static str {
        match self {
            Self::Germany => "DE",
            Self::Austria => "AT",
            Self::Switzerland => "CH",
            Self::Netherlands => "NL",
            Self::Belgium => "BE",
            Self::France => "FR",
            Self::Luxembourg => "LU",
            Self::Poland => "PL",
            Self::CzechRepublic => "CZ",
            Self::Denmark => "DK",
            Self::Italy => "IT",
            Self::Spain => "ES",
            Self::UnitedKingdom => "GB",
            Self::Sweden => "SE",
            Self::Norway => "NO",
            Self::Finland => "FI",
            Self::Portugal => "PT",
            Self::Greece => "GR",
            Self::Ireland => "IE",
            Self::Hungary => "HU",
            Self::Slovakia => "SK",
            Self::Slovenia => "SI",
            Self::Croatia => "HR",
            Self::Romania => "RO",
            Self::Bulgaria => "BG",
            Self::Estonia => "EE",
            Self::Latvia => "LV",
            Self::Lithuania => "LT",
            Self::Cyprus => "CY",
            Self::Malta => "MT",
            Self::Liechtenstein => "LI",
            Self::Iceland => "IS",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Country::Germany).unwrap(), r#""DE""#);
        assert_eq!(serde_json::to_string(&Country::Austria).unwrap(), r#""AT""#);
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Country>(r#""DE""#).unwrap(),
            Country::Germany
        );
        assert_eq!(
            serde_json::from_str::<Country>(r#""CH""#).unwrap(),
            Country::Switzerland
        );
    }

    #[test]
    fn test_roundtrip() {
        for country in [
            Country::Germany,
            Country::Austria,
            Country::Switzerland,
            Country::Netherlands,
            Country::Belgium,
            Country::France,
            Country::Luxembourg,
            Country::Poland,
        ] {
            let json = serde_json::to_string(&country).unwrap();
            let parsed: Country = serde_json::from_str(&json).unwrap();
            assert_eq!(country, parsed);
        }
    }

    #[test]
    fn test_alpha2_code() {
        assert_eq!(Country::Germany.alpha2_code(), "DE");
        assert_eq!(Country::Austria.alpha2_code(), "AT");
    }
}
