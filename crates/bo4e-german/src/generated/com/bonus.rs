#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bonus {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "conditions")]
    pub bedingungen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isOneTime")]
    pub einmalig: Option<bool>,
}
impl From<bo4e_core::com::Bonus> for Bonus {
    fn from(v: bo4e_core::com::Bonus) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.description,
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bedingungen: v.conditions,
            einmalig: v.is_one_time,
        }
    }
}
impl From<Bonus> for bo4e_core::com::Bonus {
    fn from(v: Bonus) -> Self {
        Self {
            meta: v.meta,
            description: v.bezeichnung,
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            conditions: v.bedingungen,
            is_one_time: v.einmalig,
        }
    }
}
