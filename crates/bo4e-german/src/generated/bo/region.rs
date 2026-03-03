#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "regionCode")]
    pub regionscode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "regionType")]
    pub gebietstyp: Option<crate::Regiontyp>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "criteria")]
    pub regionskriterien: Vec<crate::Regionskriterium>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "parentRegion")]
    pub uebergeordnete_region: Option<Box<crate::Region>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "subRegions")]
    pub unterregionen: Vec<crate::Region>,
}
impl From<bo4e_core::bo::Region> for Region {
    fn from(v: bo4e_core::bo::Region) -> Self {
        Self {
            meta: v.meta,
            regionscode: v.region_code,
            name: v.name,
            beschreibung: v.description,
            gebietstyp: v.region_type.map(Into::into),
            regionskriterien: v.criteria.into_iter().map(Into::into).collect(),
            uebergeordnete_region: v.parent_region.map(|b| Box::new((*b).into())),
            unterregionen: v.sub_regions.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<Region> for bo4e_core::bo::Region {
    fn from(v: Region) -> Self {
        Self {
            meta: v.meta,
            region_code: v.regionscode,
            name: v.name,
            description: v.beschreibung,
            region_type: v.gebietstyp.map(Into::into),
            criteria: v.regionskriterien.into_iter().map(Into::into).collect(),
            parent_region: v.uebergeordnete_region.map(|b| Box::new((*b).into())),
            sub_regions: v.unterregionen.into_iter().map(Into::into).collect(),
        }
    }
}
