//! Concession fee (Konzessionsabgabe) component.

use serde::{Deserialize, Serialize};

use crate::enums::{ConcessionFeeCustomerGroup, ConcessionFeeType, Currency, Unit};
use crate::traits::{Bo4eMeta, Bo4eObject};

/// A concession fee charged by municipalities.
///
/// German: Konzessionsabgabe
///
/// # Example
///
/// ```rust
/// use bo4e_core::com::ConcessionFee;
/// use bo4e_core::enums::{ConcessionFeeType, ConcessionFeeCustomerGroup};
///
/// let fee = ConcessionFee {
///     fee_type: Some(ConcessionFeeType::TariffCustomer),
///     customer_group: Some(ConcessionFeeCustomerGroup::ElectricityTariff25000),
///     value: Some(1.59),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Konzessionsabgabe"))]
#[serde(rename_all = "camelCase")]
pub struct ConcessionFee {
    /// BO4E metadata
    #[serde(flatten)]
    pub meta: Bo4eMeta,

    /// Type of concession fee (Konzessionsabgabentyp)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "konzessionsabgabentyp"))]
    pub fee_type: Option<ConcessionFeeType>,

    /// Customer group for the fee (Kundengruppe KA)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "kundengruppeKA"))]
    pub customer_group: Option<ConcessionFeeCustomerGroup>,

    /// Fee value (Wert)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "wert"))]
    pub value: Option<f64>,

    /// Currency (Waehrung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "waehrung"))]
    pub currency: Option<Currency>,

    /// Reference unit (Bezugseinheit)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "bezugseinheit"))]
    pub reference_unit: Option<Unit>,

    /// Description (Beschreibung)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "beschreibung"))]
    pub description: Option<String>,
}

impl Bo4eObject for ConcessionFee {
    fn type_name_german() -> &'static str {
        "Konzessionsabgabe"
    }

    fn type_name_english() -> &'static str {
        "ConcessionFee"
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
    fn test_household_concession_fee() {
        let fee = ConcessionFee {
            fee_type: Some(ConcessionFeeType::TariffCustomer),
            customer_group: Some(ConcessionFeeCustomerGroup::ElectricityTariff25000),
            value: Some(1.59),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        assert_eq!(fee.fee_type, Some(ConcessionFeeType::TariffCustomer));
        assert_eq!(fee.value, Some(1.59));
    }

    #[test]
    fn test_business_concession_fee() {
        let fee = ConcessionFee {
            fee_type: Some(ConcessionFeeType::SpecialContractCustomer),
            customer_group: Some(ConcessionFeeCustomerGroup::ElectricitySpecialCustomer),
            value: Some(0.11),
            currency: Some(Currency::Eur),
            reference_unit: Some(Unit::KilowattHour),
            ..Default::default()
        };

        assert_eq!(
            fee.customer_group,
            Some(ConcessionFeeCustomerGroup::ElectricitySpecialCustomer)
        );
    }

    #[test]
    fn test_default() {
        let fee = ConcessionFee::default();
        assert!(fee.fee_type.is_none());
        assert!(fee.value.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let fee = ConcessionFee {
            fee_type: Some(ConcessionFeeType::TariffCustomer),
            customer_group: Some(ConcessionFeeCustomerGroup::ElectricityTariff25000),
            value: Some(1.59),
            description: Some("Konzessionsabgabe Strom".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&fee).unwrap();
        let parsed: ConcessionFee = serde_json::from_str(&json).unwrap();
        assert_eq!(fee, parsed);
    }

    #[test]
    fn test_bo4e_object_impl() {
        assert_eq!(ConcessionFee::type_name_german(), "Konzessionsabgabe");
        assert_eq!(ConcessionFee::type_name_english(), "ConcessionFee");
    }
}
