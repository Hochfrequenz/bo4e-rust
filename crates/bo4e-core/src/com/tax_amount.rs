//! Tax amount (Steuerbetrag) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, TaxType};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A calculated tax amount.
///
/// German: Steuerbetrag
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::TaxAmount;
/// use bo4e_core::enums::{Currency, TaxType};
///
/// let tax = TaxAmount {
///     tax_type: Some(TaxType::Ust19),
///     tax_rate: Some(19.0),
///     basis_value: Some(100.0),
///     tax_value: Some(19.0),
///     currency: Some(Currency::Euro),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxAmount {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of tax (Steuerart)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxType>,

    /// Tax rate as percentage (Steuersatz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<f64>,

    /// Net amount on which tax was calculated (Basiswert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_value: Option<f64>,

    /// Calculated tax amount (Steuerwert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_value: Option<f64>,

    /// Currency (Waehrungscode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
}

impl Bo4eObject for TaxAmount {
    fn type_name_german() -> &'static str {
        "Steuerbetrag"
    }

    fn type_name_english() -> &'static str {
        "TaxAmount"
    }

    fn meta(&self) -> &Bo4eMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Bo4eMeta {
        &mut self.meta
    }
}

impl TaxAmount {
    /// Calculate VAT 19% on a net amount.
    pub fn vat_19(net_amount: f64) -> Self {
        Self {
            tax_type: Some(TaxType::Ust19),
            tax_rate: Some(19.0),
            basis_value: Some(net_amount),
            tax_value: Some(net_amount * 0.19),
            currency: Some(Currency::Euro),
            ..Default::default()
        }
    }

    /// Calculate VAT 7% on a net amount.
    pub fn vat_7(net_amount: f64) -> Self {
        Self {
            tax_type: Some(TaxType::Ust7),
            tax_rate: Some(7.0),
            basis_value: Some(net_amount),
            tax_value: Some(net_amount * 0.07),
            currency: Some(Currency::Euro),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vat_19() {
        let tax = TaxAmount::vat_19(100.0);
        assert_eq!(tax.tax_type, Some(TaxType::Ust19));
        assert_eq!(tax.tax_rate, Some(19.0));
        assert_eq!(tax.basis_value, Some(100.0));
        assert_eq!(tax.tax_value, Some(19.0));
    }

    #[test]
    fn test_vat_7() {
        let tax = TaxAmount::vat_7(100.0);
        assert_eq!(tax.tax_type, Some(TaxType::Ust7));
        assert_eq!(tax.tax_rate, Some(7.0));
        assert_eq!(tax.basis_value, Some(100.0));
        assert_eq!(tax.tax_value, Some(7.0));
    }

    #[test]
    fn test_default() {
        let tax = TaxAmount::default();
        assert!(tax.tax_type.is_none());
        assert!(tax.tax_rate.is_none());
        assert!(tax.basis_value.is_none());
        assert!(tax.tax_value.is_none());
    }

    #[test]
    fn test_serialize() {
        let tax = TaxAmount::vat_19(250.0);
        let json = serde_json::to_string(&tax).unwrap();
        assert!(json.contains(r#""taxRate":19"#));
        assert!(json.contains(r#""basisValue":250"#));
    }

    #[test]
    fn test_roundtrip() {
        let tax = TaxAmount {
            tax_type: Some(TaxType::Ust19),
            tax_rate: Some(19.0),
            basis_value: Some(123.45),
            tax_value: Some(23.4555),
            currency: Some(Currency::Euro),
            ..Default::default()
        };

        let json = serde_json::to_string(&tax).unwrap();
        let parsed: TaxAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(tax, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(TaxAmount::type_name_german(), "Steuerbetrag");
        assert_eq!(TaxAmount::type_name_english(), "TaxAmount");
    }
}
