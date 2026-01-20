//! Business partner (Geschaeftspartner) business object.

use serde::{Deserialize, Serialize};

use crate::com::{Address, ContactMethod};
use crate::enums::BusinessPartnerRole;
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A business partner (company or organization).
///
/// German: Geschaeftspartner
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::BusinessPartner;
/// use bo4e_core::enums::BusinessPartnerRole;
///
/// let partner = BusinessPartner {
///     name1: Some("Stadtwerke Musterstadt GmbH".to_string()),
///     roles: vec![BusinessPartnerRole::Supplier],
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessPartner {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Partner ID (Geschaeftspartner-ID)
    #[serde(skip_serializing_if = "Option::is_none", alias = "geschaeftspartnerId")]
    pub partner_id: Option<String>,

    /// Company/organization name (Name1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name1: Option<String>,

    /// Additional name line (Name2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name2: Option<String>,

    /// Additional name line (Name3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name3: Option<String>,

    /// Roles this partner has (Rollen)
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "geschaeftspartnerrollen"
    )]
    pub roles: Vec<BusinessPartnerRole>,

    /// Primary address (Adresse)
    #[serde(skip_serializing_if = "Option::is_none", alias = "adresse")]
    pub address: Option<Address>,

    /// Contact methods (Kontaktwege)
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "kontaktwege")]
    pub contact_methods: Vec<ContactMethod>,

    /// Commercial register number (Handelsregisternummer)
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "handelsregisternummer"
    )]
    pub commercial_register_number: Option<String>,

    /// Tax ID (Steuernummer)
    #[serde(skip_serializing_if = "Option::is_none", alias = "steuernummer")]
    pub tax_id: Option<String>,

    /// VAT ID (Umsatzsteuer-ID)
    #[serde(skip_serializing_if = "Option::is_none", alias = "umsatzsteuerId")]
    pub vat_id: Option<String>,
}

impl Bo4eObject for BusinessPartner {
    fn type_name_german() -> &'static str {
        "Geschaeftspartner"
    }

    fn type_name_english() -> &'static str {
        "BusinessPartner"
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
    fn test_supplier_partner() {
        let partner = BusinessPartner {
            name1: Some("Stadtwerke Musterstadt GmbH".to_string()),
            roles: vec![BusinessPartnerRole::Supplier],
            ..Default::default()
        };

        assert!(partner.roles.contains(&BusinessPartnerRole::Supplier));
    }

    #[test]
    fn test_multiple_roles() {
        let partner = BusinessPartner {
            name1: Some("Multi-Utility AG".to_string()),
            roles: vec![
                BusinessPartnerRole::Supplier,
                BusinessPartnerRole::NetworkOperator,
            ],
            ..Default::default()
        };

        assert_eq!(partner.roles.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let partner = BusinessPartner {
            meta: Bo4eMeta::with_type("Geschaeftspartner"),
            name1: Some("Test GmbH".to_string()),
            vat_id: Some("DE123456789".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&partner).unwrap();
        assert!(json.contains(r#""name1":"Test GmbH""#));
        assert!(json.contains(r#""vatId":"DE123456789""#));
    }

    #[test]
    fn test_roundtrip() {
        let partner = BusinessPartner {
            meta: Bo4eMeta::with_type("Geschaeftspartner"),
            partner_id: Some("GP-001".to_string()),
            name1: Some("Test GmbH".to_string()),
            name2: Some("Abteilung Energie".to_string()),
            roles: vec![BusinessPartnerRole::Supplier],
            vat_id: Some("DE123456789".to_string()),
            tax_id: Some("12345/67890".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&partner).unwrap();
        let parsed: BusinessPartner = serde_json::from_str(&json).unwrap();
        assert_eq!(partner, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(BusinessPartner::type_name_german(), "Geschaeftspartner");
        assert_eq!(BusinessPartner::type_name_english(), "BusinessPartner");
    }
}
