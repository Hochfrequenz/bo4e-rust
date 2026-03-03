#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dienstleistungspreis {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "serviceType")]
    pub dienstleistungstyp: Option<crate::Dienstleistungstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "referenceUnit")]
    pub bezugseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
}
impl From<bo4e_core::com::ServicePrice> for Dienstleistungspreis {
    fn from(v: bo4e_core::com::ServicePrice) -> Self {
        Self {
            meta: v.meta,
            dienstleistungstyp: v.service_type.map(Into::into),
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bezugseinheit: v.reference_unit.map(Into::into),
            beschreibung: v.description,
        }
    }
}
impl From<Dienstleistungspreis> for bo4e_core::com::ServicePrice {
    fn from(v: Dienstleistungspreis) -> Self {
        Self {
            meta: v.meta,
            service_type: v.dienstleistungstyp.map(Into::into),
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            reference_unit: v.bezugseinheit.map(Into::into),
            description: v.beschreibung,
        }
    }
}
