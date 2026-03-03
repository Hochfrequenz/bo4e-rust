#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifpreisposition {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceType")]
    pub preistyp: Option<crate::Preistyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "referenceUnit")]
    pub bezugseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "calculationMethod")]
    pub berechnungsmethode: Option<crate::Kalkulationsmethode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "prices")]
    pub tarifpreise: Vec<crate::Tarifpreis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "articleId")]
    pub artikel_id: Option<String>,
}
impl From<bo4e_core::com::TariffPricePosition> for Tarifpreisposition {
    fn from(v: bo4e_core::com::TariffPricePosition) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.description,
            preistyp: v.price_type.map(Into::into),
            bezugseinheit: v.reference_unit.map(Into::into),
            berechnungsmethode: v.calculation_method.map(Into::into),
            tarifpreise: v.prices.into_iter().map(Into::into).collect(),
            artikel_id: v.article_id,
        }
    }
}
impl From<Tarifpreisposition> for bo4e_core::com::TariffPricePosition {
    fn from(v: Tarifpreisposition) -> Self {
        Self {
            meta: v.meta,
            description: v.bezeichnung,
            price_type: v.preistyp.map(Into::into),
            reference_unit: v.bezugseinheit.map(Into::into),
            calculation_method: v.berechnungsmethode.map(Into::into),
            prices: v.tarifpreise.into_iter().map(Into::into).collect(),
            article_id: v.artikel_id,
        }
    }
}
