//! Invoice status (Rechnungsstatus) enumeration.

use serde::{Deserialize, Serialize};

/// Status of an invoice in the processing lifecycle.
///
/// German: Rechnungsstatus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Rechnungsstatus"))]
#[non_exhaustive]
pub enum InvoiceStatus {
    /// Unchecked - invoice created/received but not yet verified (Ungeprueft)
    #[serde(rename = "UNGEPRUEFT")]
    Unchecked,

    /// Checked OK - invoice verified and found correct (Geprueft OK)
    #[serde(rename = "GEPRUEFT_OK")]
    CheckedOk,

    /// Checked with errors - invoice has errors (Geprueft fehlerhaft)
    #[serde(rename = "GEPRUEFT_FEHLERHAFT")]
    CheckedWithErrors,

    /// Booked - invoice recorded in accounting (Gebucht)
    #[serde(rename = "GEBUCHT")]
    Booked,

    /// Paid - invoice has been settled (Bezahlt)
    #[serde(rename = "BEZAHLT")]
    Paid,
}

impl InvoiceStatus {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Unchecked => "Ungeprueft",
            Self::CheckedOk => "Geprueft OK",
            Self::CheckedWithErrors => "Geprueft fehlerhaft",
            Self::Booked => "Gebucht",
            Self::Paid => "Bezahlt",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&InvoiceStatus::Unchecked).unwrap(),
            r#""UNGEPRUEFT""#
        );
        assert_eq!(
            serde_json::to_string(&InvoiceStatus::Paid).unwrap(),
            r#""BEZAHLT""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<InvoiceStatus>(r#""UNGEPRUEFT""#).unwrap(),
            InvoiceStatus::Unchecked
        );
        assert_eq!(
            serde_json::from_str::<InvoiceStatus>(r#""GEPRUEFT_OK""#).unwrap(),
            InvoiceStatus::CheckedOk
        );
    }

    #[test]
    fn test_roundtrip() {
        for status in [
            InvoiceStatus::Unchecked,
            InvoiceStatus::CheckedOk,
            InvoiceStatus::CheckedWithErrors,
            InvoiceStatus::Booked,
            InvoiceStatus::Paid,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: InvoiceStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }
}
