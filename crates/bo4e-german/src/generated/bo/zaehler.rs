#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zaehler {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meterNumber")]
    pub zaehlernummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meterType")]
    pub zaehlertyp: Option<crate::Zaehlertyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meterSize")]
    pub zaehlergroesse: Option<crate::Zaehlergroesse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "location")]
    pub standort: Option<crate::Adresse>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "registers")]
    pub zaehlwerke: Vec<crate::Zaehlwerk>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "hardware")]
    pub geraeteeigenschaften: Vec<crate::Hardware>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "ownership")]
    pub eigentumsverhaeltnis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "manufacturer")]
    pub hersteller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "manufacturingYear")]
    pub herstellungsjahr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "installationDate")]
    pub einbaudatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "removalDate")]
    pub ausbaudatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "calibrationDate")]
    pub eichdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "calibrationExpiryDate"
    )]
    pub eichablaufdatum: Option<chrono::DateTime<chrono::Utc>>,
}
impl From<bo4e_core::bo::Meter> for Zaehler {
    fn from(v: bo4e_core::bo::Meter) -> Self {
        Self {
            meta: v.meta,
            zaehlernummer: v.meter_number,
            sparte: v.division.map(Into::into),
            zaehlertyp: v.meter_type.map(Into::into),
            zaehlergroesse: v.meter_size.map(Into::into),
            standort: v.location.map(Into::into),
            zaehlwerke: v.registers.into_iter().map(Into::into).collect(),
            geraeteeigenschaften: v.hardware.into_iter().map(Into::into).collect(),
            marktlokations_id: v.market_location_id,
            messlokations_id: v.metering_location_id,
            eigentumsverhaeltnis: v.ownership,
            hersteller: v.manufacturer,
            herstellungsjahr: v.manufacturing_year,
            einbaudatum: v.installation_date,
            ausbaudatum: v.removal_date,
            eichdatum: v.calibration_date,
            eichablaufdatum: v.calibration_expiry_date,
        }
    }
}
impl From<Zaehler> for bo4e_core::bo::Meter {
    fn from(v: Zaehler) -> Self {
        Self {
            meta: v.meta,
            meter_number: v.zaehlernummer,
            division: v.sparte.map(Into::into),
            meter_type: v.zaehlertyp.map(Into::into),
            meter_size: v.zaehlergroesse.map(Into::into),
            location: v.standort.map(Into::into),
            registers: v.zaehlwerke.into_iter().map(Into::into).collect(),
            hardware: v.geraeteeigenschaften.into_iter().map(Into::into).collect(),
            market_location_id: v.marktlokations_id,
            metering_location_id: v.messlokations_id,
            ownership: v.eigentumsverhaeltnis,
            manufacturer: v.hersteller,
            manufacturing_year: v.herstellungsjahr,
            installation_date: v.einbaudatum,
            removal_date: v.ausbaudatum,
            calibration_date: v.eichdatum,
            calibration_expiry_date: v.eichablaufdatum,
        }
    }
}
