#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lokationszuordnung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocationId")]
    pub marktlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meteringLocationId")]
    pub messlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkLocationId")]
    pub netzlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "technicalResourceId")]
    pub technische_ressource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "controllableResourceId")]
    pub steuerbare_ressource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "locationType")]
    pub lokationstyp: Option<crate::Lokationstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "arithmeticOperation")]
    pub rechenoperation: Option<crate::ArithmetischeOperation>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "sequence")]
    pub reihenfolge: Option<i32>,
}
impl From<bo4e_core::bo::LocationAssignment> for Lokationszuordnung {
    fn from(v: bo4e_core::bo::LocationAssignment) -> Self {
        Self {
            meta: v.meta,
            marktlokations_id: v.market_location_id,
            messlokations_id: v.metering_location_id,
            netzlokations_id: v.network_location_id,
            technische_ressource_id: v.technical_resource_id,
            steuerbare_ressource_id: v.controllable_resource_id,
            lokationstyp: v.location_type.map(Into::into),
            rechenoperation: v.arithmetic_operation.map(Into::into),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            reihenfolge: v.sequence,
        }
    }
}
impl From<Lokationszuordnung> for bo4e_core::bo::LocationAssignment {
    fn from(v: Lokationszuordnung) -> Self {
        Self {
            meta: v.meta,
            market_location_id: v.marktlokations_id,
            metering_location_id: v.messlokations_id,
            network_location_id: v.netzlokations_id,
            technical_resource_id: v.technische_ressource_id,
            controllable_resource_id: v.steuerbare_ressource_id,
            location_type: v.lokationstyp.map(Into::into),
            arithmetic_operation: v.rechenoperation.map(Into::into),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            sequence: v.reihenfolge,
        }
    }
}
