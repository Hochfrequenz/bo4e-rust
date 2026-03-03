#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regionskriterium {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityType")]
    pub gueltigkeitstyp: Option<crate::Gueltigkeitstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "criterionType")]
    pub regionskriteriumtyp: Option<crate::Regionskriteriumtyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<String>,
}
impl From<bo4e_core::com::RegionCriterion> for Regionskriterium {
    fn from(v: bo4e_core::com::RegionCriterion) -> Self {
        Self {
            meta: v.meta,
            gueltigkeitstyp: v.validity_type.map(Into::into),
            regionskriteriumtyp: v.criterion_type.map(Into::into),
            wert: v.value,
        }
    }
}
impl From<Regionskriterium> for bo4e_core::com::RegionCriterion {
    fn from(v: Regionskriterium) -> Self {
        Self {
            meta: v.meta,
            validity_type: v.gueltigkeitstyp.map(Into::into),
            criterion_type: v.regionskriteriumtyp.map(Into::into),
            value: v.wert,
        }
    }
}
