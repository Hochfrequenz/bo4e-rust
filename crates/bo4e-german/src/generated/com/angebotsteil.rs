#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Angebotsteil {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "requestSubReference")]
    pub anfrage_subreferenz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "positionCount")]
    pub anzahl_positionen: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalQuantityValue")]
    pub gesamtmenge_angebotsteil: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalCostValue")]
    pub gesamtkosten_angebotsteil: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deliveryPeriodStart")]
    pub lieferzeitraum_beginn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deliveryPeriodEnd")]
    pub lieferzeitraum_ende: Option<String>,
}
impl From<bo4e_core::com::OfferPart> for Angebotsteil {
    fn from(v: bo4e_core::com::OfferPart) -> Self {
        Self {
            meta: v.meta,
            anfrage_subreferenz: v.request_sub_reference,
            anzahl_positionen: v.position_count,
            gesamtmenge_angebotsteil: v.total_quantity_value,
            gesamtkosten_angebotsteil: v.total_cost_value,
            lieferzeitraum_beginn: v.delivery_period_start,
            lieferzeitraum_ende: v.delivery_period_end,
        }
    }
}
impl From<Angebotsteil> for bo4e_core::com::OfferPart {
    fn from(v: Angebotsteil) -> Self {
        Self {
            meta: v.meta,
            request_sub_reference: v.anfrage_subreferenz,
            position_count: v.anzahl_positionen,
            total_quantity_value: v.gesamtmenge_angebotsteil,
            total_cost_value: v.gesamtkosten_angebotsteil,
            delivery_period_start: v.lieferzeitraum_beginn,
            delivery_period_end: v.lieferzeitraum_ende,
        }
    }
}
