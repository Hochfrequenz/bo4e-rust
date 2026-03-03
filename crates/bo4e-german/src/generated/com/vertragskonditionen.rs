#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vertragskonditionen {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "installmentCount")]
    pub anzahl_abschlaege: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "contractDuration")]
    pub vertragslaufzeit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "noticePeriod")]
    pub kuendigungsfrist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "extensionPeriod")]
    pub vertragsverlaengerung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "installmentCycle")]
    pub abschlagszyklus: Option<String>,
}
impl From<bo4e_core::com::ContractConditions> for Vertragskonditionen {
    fn from(v: bo4e_core::com::ContractConditions) -> Self {
        Self {
            meta: v.meta,
            beschreibung: v.description,
            anzahl_abschlaege: v.installment_count,
            vertragslaufzeit: v.contract_duration,
            kuendigungsfrist: v.notice_period,
            vertragsverlaengerung: v.extension_period,
            abschlagszyklus: v.installment_cycle,
        }
    }
}
impl From<Vertragskonditionen> for bo4e_core::com::ContractConditions {
    fn from(v: Vertragskonditionen) -> Self {
        Self {
            meta: v.meta,
            description: v.beschreibung,
            installment_count: v.anzahl_abschlaege,
            contract_duration: v.vertragslaufzeit,
            notice_period: v.kuendigungsfrist,
            extension_period: v.vertragsverlaengerung,
            installment_cycle: v.abschlagszyklus,
        }
    }
}
