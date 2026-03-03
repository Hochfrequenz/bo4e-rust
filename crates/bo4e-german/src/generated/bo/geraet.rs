#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geraet {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deviceId")]
    pub geraetkennung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "serialNumber")]
    pub seriennummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deviceCategory")]
    pub geraeteklasse: Option<crate::Geraeteklasse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deviceType")]
    pub geraetetyp: Option<crate::Geraetetyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "manufacturer")]
    pub hersteller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "model")]
    pub modellbezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "manufacturingYear")]
    pub baujahr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "installationDate")]
    pub einbaudatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "removalDate")]
    pub ausbaudatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
}
impl From<bo4e_core::bo::Device> for Geraet {
    fn from(v: bo4e_core::bo::Device) -> Self {
        Self {
            meta: v.meta,
            geraetkennung: v.device_id,
            seriennummer: v.serial_number,
            geraeteklasse: v.device_category.map(Into::into),
            geraetetyp: v.device_type.map(Into::into),
            hersteller: v.manufacturer,
            modellbezeichnung: v.model,
            baujahr: v.manufacturing_year,
            einbaudatum: v.installation_date,
            ausbaudatum: v.removal_date,
            firmware_version: v.firmware_version,
            beschreibung: v.description,
            messlokations_id: v.metering_location_id,
        }
    }
}
impl From<Geraet> for bo4e_core::bo::Device {
    fn from(v: Geraet) -> Self {
        Self {
            meta: v.meta,
            device_id: v.geraetkennung,
            serial_number: v.seriennummer,
            device_category: v.geraeteklasse.map(Into::into),
            device_type: v.geraetetyp.map(Into::into),
            manufacturer: v.hersteller,
            model: v.modellbezeichnung,
            manufacturing_year: v.baujahr,
            installation_date: v.einbaudatum,
            removal_date: v.ausbaudatum,
            firmware_version: v.firmware_version,
            description: v.beschreibung,
            metering_location_id: v.messlokations_id,
        }
    }
}
