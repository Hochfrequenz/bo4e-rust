#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zaehlzeitregister {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "registerId")]
    pub zaehlwerkskennung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariffTime")]
    pub tarifzeit: Option<crate::Tarifzeit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "activeStartTime")]
    pub aktivzeitbeginn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "activeEndTime")]
    pub aktivzeitende: Option<String>,
}
impl From<bo4e_core::com::TimeOfUseRegister> for Zaehlzeitregister {
    fn from(v: bo4e_core::com::TimeOfUseRegister) -> Self {
        Self {
            meta: v.meta,
            zaehlwerkskennung: v.register_id,
            obis_kennzahl: v.obis_code,
            tarifzeit: v.tariff_time.map(Into::into),
            einheit: v.unit.map(Into::into),
            bezeichnung: v.description,
            aktivzeitbeginn: v.active_start_time,
            aktivzeitende: v.active_end_time,
        }
    }
}
impl From<Zaehlzeitregister> for bo4e_core::com::TimeOfUseRegister {
    fn from(v: Zaehlzeitregister) -> Self {
        Self {
            meta: v.meta,
            register_id: v.zaehlwerkskennung,
            obis_code: v.obis_kennzahl,
            tariff_time: v.tarifzeit.map(Into::into),
            unit: v.einheit.map(Into::into),
            description: v.bezeichnung,
            active_start_time: v.aktivzeitbeginn,
            active_end_time: v.aktivzeitende,
        }
    }
}
