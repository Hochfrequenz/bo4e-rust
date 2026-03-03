#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Energiemenge {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyAmountId")]
    pub energiemenge_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyDirection")]
    pub energierichtung: Option<crate::Energierichtung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "measurementType")]
    pub messart: Option<crate::Messart>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "measuredValues"
    )]
    pub messwerte: Vec<crate::Messwert>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalEnergy")]
    pub gesamtenergie: Option<f64>,
}
impl From<bo4e_core::bo::EnergyAmount> for Energiemenge {
    fn from(v: bo4e_core::bo::EnergyAmount) -> Self {
        Self {
            meta: v.meta,
            energiemenge_id: v.energy_amount_id,
            sparte: v.division.map(Into::into),
            energierichtung: v.energy_direction.map(Into::into),
            messart: v.measurement_type.map(Into::into),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            messwerte: v.measured_values.into_iter().map(Into::into).collect(),
            marktlokations_id: v.market_location_id,
            messlokations_id: v.metering_location_id,
            obis_kennzahl: v.obis_code,
            gesamtenergie: v.total_energy,
        }
    }
}
impl From<Energiemenge> for bo4e_core::bo::EnergyAmount {
    fn from(v: Energiemenge) -> Self {
        Self {
            meta: v.meta,
            energy_amount_id: v.energiemenge_id,
            division: v.sparte.map(Into::into),
            energy_direction: v.energierichtung.map(Into::into),
            measurement_type: v.messart.map(Into::into),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            measured_values: v.messwerte.into_iter().map(Into::into).collect(),
            market_location_id: v.marktlokations_id,
            metering_location_id: v.messlokations_id,
            obis_code: v.obis_kennzahl,
            total_energy: v.gesamtenergie,
        }
    }
}
