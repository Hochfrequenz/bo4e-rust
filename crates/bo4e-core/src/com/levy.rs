//! Levy (Umlage) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A levy such as EEG, KWK, etc.
///
/// German: Umlage
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::Levy;
///
/// let levy = Levy {
///     description: Some("EEG-Umlage".to_string()),
///     value: Some(6.756),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Levy {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Description/name of the levy (Bezeichnung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Levy value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<Unit>,

    /// Legal reference (Gesetzliche Grundlage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_reference: Option<String>,

    /// Website for more information (Website)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl Bo4eObject for Levy {
    fn type_name_german() -> &'static str {
        "Umlage"
    }

    fn type_name_english() -> &'static str {
        "Levy"
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
    fn test_eeg_levy() {
        let levy = Levy {
            description: Some("EEG-Umlage".to_string()),
            value: Some(6.756),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            legal_reference: Some("§ 60 EEG".to_string()),
            ..Default::default()
        };

        assert_eq!(levy.description, Some("EEG-Umlage".to_string()));
        assert_eq!(levy.value, Some(6.756));
    }

    #[test]
    fn test_kwk_levy() {
        let levy = Levy {
            description: Some("KWK-Umlage".to_string()),
            value: Some(0.254),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        assert_eq!(levy.description, Some("KWK-Umlage".to_string()));
    }

    #[test]
    fn test_default() {
        let levy = Levy::default();
        assert!(levy.description.is_none());
        assert!(levy.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let levy = Levy {
            description: Some("§19 StromNEV-Umlage".to_string()),
            value: Some(0.437),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            legal_reference: Some("§ 19 StromNEV".to_string()),
            website: Some("https://www.netztransparenz.de".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&levy).unwrap();
        let parsed: Levy = serde_json::from_str(&json).unwrap();
        assert_eq!(levy, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(Levy::type_name_german(), "Umlage");
        assert_eq!(Levy::type_name_english(), "Levy");
    }
}
