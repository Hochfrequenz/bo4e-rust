#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Betrag {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
}
impl From<bo4e_core::com::Amount> for Betrag {
    fn from(v: bo4e_core::com::Amount) -> Self {
        Self {
            meta: v.meta,
            wert: v.value,
            waehrung: v.currency.map(Into::into),
        }
    }
}
impl From<Betrag> for bo4e_core::com::Amount {
    fn from(v: Betrag) -> Self {
        Self {
            meta: v.meta,
            value: v.wert,
            currency: v.waehrung.map(Into::into),
        }
    }
}
