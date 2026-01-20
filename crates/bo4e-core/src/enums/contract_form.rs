//! Contract form (Vertragsform) enumeration.

use serde::{Deserialize, Serialize};

/// Form of contract in tenders and offers.
///
/// German: Vertragsform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContractForm {
    /// Online contract
    #[serde(rename = "ONLINE")]
    Online,

    /// Direct contract
    #[serde(rename = "DIREKT")]
    Direct,

    /// Fax contract
    #[serde(rename = "FAX")]
    Fax,
}

impl ContractForm {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Online => "Online",
            Self::Direct => "Direkt",
            Self::Fax => "Auftragsfax",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ContractForm::Online).unwrap(),
            r#""ONLINE""#
        );
        assert_eq!(
            serde_json::to_string(&ContractForm::Direct).unwrap(),
            r#""DIREKT""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<ContractForm>(r#""ONLINE""#).unwrap(),
            ContractForm::Online
        );
        assert_eq!(
            serde_json::from_str::<ContractForm>(r#""FAX""#).unwrap(),
            ContractForm::Fax
        );
    }

    #[test]
    fn test_roundtrip() {
        for form in [ContractForm::Online, ContractForm::Direct, ContractForm::Fax] {
            let json = serde_json::to_string(&form).unwrap();
            let parsed: ContractForm = serde_json::from_str(&json).unwrap();
            assert_eq!(form, parsed);
        }
    }
}
