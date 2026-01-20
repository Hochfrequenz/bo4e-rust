//! Tender (Ausschreibung) business object.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::com::TimePeriod;
use crate::enums::{Division, TenderStatus, TenderType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A tender/RFP (Request for Proposal) for energy supply.
///
/// German: Ausschreibung
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Tender;
/// use bo4e_core::enums::{Division, TenderStatus, TenderType};
///
/// let tender = Tender {
///     tender_number: Some("T-2024-001".to_string()),
///     tender_type: Some(TenderType::PublicLaw),
///     status: Some(TenderStatus::Phase2),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tender {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Tender number (Ausschreibungsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_number: Option<String>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Type of tender (Ausschreibungstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tender_type: Option<TenderType>,

    /// Status/phase of tender (Ausschreibungsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TenderStatus>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<Division>,

    /// Publication date (Veroeffentlichungsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<DateTime<Utc>>,

    /// Submission deadline (Abgabefrist)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_deadline: Option<DateTime<Utc>>,

    /// Delivery period (Lieferzeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_period: Option<TimePeriod>,

    /// Tendering party (Ausschreibender)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tendering_party: Option<Box<super::BusinessPartner>>,

    /// Estimated annual consumption in kWh (Jahresverbrauch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_annual_consumption: Option<f64>,

    /// Number of delivery points (Anzahl Lieferstellen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_delivery_points: Option<i32>,
}

impl Bo4eObject for Tender {
    fn type_name_german() -> &'static str {
        "Ausschreibung"
    }

    fn type_name_english() -> &'static str {
        "Tender"
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
    fn test_tender_creation() {
        let tender = Tender {
            tender_number: Some("T-2024-001".to_string()),
            tender_type: Some(TenderType::PublicLaw),
            status: Some(TenderStatus::Phase2),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(tender.status, Some(TenderStatus::Phase2));
    }

    #[test]
    fn test_europe_wide_tender() {
        let tender = Tender {
            tender_number: Some("T-EU-2024-001".to_string()),
            tender_type: Some(TenderType::EuropeWide),
            status: Some(TenderStatus::Phase1),
            estimated_annual_consumption: Some(10_000_000.0),
            number_of_delivery_points: Some(50),
            ..Default::default()
        };

        assert_eq!(tender.tender_type, Some(TenderType::EuropeWide));
        assert_eq!(tender.estimated_annual_consumption, Some(10_000_000.0));
    }

    #[test]
    fn test_serialize() {
        let tender = Tender {
            meta: Bo4eMeta::with_type("Ausschreibung"),
            tender_number: Some("T-123".to_string()),
            tender_type: Some(TenderType::PublicLaw),
            ..Default::default()
        };

        let json = serde_json::to_string(&tender).unwrap();
        assert!(json.contains(r#""tenderNumber":"T-123""#));
    }

    #[test]
    fn test_roundtrip() {
        let tender = Tender {
            meta: Bo4eMeta::with_type("Ausschreibung"),
            tender_number: Some("T-123".to_string()),
            description: Some("Test tender".to_string()),
            tender_type: Some(TenderType::PublicLaw),
            status: Some(TenderStatus::Phase2),
            division: Some(Division::Electricity),
            estimated_annual_consumption: Some(1_000_000.0),
            number_of_delivery_points: Some(10),
            ..Default::default()
        };

        let json = serde_json::to_string(&tender).unwrap();
        let parsed: Tender = serde_json::from_str(&json).unwrap();
        assert_eq!(tender, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Tender::type_name_german(), "Ausschreibung");
        assert_eq!(Tender::type_name_english(), "Tender");
    }
}
