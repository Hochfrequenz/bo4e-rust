#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnischeRessource {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "technicalResourceId")]
    pub technische_ressource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "usage")]
    pub verwendungszweck: Option<crate::TechnischeRessourceNutzung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyDirection")]
    pub energierichtung: Option<crate::Energierichtung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub standort: Option<crate::Adresse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "nominalPower")]
    pub nennleistung: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "maxPower")]
    pub maximalleistung: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "minPower")]
    pub minimalleistung: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyCapacity")]
    pub speicherkapazitaet: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "commissioningDate")]
    pub inbetriebnahmedatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "decommissioningDate")]
    pub stilllegungsdatum: Option<chrono::DateTime<chrono::Utc>>,
}
impl From<bo4e_core::bo::TechnicalResource> for TechnischeRessource {
    fn from(v: bo4e_core::bo::TechnicalResource) -> Self {
        Self {
            meta: v.meta,
            technische_ressource_id: v.technical_resource_id,
            sparte: v.division.map(Into::into),
            verwendungszweck: v.usage.map(Into::into),
            energierichtung: v.energy_direction.map(Into::into),
            standort: v.address.map(Into::into),
            beschreibung: v.description,
            nennleistung: v.nominal_power,
            maximalleistung: v.max_power,
            minimalleistung: v.min_power,
            speicherkapazitaet: v.energy_capacity,
            messlokations_id: v.metering_location_id,
            marktlokations_id: v.market_location_id,
            inbetriebnahmedatum: v.commissioning_date,
            stilllegungsdatum: v.decommissioning_date,
        }
    }
}
impl From<TechnischeRessource> for bo4e_core::bo::TechnicalResource {
    fn from(v: TechnischeRessource) -> Self {
        Self {
            meta: v.meta,
            technical_resource_id: v.technische_ressource_id,
            division: v.sparte.map(Into::into),
            usage: v.verwendungszweck.map(Into::into),
            energy_direction: v.energierichtung.map(Into::into),
            address: v.standort.map(Into::into),
            description: v.beschreibung,
            nominal_power: v.nennleistung,
            max_power: v.maximalleistung,
            min_power: v.minimalleistung,
            energy_capacity: v.speicherkapazitaet,
            metering_location_id: v.messlokations_id,
            market_location_id: v.marktlokations_id,
            commissioning_date: v.inbetriebnahmedatum,
            decommissioning_date: v.stilllegungsdatum,
        }
    }
}
