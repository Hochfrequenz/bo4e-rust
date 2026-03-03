#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hardware {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deviceNumber")]
    pub geraetenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deviceCategory")]
    pub geraeteklasse: Option<crate::Geraeteklasse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deviceType")]
    pub geraetetyp: Option<crate::Geraetetyp>,
}
impl From<bo4e_core::com::Hardware> for Hardware {
    fn from(v: bo4e_core::com::Hardware) -> Self {
        Self {
            meta: v.meta,
            geraetenummer: v.device_number,
            bezeichnung: v.description,
            geraeteklasse: v.device_category.map(Into::into),
            geraetetyp: v.device_type.map(Into::into),
        }
    }
}
impl From<Hardware> for bo4e_core::com::Hardware {
    fn from(v: Hardware) -> Self {
        Self {
            meta: v.meta,
            device_number: v.geraetenummer,
            description: v.bezeichnung,
            device_category: v.geraeteklasse.map(Into::into),
            device_type: v.geraetetyp.map(Into::into),
        }
    }
}
