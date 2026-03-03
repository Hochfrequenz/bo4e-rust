#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionalePreisstaffel {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "regionCode")]
    pub regionscode: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "tiers")]
    pub preisstaffeln: Vec<crate::Preisstaffel>,
}
impl From<bo4e_core::com::RegionalPriceTier> for RegionalePreisstaffel {
    fn from(v: bo4e_core::com::RegionalPriceTier) -> Self {
        Self {
            meta: v.meta,
            regionscode: v.region_code,
            preisstaffeln: v.tiers.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<RegionalePreisstaffel> for bo4e_core::com::RegionalPriceTier {
    fn from(v: RegionalePreisstaffel) -> Self {
        Self {
            meta: v.meta,
            region_code: v.regionscode,
            tiers: v.preisstaffeln.into_iter().map(Into::into).collect(),
        }
    }
}
