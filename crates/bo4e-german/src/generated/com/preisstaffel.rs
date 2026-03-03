#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preisstaffel {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "lowerLimit")]
    pub staffelgrenze_von: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "upperLimit")]
    pub staffelgrenze_bis: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unitPrice")]
    pub einheitspreis: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tierNumber")]
    pub staffelnummer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "articleId")]
    pub artikel_id: Option<String>,
}
impl From<bo4e_core::com::PriceTier> for Preisstaffel {
    fn from(v: bo4e_core::com::PriceTier) -> Self {
        Self {
            meta: v.meta,
            staffelgrenze_von: v.lower_limit,
            staffelgrenze_bis: v.upper_limit,
            einheitspreis: v.unit_price,
            staffelnummer: v.tier_number,
            artikel_id: v.article_id,
        }
    }
}
impl From<Preisstaffel> for bo4e_core::com::PriceTier {
    fn from(v: Preisstaffel) -> Self {
        Self {
            meta: v.meta,
            lower_limit: v.staffelgrenze_von,
            upper_limit: v.staffelgrenze_bis,
            unit_price: v.einheitspreis,
            tier_number: v.staffelnummer,
            article_id: v.artikel_id,
        }
    }
}
