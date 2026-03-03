#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AufAbschlagProOrt {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "postalCode")]
    pub postleitzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "municipality")]
    pub ort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkAreaCode")]
    pub netznummer: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "surcharges")]
    pub auf_abschlaege: Vec<crate::AufAbschlag>,
}
impl From<bo4e_core::com::SurchargePerLocation> for AufAbschlagProOrt {
    fn from(v: bo4e_core::com::SurchargePerLocation) -> Self {
        Self {
            meta: v.meta,
            postleitzahl: v.postal_code,
            ort: v.municipality,
            netznummer: v.network_area_code,
            auf_abschlaege: v.surcharges.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<AufAbschlagProOrt> for bo4e_core::com::SurchargePerLocation {
    fn from(v: AufAbschlagProOrt) -> Self {
        Self {
            meta: v.meta,
            postal_code: v.postleitzahl,
            municipality: v.ort,
            network_area_code: v.netznummer,
            surcharges: v.auf_abschlaege.into_iter().map(Into::into).collect(),
        }
    }
}
