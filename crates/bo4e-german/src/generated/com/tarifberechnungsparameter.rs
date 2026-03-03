#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifberechnungsparameter {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "calculationMethod")]
    pub berechnungsmethode: Option<crate::Tarifkalkulationsmethode>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariffTime")]
    pub tarifzeit: Option<crate::Tarifzeit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isDemandBased")]
    pub ist_leistungsabhaengig: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "minAnnualConsumption")]
    pub mindestjahresverbrauch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "maxAnnualConsumption")]
    pub hoechstjahresverbrauch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
}
impl From<bo4e_core::com::TariffCalculationParameter> for Tarifberechnungsparameter {
    fn from(v: bo4e_core::com::TariffCalculationParameter) -> Self {
        Self {
            meta: v.meta,
            berechnungsmethode: v.calculation_method.map(Into::into),
            tarifzeit: v.tariff_time.map(Into::into),
            ist_leistungsabhaengig: v.is_demand_based,
            mindestjahresverbrauch: v.min_annual_consumption,
            hoechstjahresverbrauch: v.max_annual_consumption,
            beschreibung: v.description,
        }
    }
}
impl From<Tarifberechnungsparameter> for bo4e_core::com::TariffCalculationParameter {
    fn from(v: Tarifberechnungsparameter) -> Self {
        Self {
            meta: v.meta,
            calculation_method: v.berechnungsmethode.map(Into::into),
            tariff_time: v.tarifzeit.map(Into::into),
            is_demand_based: v.ist_leistungsabhaengig,
            min_annual_consumption: v.mindestjahresverbrauch,
            max_annual_consumption: v.hoechstjahresverbrauch,
            description: v.beschreibung,
        }
    }
}
