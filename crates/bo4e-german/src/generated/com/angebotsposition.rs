#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Angebotsposition {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "positionDescription")]
    pub positionsbezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "positionPriceValue")]
    pub positionspreis: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "positionQuantityValue"
    )]
    pub positionsmenge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "positionCostValue")]
    pub positionskosten: Option<f64>,
}
impl From<bo4e_core::com::OfferPosition> for Angebotsposition {
    fn from(v: bo4e_core::com::OfferPosition) -> Self {
        Self {
            meta: v.meta,
            positionsbezeichnung: v.position_description,
            positionspreis: v.position_price_value,
            positionsmenge: v.position_quantity_value,
            positionskosten: v.position_cost_value,
        }
    }
}
impl From<Angebotsposition> for bo4e_core::com::OfferPosition {
    fn from(v: Angebotsposition) -> Self {
        Self {
            meta: v.meta,
            position_description: v.positionsbezeichnung,
            position_price_value: v.positionspreis,
            position_quantity_value: v.positionsmenge,
            position_cost_value: v.positionskosten,
        }
    }
}
