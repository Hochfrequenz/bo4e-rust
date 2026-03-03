#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vertragsteil {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractPartStart")]
    pub vertragsteilbeginn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractPartEnd")]
    pub vertragsteilende: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "locationId")]
    pub lokation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "fixedQuantityValue")]
    pub vertraglich_fixierte_menge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "minimumQuantityValue")]
    pub minimale_abnahmemenge: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "maximumQuantityValue")]
    pub maximale_abnahmemenge: Option<f64>,
}
impl From<bo4e_core::com::ContractPart> for Vertragsteil {
    fn from(v: bo4e_core::com::ContractPart) -> Self {
        Self {
            meta: v.meta,
            vertragsteilbeginn: v.contract_part_start,
            vertragsteilende: v.contract_part_end,
            lokation: v.location_id,
            vertraglich_fixierte_menge: v.fixed_quantity_value,
            minimale_abnahmemenge: v.minimum_quantity_value,
            maximale_abnahmemenge: v.maximum_quantity_value,
        }
    }
}
impl From<Vertragsteil> for bo4e_core::com::ContractPart {
    fn from(v: Vertragsteil) -> Self {
        Self {
            meta: v.meta,
            contract_part_start: v.vertragsteilbeginn,
            contract_part_end: v.vertragsteilende,
            location_id: v.lokation,
            fixed_quantity_value: v.vertraglich_fixierte_menge,
            minimum_quantity_value: v.minimale_abnahmemenge,
            maximum_quantity_value: v.maximale_abnahmemenge,
        }
    }
}
