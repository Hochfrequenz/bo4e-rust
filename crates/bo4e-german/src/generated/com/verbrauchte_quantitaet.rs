#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerbrauchteQuantitaet {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
}
impl From<bo4e_core::com::ConsumedQuantity> for VerbrauchteQuantitaet {
    fn from(v: bo4e_core::com::ConsumedQuantity) -> Self {
        Self {
            meta: v.meta,
            wert: v.value,
            einheit: v.unit.map(Into::into),
        }
    }
}
impl From<VerbrauchteQuantitaet> for bo4e_core::com::ConsumedQuantity {
    fn from(v: VerbrauchteQuantitaet) -> Self {
        Self {
            meta: v.meta,
            value: v.wert,
            unit: v.einheit.map(Into::into),
        }
    }
}
