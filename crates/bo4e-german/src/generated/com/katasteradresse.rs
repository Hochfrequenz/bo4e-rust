#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Katasteradresse {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemarkung_flur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flurstueck: Option<String>,
}
impl From<bo4e_core::com::CadastralAddress> for Katasteradresse {
    fn from(v: bo4e_core::com::CadastralAddress) -> Self {
        Self {
            meta: v.meta,
            gemarkung_flur: v.gemarkung_flur,
            flurstueck: v.flurstueck,
        }
    }
}
impl From<Katasteradresse> for bo4e_core::com::CadastralAddress {
    fn from(v: Katasteradresse) -> Self {
        Self {
            meta: v.meta,
            gemarkung_flur: v.gemarkung_flur,
            flurstueck: v.flurstueck,
        }
    }
}
