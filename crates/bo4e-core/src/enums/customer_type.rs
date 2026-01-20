//! Customer type (Kundentyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of customer based on consumption characteristics.
///
/// German: Kundentyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CustomerType {
    /// Commercial/business customers (Gewerbe)
    #[serde(rename = "GEWERBE")]
    Commercial,

    /// Private households (Privat)
    #[serde(rename = "PRIVAT")]
    Private,

    /// Farmers (Landwirt)
    #[serde(rename = "LANDWIRT")]
    Farmer,

    /// Other end customers (Sonstige)
    #[serde(rename = "SONSTIGE")]
    Other,

    /// Household customers (Haushalt)
    #[serde(rename = "HAUSHALT")]
    Household,

    /// Direct heating (Direktheizung)
    #[serde(rename = "DIREKTHEIZUNG")]
    DirectHeating,

    /// Common facilities of multi-family houses (Gemeinschaft MFH)
    #[serde(rename = "GEMEINSCHAFT_MFH")]
    CommonFacilitiesMfh,

    /// Churches and charitable institutions (Kirche)
    #[serde(rename = "KIRCHE")]
    Church,

    /// Combined heat and power plants (KWK-Anlagen)
    #[serde(rename = "KWK")]
    Chp,

    /// Charging stations (Ladesaeule)
    #[serde(rename = "LADESAEULE")]
    ChargingStation,

    /// Public lighting (Oeffentliche Beleuchtung)
    #[serde(rename = "BELEUCHTUNG_OEFFENTLICH")]
    PublicLighting,

    /// Street lighting (Strassenbeleuchtung)
    #[serde(rename = "BELEUCHTUNG_STRASSE")]
    StreetLighting,

    /// Storage heating (Speicherheizung)
    #[serde(rename = "SPEICHERHEIZUNG")]
    StorageHeating,

    /// Interruptible consumption devices (Unterbrechbare Einrichtung)
    #[serde(rename = "UNTERBR_EINRICHTUNG")]
    InterruptibleDevice,

    /// Heat pumps (Waermepumpe)
    #[serde(rename = "WAERMEPUMPE")]
    HeatPump,
}

impl CustomerType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Commercial => "Gewerbe",
            Self::Private => "Privat",
            Self::Farmer => "Landwirt",
            Self::Other => "Sonstige",
            Self::Household => "Haushalt",
            Self::DirectHeating => "Direktheizung",
            Self::CommonFacilitiesMfh => "Gemeinschaft MFH",
            Self::Church => "Kirche",
            Self::Chp => "KWK-Anlagen",
            Self::ChargingStation => "Ladesaeule",
            Self::PublicLighting => "Oeffentliche Beleuchtung",
            Self::StreetLighting => "Strassenbeleuchtung",
            Self::StorageHeating => "Speicherheizung",
            Self::InterruptibleDevice => "Unterbrechbare Einrichtung",
            Self::HeatPump => "Waermepumpe",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&CustomerType::Private).unwrap(),
            r#""PRIVAT""#
        );
        assert_eq!(
            serde_json::to_string(&CustomerType::Commercial).unwrap(),
            r#""GEWERBE""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<CustomerType>(r#""PRIVAT""#).unwrap(),
            CustomerType::Private
        );
        assert_eq!(
            serde_json::from_str::<CustomerType>(r#""WAERMEPUMPE""#).unwrap(),
            CustomerType::HeatPump
        );
    }

    #[test]
    fn test_roundtrip() {
        for customer_type in [
            CustomerType::Commercial,
            CustomerType::Private,
            CustomerType::Farmer,
            CustomerType::Other,
            CustomerType::Household,
            CustomerType::DirectHeating,
            CustomerType::CommonFacilitiesMfh,
            CustomerType::Church,
            CustomerType::Chp,
            CustomerType::ChargingStation,
            CustomerType::PublicLighting,
            CustomerType::StreetLighting,
            CustomerType::StorageHeating,
            CustomerType::InterruptibleDevice,
            CustomerType::HeatPump,
        ] {
            let json = serde_json::to_string(&customer_type).unwrap();
            let parsed: CustomerType = serde_json::from_str(&json).unwrap();
            assert_eq!(customer_type, parsed);
        }
    }
}
