#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kostenposition {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    pub positionstitel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "amount")]
    pub betrag_kostenposition: Option<crate::Betrag>,
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
    #[serde(skip_serializing_if = "Option::is_none", alias = "timeQuantityValue")]
    pub zeitmenge_wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timeQuantityUnit")]
    pub zeitmenge_einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "articleDetail")]
    pub artikeldetail: Option<String>,
}
impl From<bo4e_core::com::CostPosition> for Kostenposition {
    fn from(v: bo4e_core::com::CostPosition) -> Self {
        Self {
            meta: v.meta,
            positionstitel: v.title,
            betrag_kostenposition: v.amount.map(Into::into),
            artikelbezeichnung: v.article_description,
            einzelpreis: v.unit_price.map(Into::into),
            von: v.start_date,
            bis: v.end_date,
            menge_wert: v.quantity_value,
            menge_einheit: v.quantity_unit.map(Into::into),
            zeitmenge_wert: v.time_quantity_value,
            zeitmenge_einheit: v.time_quantity_unit.map(Into::into),
            artikeldetail: v.article_detail,
        }
    }
}
impl From<Kostenposition> for bo4e_core::com::CostPosition {
    fn from(v: Kostenposition) -> Self {
        Self {
            meta: v.meta,
            title: v.positionstitel,
            amount: v.betrag_kostenposition.map(Into::into),
            article_description: v.artikelbezeichnung,
            unit_price: v.einzelpreis.map(Into::into),
            start_date: v.von,
            end_date: v.bis,
            quantity_value: v.menge_wert,
            quantity_unit: v.menge_einheit.map(Into::into),
            time_quantity_value: v.zeitmenge_wert,
            time_quantity_unit: v.zeitmenge_einheit.map(Into::into),
            article_detail: v.artikeldetail,
        }
    }
}
