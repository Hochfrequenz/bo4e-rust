#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rabatt {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "discountType")]
    pub rabatttyp: Option<crate::AufAbschlagstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "conditions")]
    pub bedingungen: Option<String>,
}
impl From<bo4e_core::com::Discount> for Rabatt {
    fn from(v: bo4e_core::com::Discount) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.description,
            rabatttyp: v.discount_type.map(Into::into),
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bedingungen: v.conditions,
        }
    }
}
impl From<Rabatt> for bo4e_core::com::Discount {
    fn from(v: Rabatt) -> Self {
        Self {
            meta: v.meta,
            description: v.bezeichnung,
            discount_type: v.rabatttyp.map(Into::into),
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            conditions: v.bedingungen,
        }
    }
}
