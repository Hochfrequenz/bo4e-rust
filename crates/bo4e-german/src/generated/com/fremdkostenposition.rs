#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fremdkostenposition {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    pub positionstitel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "amount")]
    pub betrag: Option<crate::Betrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "articleDescription")]
    pub artikelbezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unitPrice")]
    pub einzelpreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startDate")]
    pub von: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endDate")]
    pub bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "quantityValue")]
    pub menge_wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "quantityUnit")]
    pub menge_einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "externalReference")]
    pub link: Option<String>,
}
impl From<bo4e_core::com::ExternalCostPosition> for Fremdkostenposition {
    fn from(v: bo4e_core::com::ExternalCostPosition) -> Self {
        Self {
            meta: v.meta,
            positionstitel: v.title,
            betrag: v.amount.map(Into::into),
            artikelbezeichnung: v.article_description,
            einzelpreis: v.unit_price.map(Into::into),
            von: v.start_date,
            bis: v.end_date,
            menge_wert: v.quantity_value,
            menge_einheit: v.quantity_unit.map(Into::into),
            link: v.external_reference,
        }
    }
}
impl From<Fremdkostenposition> for bo4e_core::com::ExternalCostPosition {
    fn from(v: Fremdkostenposition) -> Self {
        Self {
            meta: v.meta,
            title: v.positionstitel,
            amount: v.betrag.map(Into::into),
            article_description: v.artikelbezeichnung,
            unit_price: v.einzelpreis.map(Into::into),
            start_date: v.von,
            end_date: v.bis,
            quantity_value: v.menge_wert,
            quantity_unit: v.menge_einheit.map(Into::into),
            external_reference: v.link,
        }
    }
}
