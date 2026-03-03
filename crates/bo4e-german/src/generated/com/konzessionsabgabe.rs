#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Konzessionsabgabe {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "feeType")]
    pub konzessionsabgabentyp: Option<crate::Abgabeart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customerGroup")]
    pub kundengruppe_ka: Option<crate::KundengruppeKA>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub wert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "currency")]
    pub waehrung: Option<crate::Waehrungscode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "referenceUnit")]
    pub bezugseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
}
impl From<bo4e_core::com::ConcessionFee> for Konzessionsabgabe {
    fn from(v: bo4e_core::com::ConcessionFee) -> Self {
        Self {
            meta: v.meta,
            konzessionsabgabentyp: v.fee_type.map(Into::into),
            kundengruppe_ka: v.customer_group.map(Into::into),
            wert: v.value,
            waehrung: v.currency.map(Into::into),
            bezugseinheit: v.reference_unit.map(Into::into),
            beschreibung: v.description,
        }
    }
}
impl From<Konzessionsabgabe> for bo4e_core::com::ConcessionFee {
    fn from(v: Konzessionsabgabe) -> Self {
        Self {
            meta: v.meta,
            fee_type: v.konzessionsabgabentyp.map(Into::into),
            customer_group: v.kundengruppe_ka.map(Into::into),
            value: v.wert,
            currency: v.waehrung.map(Into::into),
            reference_unit: v.bezugseinheit.map(Into::into),
            description: v.beschreibung,
        }
    }
}
