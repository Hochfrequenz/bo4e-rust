#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Messlokation {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "coordinates")]
    pub geokoordinaten: Option<crate::Geokoordinaten>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "meteringOperatorCode"
    )]
    pub messstellenbetreiber_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkOperatorCode")]
    pub netzbetreiber_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "gridArea")]
    pub regelzone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "hardware")]
    pub geraete: Vec<crate::Hardware>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "meterIds")]
    pub zaehler: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "marketLocationIds"
    )]
    pub marktlokationen: Vec<String>,
}
impl From<bo4e_core::bo::MeteringLocation> for Messlokation {
    fn from(v: bo4e_core::bo::MeteringLocation) -> Self {
        Self {
            meta: v.meta,
            messlokations_id: v.metering_location_id,
            sparte: v.division.map(Into::into),
            adresse: v.address.map(Into::into),
            geokoordinaten: v.coordinates.map(Into::into),
            messstellenbetreiber_codenummer: v.metering_operator_code,
            netzbetreiber_codenummer: v.network_operator_code,
            regelzone: v.grid_area,
            beschreibung: v.description,
            geraete: v.hardware.into_iter().map(Into::into).collect(),
            zaehler: v.meter_ids,
            marktlokationen: v.market_location_ids,
        }
    }
}
impl From<Messlokation> for bo4e_core::bo::MeteringLocation {
    fn from(v: Messlokation) -> Self {
        Self {
            meta: v.meta,
            metering_location_id: v.messlokations_id,
            division: v.sparte.map(Into::into),
            address: v.adresse.map(Into::into),
            coordinates: v.geokoordinaten.map(Into::into),
            metering_operator_code: v.messstellenbetreiber_codenummer,
            network_operator_code: v.netzbetreiber_codenummer,
            grid_area: v.regelzone,
            description: v.beschreibung,
            hardware: v.geraete.into_iter().map(Into::into).collect(),
            meter_ids: v.zaehler,
            market_location_ids: v.marktlokationen,
        }
    }
}
