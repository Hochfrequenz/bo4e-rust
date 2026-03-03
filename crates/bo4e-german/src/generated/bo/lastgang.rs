#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lastgang {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "loadProfileId")]
    pub lastgang_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyDirection")]
    pub energierichtung: Option<crate::Energierichtung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "measurementType")]
    pub messart: Option<crate::Messart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "values")]
    pub lastgangwerte: Vec<crate::Lastprofilwert>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "intervalMinutes")]
    pub intervalllaenge: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "standardProfileType")]
    pub standardlastprofil: Option<String>,
}
impl From<bo4e_core::bo::LoadProfile> for Lastgang {
    fn from(v: bo4e_core::bo::LoadProfile) -> Self {
        Self {
            meta: v.meta,
            lastgang_id: v.load_profile_id,
            sparte: v.division.map(Into::into),
            energierichtung: v.energy_direction.map(Into::into),
            messart: v.measurement_type.map(Into::into),
            einheit: v.unit.map(Into::into),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            lastgangwerte: v.values.into_iter().map(Into::into).collect(),
            marktlokations_id: v.market_location_id,
            messlokations_id: v.metering_location_id,
            obis_kennzahl: v.obis_code,
            intervalllaenge: v.interval_minutes,
            standardlastprofil: v.standard_profile_type,
        }
    }
}
impl From<Lastgang> for bo4e_core::bo::LoadProfile {
    fn from(v: Lastgang) -> Self {
        Self {
            meta: v.meta,
            load_profile_id: v.lastgang_id,
            division: v.sparte.map(Into::into),
            energy_direction: v.energierichtung.map(Into::into),
            measurement_type: v.messart.map(Into::into),
            unit: v.einheit.map(Into::into),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            values: v.lastgangwerte.into_iter().map(Into::into).collect(),
            market_location_id: v.marktlokations_id,
            metering_location_id: v.messlokations_id,
            obis_code: v.obis_kennzahl,
            interval_minutes: v.intervalllaenge,
            standard_profile_type: v.standardlastprofil,
        }
    }
}
