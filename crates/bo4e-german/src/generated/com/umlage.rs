#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Umlage {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "referenceUnit")]
    pub bezugseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "legalReference")]
    pub gesetzliche_grundlage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl From<bo4e_core::com::Levy> for Umlage {
    fn from(v: bo4e_core::com::Levy) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.description,
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bezugseinheit: v.reference_unit.map(Into::into),
            gesetzliche_grundlage: v.legal_reference,
            website: v.website,
        }
    }
}
impl From<Umlage> for bo4e_core::com::Levy {
    fn from(v: Umlage) -> Self {
        Self {
            meta: v.meta,
            description: v.bezeichnung,
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            reference_unit: v.bezugseinheit.map(Into::into),
            legal_reference: v.gesetzliche_grundlage,
            website: v.website,
        }
    }
}
