#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsAufAbschlag {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "surchargeType")]
    pub auf_abschlagstyp: Option<crate::AufAbschlagstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
}
impl From<bo4e_core::com::PositionSurcharge> for PositionsAufAbschlag {
    fn from(v: bo4e_core::com::PositionSurcharge) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.description,
            auf_abschlagstyp: v.surcharge_type.map(Into::into),
            wert: v.value,
            waehrung: v.currency.map(Into::into),
        }
    }
}
impl From<PositionsAufAbschlag> for bo4e_core::com::PositionSurcharge {
    fn from(v: PositionsAufAbschlag) -> Self {
        Self {
            meta: v.meta,
            description: v.bezeichnung,
            surcharge_type: v.auf_abschlagstyp.map(Into::into),
            value: v.wert,
            currency: v.waehrung.map(Into::into),
        }
    }
}
