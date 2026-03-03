#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zeitreihe {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timeSeriesId")]
    pub zeitreihe_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "measurementType")]
    pub messart: Option<crate::Messart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "values")]
    pub zeitreihenwerte: Vec<crate::Zeitreihenwert>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "seriesVersion")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "resolutionMinutes")]
    pub aufloesung: Option<i32>,
}
impl From<bo4e_core::bo::TimeSeries> for Zeitreihe {
    fn from(v: bo4e_core::bo::TimeSeries) -> Self {
        Self {
            meta: v.meta,
            zeitreihe_id: v.time_series_id,
            sparte: v.division.map(Into::into),
            messart: v.measurement_type.map(Into::into),
            einheit: v.unit.map(Into::into),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            zeitreihenwerte: v.values.into_iter().map(Into::into).collect(),
            marktlokations_id: v.market_location_id,
            messlokations_id: v.metering_location_id,
            beschreibung: v.description,
            obis_kennzahl: v.obis_code,
            version: v.series_version,
            aufloesung: v.resolution_minutes,
        }
    }
}
impl From<Zeitreihe> for bo4e_core::bo::TimeSeries {
    fn from(v: Zeitreihe) -> Self {
        Self {
            meta: v.meta,
            time_series_id: v.zeitreihe_id,
            division: v.sparte.map(Into::into),
            measurement_type: v.messart.map(Into::into),
            unit: v.einheit.map(Into::into),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            values: v.zeitreihenwerte.into_iter().map(Into::into).collect(),
            market_location_id: v.marktlokations_id,
            metering_location_id: v.messlokations_id,
            description: v.beschreibung,
            obis_code: v.obis_kennzahl,
            series_version: v.version,
            resolution_minutes: v.aufloesung,
        }
    }
}
