#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Energieherkunft {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "generationType")]
    pub erzeugungsart: Option<crate::Erzeugungsart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "percentageShare")]
    pub anteil_prozent: Option<f64>,
}
impl From<bo4e_core::com::EnergySource> for Energieherkunft {
    fn from(v: bo4e_core::com::EnergySource) -> Self {
        Self {
            meta: v.meta,
            erzeugungsart: v.generation_type.map(Into::into),
            anteil_prozent: v.percentage_share,
        }
    }
}
impl From<Energieherkunft> for bo4e_core::com::EnergySource {
    fn from(v: Energieherkunft) -> Self {
        Self {
            meta: v.meta,
            generation_type: v.erzeugungsart.map(Into::into),
            percentage_share: v.anteil_prozent,
        }
    }
}
