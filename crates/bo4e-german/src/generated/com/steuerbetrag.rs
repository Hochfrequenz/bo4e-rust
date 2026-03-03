#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steuerbetrag {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "taxType")]
    pub steuerart: Option<crate::Steuerart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "taxRate")]
    pub steuersatz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "basisValue")]
    pub basiswert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "taxValue")]
    pub steuerwert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrungscode: Option<crate::Waehrungscode>,
}
impl From<bo4e_core::com::TaxAmount> for Steuerbetrag {
    fn from(v: bo4e_core::com::TaxAmount) -> Self {
        Self {
            meta: v.meta,
            steuerart: v.tax_type.map(Into::into),
            steuersatz: v.tax_rate,
            basiswert: v.basis_value,
            steuerwert: v.tax_value,
            waehrungscode: v.currency.map(Into::into),
        }
    }
}
impl From<Steuerbetrag> for bo4e_core::com::TaxAmount {
    fn from(v: Steuerbetrag) -> Self {
        Self {
            meta: v.meta,
            tax_type: v.steuerart.map(Into::into),
            tax_rate: v.steuersatz,
            basis_value: v.basiswert,
            tax_value: v.steuerwert,
            currency: v.waehrungscode.map(Into::into),
        }
    }
}
