//! Service price (Dienstleistungspreis) component.

use serde::{Deserialize, Serialize};

use crate::enums::{Currency, ServiceType, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A price for a service.
///
/// German: Dienstleistungspreis
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ServicePrice;
/// use bo4e_core::enums::ServiceType;
///
/// let price = ServicePrice {
///     service_type: Some(ServiceType::Disconnection),
///     value: Some(75.0),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePrice {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of service (Dienstleistungstyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,

    /// Price value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_unit: Option<Unit>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Bo4eObject for ServicePrice {
    fn type_name_german() -> &'static str {
        "Dienstleistungspreis"
    }

    fn type_name_english() -> &'static str {
        "ServicePrice"
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
    fn test_installation_service() {
        let price = ServicePrice {
            service_type: Some(ServiceType::Disconnection),
            value: Some(75.0),
            currency: Some(Currency::Eur),
            description: Some("Zählerinstallation".to_string()),
            ..Default::default()
        };

        assert_eq!(price.service_type, Some(ServiceType::Disconnection));
        assert_eq!(price.value, Some(75.0));
    }

    #[test]
    fn test_default() {
        let price = ServicePrice::default();
        assert!(price.service_type.is_none());
        assert!(price.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let price = ServicePrice {
            service_type: Some(ServiceType::ManualReadingMonthly),
            value: Some(50.0),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::Hour),
            description: Some("Energieberatung".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&price).unwrap();
        let parsed: ServicePrice = serde_json::from_str(&json).unwrap();
        assert_eq!(price, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ServicePrice::type_name_german(), "Dienstleistungspreis");
        assert_eq!(ServicePrice::type_name_english(), "ServicePrice");
    }
}
