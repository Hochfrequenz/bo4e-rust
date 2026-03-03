#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preis {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "referenceUnit")]
    pub bezugswert: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceType")]
    pub preistyp: Option<crate::Preistyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "status")]
    pub preisstatus: Option<crate::Preisstatus>,
}
impl From<bo4e_core::com::Price> for Preis {
    fn from(v: bo4e_core::com::Price) -> Self {
        Self {
            meta: v.meta,
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bezugswert: v.reference_unit.map(Into::into),
            preistyp: v.price_type.map(Into::into),
            preisstatus: v.status.map(Into::into),
        }
    }
}
impl From<Preis> for bo4e_core::com::Price {
    fn from(v: Preis) -> Self {
        Self {
            meta: v.meta,
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            reference_unit: v.bezugswert.map(Into::into),
            price_type: v.preistyp.map(Into::into),
            status: v.preisstatus.map(Into::into),
        }
    }
}
