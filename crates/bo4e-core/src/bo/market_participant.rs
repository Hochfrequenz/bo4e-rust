//! Market participant (Marktteilnehmer) business object.

use serde::{Deserialize, Serialize};

use crate::com::{Address, ContactMethod};
use crate::enums::{Division, MarketRole};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A market participant in the energy market.
///
/// German: Marktteilnehmer
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::MarketParticipant;
/// use bo4e_core::enums::{Division, MarketRole};
///
/// let participant = MarketParticipant {
///     market_partner_id: Some("9900000000001".to_string()),
///     market_role: Some(MarketRole::Supplier),
///     division: Some(Division::Electricity),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Marktteilnehmer"))]
#[serde(rename_all = "camelCase")]
pub struct MarketParticipant {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Market partner ID (Marktpartner-ID) - typically BDEW code number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktpartnerId"))]
    pub market_partner_id: Option<String>,

    /// Name of the market participant (Name)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "name"))]
    pub name: Option<String>,

    /// Market role (Marktrolle)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "marktrolle"))]
    pub market_role: Option<MarketRole>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Primary address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "adresse"))]
    pub address: Option<Address>,

    /// Contact methods (Kontaktwege)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kontaktwege"))]
    pub contact_methods: Vec<ContactMethod>,

    /// Associated business partner (Geschaeftspartner)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "geschaeftspartner"))]
    pub business_partner: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for MarketParticipant {
    fn type_name_german() -> &'static str {
        "Marktteilnehmer"
    }

    fn type_name_english() -> &'static str {
        "MarketParticipant"
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
    fn test_market_participant_creation() {
        let participant = MarketParticipant {
            market_partner_id: Some("9900000000001".to_string()),
            market_role: Some(MarketRole::Supplier),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(
            participant.market_partner_id,
            Some("9900000000001".to_string())
        );
        assert_eq!(participant.market_role, Some(MarketRole::Supplier));
    }

    #[test]
    fn test_network_operator() {
        let participant = MarketParticipant {
            market_partner_id: Some("9900000000002".to_string()),
            name: Some("Verteilnetz GmbH".to_string()),
            market_role: Some(MarketRole::NetworkOperator),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        assert_eq!(participant.market_role, Some(MarketRole::NetworkOperator));
    }

    #[test]
    fn test_serialize() {
        let participant = MarketParticipant {
            meta: Bo4eMeta::with_type("Marktteilnehmer"),
            market_partner_id: Some("9900000000001".to_string()),
            name: Some("Test Lieferant GmbH".to_string()),
            market_role: Some(MarketRole::Supplier),
            ..Default::default()
        };

        let json = serde_json::to_string(&participant).unwrap();
        assert!(json.contains(r#""marketPartnerId":"9900000000001""#));
        assert!(json.contains(r#""name":"Test Lieferant GmbH""#));
    }

    #[test]
    fn test_roundtrip() {
        let participant = MarketParticipant {
            meta: Bo4eMeta::with_type("Marktteilnehmer"),
            market_partner_id: Some("9900000000001".to_string()),
            name: Some("Test Lieferant GmbH".to_string()),
            market_role: Some(MarketRole::Supplier),
            division: Some(Division::Electricity),
            ..Default::default()
        };

        let json = serde_json::to_string(&participant).unwrap();
        let parsed: MarketParticipant = serde_json::from_str(&json).unwrap();
        assert_eq!(participant, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(MarketParticipant::type_name_german(), "Marktteilnehmer");
        assert_eq!(MarketParticipant::type_name_english(), "MarketParticipant");
    }
}
