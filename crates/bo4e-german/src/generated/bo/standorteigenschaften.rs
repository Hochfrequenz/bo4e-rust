#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standorteigenschaften {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "locationPropertiesId")]
    pub standorteigenschaften_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "coordinates")]
    pub geokoordinaten: Option<crate::Geokoordinaten>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "buildingType")]
    pub gebaeudeart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "constructionYear")]
    pub baujahr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "floorArea")]
    pub flaeche: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "numberOfFloors")]
    pub anzahl_etagen: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "numberOfUnits")]
    pub anzahl_wohneinheiten: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "heatingType")]
    pub heizungsart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyEfficiencyClass")]
    pub energieeffizienzklasse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "hasSolar")]
    pub hat_solaranlage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "hasEvCharging")]
    pub hat_e_ladestation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "hasHeatPump")]
    pub hat_waermepumpe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
}
impl From<bo4e_core::bo::LocationProperties> for Standorteigenschaften {
    fn from(v: bo4e_core::bo::LocationProperties) -> Self {
        Self {
            meta: v.meta,
            standorteigenschaften_id: v.location_properties_id,
            adresse: v.address.map(Into::into),
            geokoordinaten: v.coordinates.map(Into::into),
            gebaeudeart: v.building_type,
            baujahr: v.construction_year,
            flaeche: v.floor_area,
            anzahl_etagen: v.number_of_floors,
            anzahl_wohneinheiten: v.number_of_units,
            heizungsart: v.heating_type,
            energieeffizienzklasse: v.energy_efficiency_class,
            hat_solaranlage: v.has_solar,
            hat_e_ladestation: v.has_ev_charging,
            hat_waermepumpe: v.has_heat_pump,
            beschreibung: v.description,
        }
    }
}
impl From<Standorteigenschaften> for bo4e_core::bo::LocationProperties {
    fn from(v: Standorteigenschaften) -> Self {
        Self {
            meta: v.meta,
            location_properties_id: v.standorteigenschaften_id,
            address: v.adresse.map(Into::into),
            coordinates: v.geokoordinaten.map(Into::into),
            building_type: v.gebaeudeart,
            construction_year: v.baujahr,
            floor_area: v.flaeche,
            number_of_floors: v.anzahl_etagen,
            number_of_units: v.anzahl_wohneinheiten,
            heating_type: v.heizungsart,
            energy_efficiency_class: v.energieeffizienzklasse,
            has_solar: v.hat_solaranlage,
            has_ev_charging: v.hat_e_ladestation,
            has_heat_pump: v.hat_waermepumpe,
            description: v.beschreibung,
        }
    }
}
