#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteuerbareRessource {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "controllableResourceId")]
    pub steuerbare_ressource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "resourceType")]
    pub ressourcentyp: Option<crate::SteuerbareRessourceTyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyDirection")]
    pub energierichtung: Option<crate::Energierichtung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub standort: Option<crate::Adresse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "controllablePower")]
    pub steuerbare_leistung: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "minActivationTime")]
    pub mindestaktivierungszeit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "maxActivationTime")]
    pub maximalaktivierungszeit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "rampUpTime")]
    pub hochlaufzeit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "rampDownTime")]
    pub herunterlaufzeit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "technicalResourceId")]
    pub technische_ressource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isActive")]
    pub ist_aktiv: Option<bool>,
}
impl From<bo4e_core::bo::ControllableResource> for SteuerbareRessource {
    fn from(v: bo4e_core::bo::ControllableResource) -> Self {
        Self {
            meta: v.meta,
            steuerbare_ressource_id: v.controllable_resource_id,
            sparte: v.division.map(Into::into),
            ressourcentyp: v.resource_type.map(Into::into),
            energierichtung: v.energy_direction.map(Into::into),
            standort: v.address.map(Into::into),
            beschreibung: v.description,
            steuerbare_leistung: v.controllable_power,
            mindestaktivierungszeit: v.min_activation_time,
            maximalaktivierungszeit: v.max_activation_time,
            hochlaufzeit: v.ramp_up_time,
            herunterlaufzeit: v.ramp_down_time,
            technische_ressource_id: v.technical_resource_id,
            marktlokations_id: v.market_location_id,
            ist_aktiv: v.is_active,
        }
    }
}
impl From<SteuerbareRessource> for bo4e_core::bo::ControllableResource {
    fn from(v: SteuerbareRessource) -> Self {
        Self {
            meta: v.meta,
            controllable_resource_id: v.steuerbare_ressource_id,
            division: v.sparte.map(Into::into),
            resource_type: v.ressourcentyp.map(Into::into),
            energy_direction: v.energierichtung.map(Into::into),
            address: v.standort.map(Into::into),
            description: v.beschreibung,
            controllable_power: v.steuerbare_leistung,
            min_activation_time: v.mindestaktivierungszeit,
            max_activation_time: v.maximalaktivierungszeit,
            ramp_up_time: v.hochlaufzeit,
            ramp_down_time: v.herunterlaufzeit,
            technical_resource_id: v.technische_ressource_id,
            market_location_id: v.marktlokations_id,
            is_active: v.ist_aktiv,
        }
    }
}
