#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Angebotsvariante {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "offerStatus")]
    pub angebotsstatus: Option<crate::Angebotsstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "creationDate")]
    pub erstellungsdatum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "bindingDeadline")]
    pub bindefrist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "partsCount")]
    pub anzahl_teile: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalQuantityValue")]
    pub gesamtmenge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalCostValue")]
    pub gesamtkosten: Option<f64>,
}
impl From<bo4e_core::com::OfferVariant> for Angebotsvariante {
    fn from(v: bo4e_core::com::OfferVariant) -> Self {
        Self {
            meta: v.meta,
            angebotsstatus: v.offer_status.map(Into::into),
            erstellungsdatum: v.creation_date,
            bindefrist: v.binding_deadline,
            anzahl_teile: v.parts_count,
            gesamtmenge: v.total_quantity_value,
            gesamtkosten: v.total_cost_value,
        }
    }
}
impl From<Angebotsvariante> for bo4e_core::com::OfferVariant {
    fn from(v: Angebotsvariante) -> Self {
        Self {
            meta: v.meta,
            offer_status: v.angebotsstatus.map(Into::into),
            creation_date: v.erstellungsdatum,
            binding_deadline: v.bindefrist,
            parts_count: v.anzahl_teile,
            total_quantity_value: v.gesamtmenge,
            total_cost_value: v.gesamtkosten,
        }
    }
}
