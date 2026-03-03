#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lastkurvendaten {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timestamp")]
    pub zeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "powerValue")]
    pub leistungswert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "powerUnit")]
    pub leistungseinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyValue")]
    pub energiewert: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyUnit")]
    pub energieeinheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "intervalMinutes")]
    pub intervalllaenge: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "measurementLocationId")]
    pub messlokations_id: Option<String>,
}
impl From<bo4e_core::com::LoadCurveData> for Lastkurvendaten {
    fn from(v: bo4e_core::com::LoadCurveData) -> Self {
        Self {
            meta: v.meta,
            zeitpunkt: v.timestamp,
            leistungswert: v.power_value,
            leistungseinheit: v.power_unit.map(Into::into),
            energiewert: v.energy_value,
            energieeinheit: v.energy_unit.map(Into::into),
            intervalllaenge: v.interval_minutes,
            obis_kennzahl: v.obis_code,
            messlokations_id: v.measurement_location_id,
        }
    }
}
impl From<Lastkurvendaten> for bo4e_core::com::LoadCurveData {
    fn from(v: Lastkurvendaten) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.zeitpunkt,
            power_value: v.leistungswert,
            power_unit: v.leistungseinheit.map(Into::into),
            energy_value: v.energiewert,
            energy_unit: v.energieeinheit.map(Into::into),
            interval_minutes: v.intervalllaenge,
            obis_code: v.obis_kennzahl,
            measurement_location_id: v.messlokations_id,
        }
    }
}
