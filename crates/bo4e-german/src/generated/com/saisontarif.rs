#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Saisontarif {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "seasonName")]
    pub saisonbezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "startDate")]
    pub startdatum: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "endDate")]
    pub enddatum: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariffId")]
    pub tarifkennung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceFactor")]
    pub preisfaktor: Option<f64>,
}
impl From<bo4e_core::com::SeasonalTariff> for Saisontarif {
    fn from(v: bo4e_core::com::SeasonalTariff) -> Self {
        Self {
            meta: v.meta,
            saisonbezeichnung: v.season_name,
            startdatum: v.start_date,
            enddatum: v.end_date,
            tarifkennung: v.tariff_id,
            preisfaktor: v.price_factor,
        }
    }
}
impl From<Saisontarif> for bo4e_core::com::SeasonalTariff {
    fn from(v: Saisontarif) -> Self {
        Self {
            meta: v.meta,
            season_name: v.saisonbezeichnung,
            start_date: v.startdatum,
            end_date: v.enddatum,
            tariff_id: v.tarifkennung,
            price_factor: v.preisfaktor,
        }
    }
}
