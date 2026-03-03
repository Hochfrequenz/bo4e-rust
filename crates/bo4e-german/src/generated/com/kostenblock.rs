#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kostenblock {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub kostenblockbezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "costClass")]
    pub kostenklasse: Option<crate::Kostenklasse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalAmount")]
    pub summe_kostenblock: Option<crate::Betrag>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "positions")]
    pub kostenpositionen: Vec<crate::Kostenposition>,
}
impl From<bo4e_core::com::CostBlock> for Kostenblock {
    fn from(v: bo4e_core::com::CostBlock) -> Self {
        Self {
            meta: v.meta,
            kostenblockbezeichnung: v.designation,
            kostenklasse: v.cost_class.map(Into::into),
            summe_kostenblock: v.total_amount.map(Into::into),
            kostenpositionen: v.positions.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<Kostenblock> for bo4e_core::com::CostBlock {
    fn from(v: Kostenblock) -> Self {
        Self {
            meta: v.meta,
            designation: v.kostenblockbezeichnung,
            cost_class: v.kostenklasse.map(Into::into),
            total_amount: v.summe_kostenblock.map(Into::into),
            positions: v.kostenpositionen.into_iter().map(Into::into).collect(),
        }
    }
}
