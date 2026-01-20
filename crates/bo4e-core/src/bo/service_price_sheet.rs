//! Service price sheet (PreisblattDienstleistung) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::{ServicePrice, TimePeriod};
use crate::enums::{Division, ServiceType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price sheet for services (e.g., metering, billing).
///
/// German: PreisblattDienstleistung
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::ServicePriceSheet;
/// use bo4e_core::enums::{Division, ServiceType};
///
/// let price_sheet = ServicePriceSheet {
///     designation: Some("Dienstleistungspreisblatt 2024".to_string()),
///     division: Some(Division::Electricity),
///     service_type: Some(ServiceType::RemoteReading),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePriceSheet {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Name/designation of the price sheet (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designation: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Type of service (Dienstleistungsart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,

    /// Price sheet number/identifier (Preisblattnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_sheet_number: Option<String>,

    /// Validity period (Gueltigkeitszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<TimePeriod>,

    /// Valid from date (Gueltig ab)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    /// Valid until date (Gueltig bis)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,

    /// Service prices (Dienstleistungspreise)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prices: Vec<ServicePrice>,

    /// Service provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for ServicePriceSheet {
    fn type_name_german() -> &'static str {
        "PreisblattDienstleistung"
    }

    fn type_name_english() -> &'static str {
        "ServicePriceSheet"
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
    fn test_service_price_sheet_creation() {
        let price_sheet = ServicePriceSheet {
            designation: Some("Dienstleistungspreisblatt 2024".to_string()),
            division: Some(Division::Electricity),
            service_type: Some(ServiceType::RemoteReading),
            price_sheet_number: Some("DL-2024-001".to_string()),
            ..Default::default()
        };

        assert_eq!(price_sheet.service_type, Some(ServiceType::RemoteReading));
    }

    #[test]
    fn test_serialize() {
        let price_sheet = ServicePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattDienstleistung"),
            designation: Some("Service Prices".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        assert!(json.contains(r#""designation":"Service Prices""#));
        assert!(json.contains(r#""_typ":"PreisblattDienstleistung""#));
    }

    #[test]
    fn test_roundtrip() {
        let price_sheet = ServicePriceSheet {
            meta: Bo4eMeta::with_type("PreisblattDienstleistung"),
            designation: Some("Billing Services".to_string()),
            description: Some("Standard billing service prices".to_string()),
            division: Some(Division::Electricity),
            price_sheet_number: Some("BS-2024".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price_sheet).unwrap();
        let parsed: ServicePriceSheet = serde_json::from_str(&json).unwrap();
        assert_eq!(price_sheet, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(
            ServicePriceSheet::type_name_german(),
            "PreisblattDienstleistung"
        );
        assert_eq!(ServicePriceSheet::type_name_english(), "ServicePriceSheet");
    }
}
