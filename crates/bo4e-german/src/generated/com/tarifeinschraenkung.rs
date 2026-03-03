#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifeinschraenkung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "customerTypes")]
    pub kundentypen: Vec<crate::Kundentyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "requiredFeatures")]
    pub tarifmerkmale: Vec<crate::Tarifmerkmal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "excludedFeatures")]
    pub ausgeschlossene_tarifmerkmale: Vec<crate::Tarifmerkmal>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "minAnnualConsumption")]
    pub mindestjahresverbrauch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "maxAnnualConsumption")]
    pub hoechstjahresverbrauch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "notes")]
    pub bemerkung: Option<String>,
}
impl From<bo4e_core::com::TariffRestriction> for Tarifeinschraenkung {
    fn from(v: bo4e_core::com::TariffRestriction) -> Self {
        Self {
            meta: v.meta,
            kundentypen: v.customer_types.into_iter().map(Into::into).collect(),
            sparte: v.division.map(Into::into),
            tarifmerkmale: v.required_features.into_iter().map(Into::into).collect(),
            ausgeschlossene_tarifmerkmale: v
                .excluded_features
                .into_iter()
                .map(Into::into)
                .collect(),
            mindestjahresverbrauch: v.min_annual_consumption,
            hoechstjahresverbrauch: v.max_annual_consumption,
            bemerkung: v.notes,
        }
    }
}
impl From<Tarifeinschraenkung> for bo4e_core::com::TariffRestriction {
    fn from(v: Tarifeinschraenkung) -> Self {
        Self {
            meta: v.meta,
            customer_types: v.kundentypen.into_iter().map(Into::into).collect(),
            division: v.sparte.map(Into::into),
            required_features: v.tarifmerkmale.into_iter().map(Into::into).collect(),
            excluded_features: v
                .ausgeschlossene_tarifmerkmale
                .into_iter()
                .map(Into::into)
                .collect(),
            min_annual_consumption: v.mindestjahresverbrauch,
            max_annual_consumption: v.hoechstjahresverbrauch,
            notes: v.bemerkung,
        }
    }
}
