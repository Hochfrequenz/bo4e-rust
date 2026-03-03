#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AufAbschlagRegional {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "regionCriterion")]
    pub tarifregionskriterium: Option<crate::Tarifregionskriterium>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "regionCode")]
    pub regionscode: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "surcharges")]
    pub auf_abschlaege: Vec<crate::AufAbschlag>,
}
impl From<bo4e_core::com::RegionalSurcharge> for AufAbschlagRegional {
    fn from(v: bo4e_core::com::RegionalSurcharge) -> Self {
        Self {
            meta: v.meta,
            tarifregionskriterium: v.region_criterion.map(Into::into),
            regionscode: v.region_code,
            auf_abschlaege: v.surcharges.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<AufAbschlagRegional> for bo4e_core::com::RegionalSurcharge {
    fn from(v: AufAbschlagRegional) -> Self {
        Self {
            meta: v.meta,
            region_criterion: v.tarifregionskriterium.map(Into::into),
            region_code: v.regionscode,
            surcharges: v.auf_abschlaege.into_iter().map(Into::into).collect(),
        }
    }
}
