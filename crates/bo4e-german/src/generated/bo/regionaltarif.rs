#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regionaltarif {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariffCode")]
    pub tarifcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "name")]
    pub tarifname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "provider")]
    pub tarifanbieter: Option<Box<crate::Geschaeftspartner>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Box<crate::Region>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startDate")]
    pub startdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endDate")]
    pub enddatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "priceTiers")]
    pub regionale_preisstufen: Vec<crate::RegionalePreisstaffel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "surcharges")]
    pub regionale_aufschlaege: Vec<crate::AufAbschlagRegional>,
}
impl From<bo4e_core::bo::RegionalTariff> for Regionaltarif {
    fn from(v: bo4e_core::bo::RegionalTariff) -> Self {
        Self {
            meta: v.meta,
            tarifcode: v.tariff_code,
            tarifname: v.name,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            tarifanbieter: v.provider.map(|b| Box::new((*b).into())),
            region: v.region.map(|b| Box::new((*b).into())),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            startdatum: v.start_date,
            enddatum: v.end_date,
            regionale_preisstufen: v.price_tiers.into_iter().map(Into::into).collect(),
            regionale_aufschlaege: v.surcharges.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<Regionaltarif> for bo4e_core::bo::RegionalTariff {
    fn from(v: Regionaltarif) -> Self {
        Self {
            meta: v.meta,
            tariff_code: v.tarifcode,
            name: v.tarifname,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            provider: v.tarifanbieter.map(|b| Box::new((*b).into())),
            region: v.region.map(|b| Box::new((*b).into())),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            start_date: v.startdatum,
            end_date: v.enddatum,
            price_tiers: v
                .regionale_preisstufen
                .into_iter()
                .map(Into::into)
                .collect(),
            surcharges: v
                .regionale_aufschlaege
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
