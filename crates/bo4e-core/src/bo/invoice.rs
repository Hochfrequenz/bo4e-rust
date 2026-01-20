//! Invoice (Rechnung) business object.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::com::{Amount, InvoicePosition, TimePeriod};
use crate::enums::{Division, InvoiceStatus, InvoiceType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// An invoice for energy services.
///
/// German: Rechnung
///
/// # Example
///
/// ```rust
/// use bo4e_core::bo::Invoice;
/// use bo4e_core::com::Amount;
/// use bo4e_core::enums::{InvoiceStatus, InvoiceType};
///
/// let invoice = Invoice {
///     invoice_number: Some("RE-2024-001234".to_string()),
///     invoice_type: Some(InvoiceType::EndCustomerInvoice),
///     status: Some(InvoiceStatus::CheckedOk),
///     gross_amount: Some(Amount::eur(1190.00)),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Rechnung"))]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Invoice number (Rechnungsnummer)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "rechnungsnummer"))]
    pub invoice_number: Option<String>,

    /// Invoice type (Rechnungstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "rechnungstyp"))]
    pub invoice_type: Option<InvoiceType>,

    /// Invoice status (Rechnungsstatus)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "rechnungsstatus"))]
    pub status: Option<InvoiceStatus>,

    /// Energy division (Sparte)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "sparte"))]
    pub division: Option<Division>,

    /// Invoice date (Rechnungsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "rechnungsdatum"))]
    pub invoice_date: Option<NaiveDate>,

    /// Due date (Faelligkeitsdatum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "faelligkeitsdatum"))]
    pub due_date: Option<NaiveDate>,

    /// Billing period (Abrechnungszeitraum)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "abrechnungszeitraum"))]
    pub billing_period: Option<TimePeriod>,

    /// Net amount (Nettobetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "nettobetrag"))]
    pub net_amount: Option<Amount>,

    /// Tax amount (Steuerbetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "steuerbetrag"))]
    pub tax_amount: Option<Amount>,

    /// Gross amount (Bruttobetrag)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bruttobetrag"))]
    pub gross_amount: Option<Amount>,

    /// Invoice line items (Rechnungspositionen)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "rechnungspositionen"))]
    pub positions: Vec<InvoicePosition>,

    /// Invoice recipient (Rechnungsempfaenger)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "rechnungsempfaenger"))]
    pub recipient: Option<Box<super::BusinessPartner>>,
}

impl Bo4eObject for Invoice {
    fn type_name_german() -> &'static str {
        "Rechnung"
    }

    fn type_name_english() -> &'static str {
        "Invoice"
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
    fn test_invoice_creation() {
        let invoice = Invoice {
            invoice_number: Some("RE-2024-001234".to_string()),
            invoice_type: Some(InvoiceType::EndCustomerInvoice),
            status: Some(InvoiceStatus::CheckedOk),
            net_amount: Some(Amount::eur(1000.00)),
            tax_amount: Some(Amount::eur(190.00)),
            gross_amount: Some(Amount::eur(1190.00)),
            ..Default::default()
        };

        assert_eq!(invoice.invoice_number, Some("RE-2024-001234".to_string()));
        assert_eq!(invoice.status, Some(InvoiceStatus::CheckedOk));
    }

    #[test]
    fn test_invoice_with_positions() {
        let invoice = Invoice {
            invoice_number: Some("RE-001".to_string()),
            positions: vec![
                InvoicePosition {
                    position_number: Some(1),
                    position_text: Some("Electricity consumption".to_string()),
                    total_price_value: Some(500.0),
                    ..Default::default()
                },
                InvoicePosition {
                    position_number: Some(2),
                    position_text: Some("Network fees".to_string()),
                    total_price_value: Some(100.0),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        assert_eq!(invoice.positions.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let invoice = Invoice {
            meta: Bo4eMeta::with_type("Rechnung"),
            invoice_number: Some("RE-001".to_string()),
            gross_amount: Some(Amount::eur(119.00)),
            ..Default::default()
        };

        let json = serde_json::to_string(&invoice).unwrap();
        assert!(json.contains(r#""invoiceNumber":"RE-001""#));
        assert!(json.contains(r#""_typ":"Rechnung""#));
    }

    #[test]
    fn test_roundtrip() {
        let invoice = Invoice {
            meta: Bo4eMeta::with_type("Rechnung"),
            invoice_number: Some("RE-123".to_string()),
            invoice_type: Some(InvoiceType::MonthlyInvoice),
            status: Some(InvoiceStatus::Paid),
            division: Some(Division::Electricity),
            net_amount: Some(Amount::eur(100.0)),
            ..Default::default()
        };

        let json = serde_json::to_string(&invoice).unwrap();
        let parsed: Invoice = serde_json::from_str(&json).unwrap();
        assert_eq!(invoice, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Invoice::type_name_german(), "Rechnung");
        assert_eq!(Invoice::type_name_english(), "Invoice");
    }
}
