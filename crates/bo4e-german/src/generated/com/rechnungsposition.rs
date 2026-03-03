#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rechnungsposition {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "positionNumber")]
    pub positionsnummer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "positionText")]
    pub positionstext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deliveryPeriodStart")]
    pub lieferungszeitraum_von: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deliveryPeriodEnd")]
    pub lieferungszeitraum_bis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "quantityValue")]
    pub positionsmenge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unitPriceValue")]
    pub einzelpreis: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalPriceValue")]
    pub gesamtpreis: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "articleNumber")]
    pub artikelnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "articleId")]
    pub artikel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "taxAmountValue")]
    pub steuerbetrag: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timeUnit")]
    pub zeiteinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timeBasedQuantityValue")]
    pub zeitbezogene_menge: Option<f64>,
}
impl From<bo4e_core::com::InvoicePosition> for Rechnungsposition {
    fn from(v: bo4e_core::com::InvoicePosition) -> Self {
        Self {
            meta: v.meta,
            positionsnummer: v.position_number,
            positionstext: v.position_text,
            lieferungszeitraum_von: v.delivery_period_start,
            lieferungszeitraum_bis: v.delivery_period_end,
            positionsmenge: v.quantity_value,
            einzelpreis: v.unit_price_value,
            gesamtpreis: v.total_price_value,
            artikelnummer: v.article_number,
            artikel_id: v.article_id,
            steuerbetrag: v.tax_amount_value,
            zeiteinheit: v.time_unit.map(Into::into),
            zeitbezogene_menge: v.time_based_quantity_value,
        }
    }
}
impl From<Rechnungsposition> for bo4e_core::com::InvoicePosition {
    fn from(v: Rechnungsposition) -> Self {
        Self {
            meta: v.meta,
            position_number: v.positionsnummer,
            position_text: v.positionstext,
            delivery_period_start: v.lieferungszeitraum_von,
            delivery_period_end: v.lieferungszeitraum_bis,
            quantity_value: v.positionsmenge,
            unit_price_value: v.einzelpreis,
            total_price_value: v.gesamtpreis,
            article_number: v.artikelnummer,
            article_id: v.artikel_id,
            tax_amount_value: v.steuerbetrag,
            time_unit: v.zeiteinheit.map(Into::into),
            time_based_quantity_value: v.zeitbezogene_menge,
        }
    }
}
