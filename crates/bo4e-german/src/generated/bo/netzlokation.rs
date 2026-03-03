#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Netzlokation {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkLocationId")]
    pub netzlokations_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkLevel")]
    pub netzebene: Option<crate::Netzebene>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "networkOperatorCode")]
    pub netzbetreiber_codenummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "meteringLocationIds"
    )]
    pub messlokations_ids: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "technicalResourceIds"
    )]
    pub technische_ressource_ids: Vec<String>,
}
impl From<bo4e_core::bo::NetworkLocation> for Netzlokation {
    fn from(v: bo4e_core::bo::NetworkLocation) -> Self {
        Self {
            meta: v.meta,
            netzlokations_id: v.network_location_id,
            sparte: v.division.map(Into::into),
            netzebene: v.network_level.map(Into::into),
            adresse: v.address.map(Into::into),
            netzbetreiber_codenummer: v.network_operator_code,
            beschreibung: v.description,
            messlokations_ids: v.metering_location_ids,
            technische_ressource_ids: v.technical_resource_ids,
        }
    }
}
impl From<Netzlokation> for bo4e_core::bo::NetworkLocation {
    fn from(v: Netzlokation) -> Self {
        Self {
            meta: v.meta,
            network_location_id: v.netzlokations_id,
            division: v.sparte.map(Into::into),
            network_level: v.netzebene.map(Into::into),
            address: v.adresse.map(Into::into),
            network_operator_code: v.netzbetreiber_codenummer,
            description: v.beschreibung,
            metering_location_ids: v.messlokations_ids,
            technical_resource_ids: v.technische_ressource_ids,
        }
    }
}
