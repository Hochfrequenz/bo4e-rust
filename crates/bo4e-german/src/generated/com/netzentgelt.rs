#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Netzentgelt {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceType")]
    pub preistyp: Option<crate::Preistyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "referenceUnit")]
    pub bezugseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkOperatorCode")]
    pub netzbetreiber: Option<String>,
}
impl From<bo4e_core::com::NetworkCharge> for Netzentgelt {
    fn from(v: bo4e_core::com::NetworkCharge) -> Self {
        Self {
            meta: v.meta,
            preistyp: v.price_type.map(Into::into),
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bezugseinheit: v.reference_unit.map(Into::into),
            beschreibung: v.description,
            netzbetreiber: v.network_operator_code,
        }
    }
}
impl From<Netzentgelt> for bo4e_core::com::NetworkCharge {
    fn from(v: Netzentgelt) -> Self {
        Self {
            meta: v.meta,
            price_type: v.preistyp.map(Into::into),
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            reference_unit: v.bezugseinheit.map(Into::into),
            description: v.beschreibung,
            network_operator_code: v.netzbetreiber,
        }
    }
}
