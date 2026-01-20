//! Meter (Zaehler) business object.
//!
//! Represents a metering device for measuring energy consumption or production.

use serde::{Deserialize, Serialize};

use crate::com::{Address, Hardware, MeterRegister};
use crate::enums::{Division, MeterSize, MeterType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A meter (Zähler) for measuring energy consumption or production.
///
/// German: Zaehler
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Meter;
/// use bo4e_core::enums::{Division, MeterType};
///
/// let meter = Meter {
///     meter_number: Some("1EMH0012345678".to_string()),
///     division: Some(Division::Electricity),
///     meter_type: Some(MeterType::ModernMeasuringDevice),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Unique meter identification number (Zaehlernummer)
    #[serde(skip_serializing_if = "Option::is_none", alias = "zaehlernummer")]
    pub meter_number: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none", alias = "sparte")]
    pub division: Option<Division>,

    /// Type of meter (Zaehlertyp)
    #[serde(skip_serializing_if = "Option::is_none", alias = "zaehlertyp")]
    pub meter_type: Option<MeterType>,

    /// Meter size classification (Zaehlergroesse)
    #[serde(skip_serializing_if = "Option::is_none", alias = "zaehlergroesse")]
    pub meter_size: Option<MeterSize>,

    /// Installation location address (Standort)
    #[serde(skip_serializing_if = "Option::is_none", alias = "standort")]
    pub location: Option<Address>,

    /// Registers on this meter (Zaehlwerke)
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "zaehlwerke")]
    pub registers: Vec<MeterRegister>,

    /// Hardware components (Geraeteeigenschaften)
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "geraeteeigenschaften"
    )]
    pub hardware: Vec<Hardware>,

    /// Reference to associated market location ID (Marktlokation)
    #[serde(skip_serializing_if = "Option::is_none", alias = "marktlokationsId")]
    pub market_location_id: Option<String>,

    /// Reference to associated metering location ID (Messlokation)
    #[serde(skip_serializing_if = "Option::is_none", alias = "messlokationsId")]
    pub metering_location_id: Option<String>,

    /// Ownership status (Eigentumsverhaeltnis)
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "eigentumsverhaeltnis"
    )]
    pub ownership: Option<String>,

    /// Manufacturer (Hersteller)
    #[serde(skip_serializing_if = "Option::is_none", alias = "hersteller")]
    pub manufacturer: Option<String>,

    /// Manufacturing year (Herstellungsjahr)
    #[serde(skip_serializing_if = "Option::is_none", alias = "herstellungsjahr")]
    pub manufacturing_year: Option<i32>,

    /// Installation date (Einbaudatum)
    #[serde(skip_serializing_if = "Option::is_none", alias = "einbaudatum")]
    pub installation_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Removal date (Ausbaudatum)
    #[serde(skip_serializing_if = "Option::is_none", alias = "ausbaudatum")]
    pub removal_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Calibration date (Eichdatum)
    #[serde(skip_serializing_if = "Option::is_none", alias = "eichdatum")]
    pub calibration_date: Option<chrono::DateTime<chrono::Utc>>,

    /// Calibration expiry date (Eichablaufdatum)
    #[serde(skip_serializing_if = "Option::is_none", alias = "eichablaufdatum")]
    pub calibration_expiry_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl Bo4eObject for Meter {
    fn type_name_german() -> &'static str {
        "Zaehler"
    }

    fn type_name_english() -> &'static str {
        "Meter"
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
    fn test_meter_creation() {
        let meter = Meter {
            meter_number: Some("1EMH0012345678".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(meter.meter_number, Some("1EMH0012345678".to_string()));
        assert_eq!(meter.division, Some(Division::Electricity));
    }

    #[test]
    fn test_meter_with_registers() {
        let register = MeterRegister {
            obis_code: Some("1-0:1.8.0".to_string()),
            ..Default::default()
        };

        let meter = Meter {
            meter_number: Some("TEST123".to_string()),
            registers: vec![register],
            ..Default::default()
        };

        assert_eq!(meter.registers.len(), 1);
    }

    #[test]
    fn test_serialize() {
        let meter = Meter {
            meta: Bo4eMeta::with_type("Zaehler"),
            meter_number: Some("1EMH0012345678".to_string()),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&meter).unwrap();
        assert!(json.contains(r#""_typ":"Zaehler""#));
        assert!(json.contains(r#""meterNumber":"1EMH0012345678""#));
    }

    #[test]
    fn test_roundtrip() {
        let meter = Meter {
            meta: Bo4eMeta::with_type("Zaehler"),
            meter_number: Some("TEST123".to_string()),
            division: Some(Division::Electricity),
            meter_type: Some(MeterType::ModernMeasuringDevice),
            meter_size: Some(MeterSize::G4),
            manufacturer: Some("Acme Corp".to_string()),
            manufacturing_year: Some(2023),
            ..Default::default()
        };

        let json = serde_json::to_string(&meter).unwrap();
        let parsed: Meter = serde_json::from_str(&json).unwrap();
        assert_eq!(meter, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Meter::type_name_german(), "Zaehler");
        assert_eq!(Meter::type_name_english(), "Meter");
    }
}
