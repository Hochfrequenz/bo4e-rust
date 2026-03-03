#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marktlokation {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyDirection")]
    pub energierichtung: Option<crate::Energierichtung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customerType")]
    pub kundentyp: Option<crate::Kundentyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "supplyStart")]
    pub lieferbeginn: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "supplyEnd")]
    pub lieferende: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "annualConsumption")]
    pub jahresverbrauchsprognose: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkOperatorCode")]
    pub netzbetreiber_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "basicSupplierCode")]
    pub grundversorger_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringOperatorCode")]
    pub messstellenbetreiber_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "transmissionOperatorCode")]
    pub uebertragungsnetzbetreiber_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "gridLevel")]
    pub netzebene: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkArea")]
    pub netzgebiet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "balancingArea")]
    pub bilanzierungsgebiet: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "meteringLocationIds"
    )]
    pub messlokations_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isControllableResource")]
    pub ist_steuerbare_ressource: Option<bool>,
}
impl From<bo4e_core::bo::MarketLocation> for Marktlokation {
    fn from(v: bo4e_core::bo::MarketLocation) -> Self {
        Self {
            meta: v.meta,
            marktlokations_id: v.market_location_id,
            sparte: v.division.map(Into::into),
            energierichtung: v.energy_direction.map(Into::into),
            kundentyp: v.customer_type.map(Into::into),
            adresse: v.address.map(Into::into),
            lieferbeginn: v.supply_start,
            lieferende: v.supply_end,
            jahresverbrauchsprognose: v.annual_consumption,
            netzbetreiber_codenummer: v.network_operator_code,
            grundversorger_codenummer: v.basic_supplier_code,
            messstellenbetreiber_codenummer: v.metering_operator_code,
            uebertragungsnetzbetreiber_codenummer: v.transmission_operator_code,
            netzebene: v.grid_level,
            netzgebiet: v.network_area,
            bilanzierungsgebiet: v.balancing_area,
            messlokations_ids: v.metering_location_ids,
            ist_steuerbare_ressource: v.is_controllable_resource,
        }
    }
}
impl From<Marktlokation> for bo4e_core::bo::MarketLocation {
    fn from(v: Marktlokation) -> Self {
        Self {
            meta: v.meta,
            market_location_id: v.marktlokations_id,
            division: v.sparte.map(Into::into),
            energy_direction: v.energierichtung.map(Into::into),
            customer_type: v.kundentyp.map(Into::into),
            address: v.adresse.map(Into::into),
            supply_start: v.lieferbeginn,
            supply_end: v.lieferende,
            annual_consumption: v.jahresverbrauchsprognose,
            network_operator_code: v.netzbetreiber_codenummer,
            basic_supplier_code: v.grundversorger_codenummer,
            metering_operator_code: v.messstellenbetreiber_codenummer,
            transmission_operator_code: v.uebertragungsnetzbetreiber_codenummer,
            grid_level: v.netzebene,
            network_area: v.netzgebiet,
            balancing_area: v.bilanzierungsgebiet,
            metering_location_ids: v.messlokations_ids,
            is_controllable_resource: v.ist_steuerbare_ressource,
        }
    }
}
