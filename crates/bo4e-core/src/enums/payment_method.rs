//! Payment method (Zahlungsart) enumeration.

use serde::{Deserialize, Serialize};

/// Method of payment.
///
/// German: Zahlungsart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PaymentMethod {
    /// SEPA direct debit (SEPA Lastschrift)
    #[serde(rename = "SEPA_LASTSCHRIFT")]
    SepaDirectDebit,

    /// Bank transfer (Ueberweisung)
    #[serde(rename = "UEBERWEISUNG")]
    BankTransfer,
}

impl PaymentMethod {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::SepaDirectDebit => "SEPA Lastschrift",
            Self::BankTransfer => "Ueberweisung",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&PaymentMethod::SepaDirectDebit).unwrap(),
            r#""SEPA_LASTSCHRIFT""#
        );
        assert_eq!(
            serde_json::to_string(&PaymentMethod::BankTransfer).unwrap(),
            r#""UEBERWEISUNG""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<PaymentMethod>(r#""SEPA_LASTSCHRIFT""#).unwrap(),
            PaymentMethod::SepaDirectDebit
        );
        assert_eq!(
            serde_json::from_str::<PaymentMethod>(r#""UEBERWEISUNG""#).unwrap(),
            PaymentMethod::BankTransfer
        );
    }

    #[test]
    fn test_roundtrip() {
        for method in [PaymentMethod::SepaDirectDebit, PaymentMethod::BankTransfer] {
            let json = serde_json::to_string(&method).unwrap();
            let parsed: PaymentMethod = serde_json::from_str(&json).unwrap();
            assert_eq!(method, parsed);
        }
    }
}
