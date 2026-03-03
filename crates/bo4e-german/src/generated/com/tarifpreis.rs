#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifpreis {
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
}
impl From<bo4e_core::com::TariffPrice> for Tarifpreis {
    fn from(v: bo4e_core::com::TariffPrice) -> Self {
        Self {
            meta: v.meta,
            preistyp: v.price_type.map(Into::into),
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bezugseinheit: v.reference_unit.map(Into::into),
            beschreibung: v.description,
        }
    }
}
impl From<Tarifpreis> for bo4e_core::com::TariffPrice {
    fn from(v: Tarifpreis) -> Self {
        Self {
            meta: v.meta,
            price_type: v.preistyp.map(Into::into),
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            reference_unit: v.bezugseinheit.map(Into::into),
            description: v.beschreibung,
        }
    }
}
