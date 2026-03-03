#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifinfo {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariffName")]
    pub tarifname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub tarifbeschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customerType")]
    pub kundentyp: Option<crate::Kundentyp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "availableFrom")]
    pub angebotsdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "availableUntil")]
    pub enddatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyMix")]
    pub energiemix: Option<crate::Energiemix>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceGuarantee")]
    pub preisgarantie: Option<crate::Preisgarantie>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "restrictions")]
    pub tarifeinschraenkungen: Vec<crate::Tarifeinschraenkung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "supplier")]
    pub anbieter: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::TariffInfo> for Tarifinfo {
    fn from(v: bo4e_core::bo::TariffInfo) -> Self {
        Self {
            meta: v.meta,
            tarifname: v.tariff_name,
            tarifbeschreibung: v.description,
            sparte: v.division.map(Into::into),
            kundentyp: v.customer_type.map(Into::into),
            website: v.website,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            angebotsdatum: v.available_from,
            enddatum: v.available_until,
            energiemix: v.energy_mix.map(Into::into),
            preisgarantie: v.price_guarantee.map(Into::into),
            tarifeinschraenkungen: v.restrictions.into_iter().map(Into::into).collect(),
            anbieter: v.supplier.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Tarifinfo> for bo4e_core::bo::TariffInfo {
    fn from(v: Tarifinfo) -> Self {
        Self {
            meta: v.meta,
            tariff_name: v.tarifname,
            description: v.tarifbeschreibung,
            division: v.sparte.map(Into::into),
            customer_type: v.kundentyp.map(Into::into),
            website: v.website,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            available_from: v.angebotsdatum,
            available_until: v.enddatum,
            energy_mix: v.energiemix.map(Into::into),
            price_guarantee: v.preisgarantie.map(Into::into),
            restrictions: v.tarifeinschraenkungen.into_iter().map(Into::into).collect(),
            supplier: v.anbieter.map(|b| Box::new((*b).into())),
        }
    }
}
