//! Market location (Marktlokation) business object.
//!
//! Represents the point of energy delivery or receipt in the energy market.

use serde::{Deserialize, Serialize};

use crate::com::Address;
use crate::enums::{CustomerType, Division, EnergyDirection};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A market location (MaLo) - the point of energy delivery/receipt.
///
/// German: Marktlokation
///
/// A market location is the central business object representing
/// a point in the energy market where energy is delivered or received.
/// It has an 11-digit identifier.
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::MarketLocation;
/// use bo4e_core::enums::{Division, EnergyDirection};
///
/// let malo = MarketLocation {
///     market_location_id: Some("12345678901".to_string()),
///     division: Some(Division::Electricity),
///     energy_direction: Some(EnergyDirection::FeedOut),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketLocation {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Market location ID - 11 digits (Marktlokations-ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_location_id: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Energy direction (Energierichtung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_direction: Option<EnergyDirection>,

    /// Customer type (Kundentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_type: Option<CustomerType>,

    /// Location address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Supply start date (Lieferbeginn)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_start: Option<chrono::DateTime<chrono::Utc>>,

    /// Supply end date (Lieferende)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_end: Option<chrono::DateTime<chrono::Utc>>,

    /// Annual consumption in kWh (Jahresverbrauchsprognose)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_consumption: Option<f64>,

    /// Network operator code (Netzbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_operator_code: Option<String>,

    /// Basic supplier code (Grundversorger-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_supplier_code: Option<String>,

    /// Metering point operator code (Messstellenbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_operator_code: Option<String>,

    /// Transmission system operator code (Übertragungsnetzbetreiber-Codenummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_operator_code: Option<String>,

    /// Grid connection level (Netzebene)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_level: Option<String>,

    /// Network area (Netzgebiet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_area: Option<String>,

    /// Billing balance area (Bilanzierungsgebiet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_area: Option<String>,

    /// Associated metering location IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metering_location_ids: Vec<String>,

    /// Is Controllable Resource (Steuerbare Ressource)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controllable_resource: Option<bool>,
}

impl Bo4eObject for MarketLocation {
    fn type_name_german() -> &'static str {
        "Marktlokation"
    }

    fn type_name_english() -> &'static str {
        "MarketLocation"
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
    fn test_malo_id_format() {
        // MaLo IDs are 11 digits
        let malo = MarketLocation {
            market_location_id: Some("12345678901".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::FeedOut),
            ..Default::default()
        };

        assert_eq!(malo.market_location_id.as_ref().unwrap().len(), 11);
    }

    #[test]
    fn test_serialize() {
        let malo = MarketLocation {
            meta: Bo4eMeta::with_type("Marktlokation"),
            market_location_id: Some("12345678901".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&malo).unwrap();
        assert!(json.contains(r#""marketLocationId":"12345678901""#));
    }

    #[test]
    fn test_roundtrip() {
        let malo = MarketLocation {
            meta: Bo4eMeta::with_type("Marktlokation"),
            market_location_id: Some("12345678901".to_string()),
            division: Some(Division::Electricity),
            energy_direction: Some(EnergyDirection::FeedOut),
            annual_consumption: Some(3500.0),
            metering_location_ids: vec!["DE00012345".to_string()],
            ..Default::default()
        };

        let json = serde_json::to_string(&malo).unwrap();
        let parsed: MarketLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(malo, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MarketLocation::type_name_german(), "Marktlokation");
        assert_eq!(MarketLocation::type_name_english(), "MarketLocation");
    }
}
