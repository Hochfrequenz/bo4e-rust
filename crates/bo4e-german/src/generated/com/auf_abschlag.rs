#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AufAbschlag {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "surchargeType")]
    pub auf_abschlagstyp: Option<crate::AufAbschlagstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrungseinheit: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "target")]
    pub auf_abschlagsziel: Option<crate::AufAbschlagsziel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "tiers")]
    pub staffeln: Vec<crate::Preisstaffel>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "details")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl From<bo4e_core::com::Surcharge> for AufAbschlag {
    fn from(v: bo4e_core::com::Surcharge) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.description,
            auf_abschlagstyp: v.surcharge_type.map(Into::into),
            wert: v.value,
            waehrungseinheit: v.currency.map(Into::into),
            auf_abschlagsziel: v.target.map(Into::into),
            staffeln: v.tiers.into_iter().map(Into::into).collect(),
            beschreibung: v.details,
            website: v.website,
        }
    }
}
impl From<AufAbschlag> for bo4e_core::com::Surcharge {
    fn from(v: AufAbschlag) -> Self {
        Self {
            meta: v.meta,
            description: v.bezeichnung,
            surcharge_type: v.auf_abschlagstyp.map(Into::into),
            value: v.wert,
            currency: v.waehrungseinheit.map(Into::into),
            target: v.auf_abschlagsziel.map(Into::into),
            tiers: v.staffeln.into_iter().map(Into::into).collect(),
            details: v.beschreibung,
            website: v.website,
        }
    }
}
