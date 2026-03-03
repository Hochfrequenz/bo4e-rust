#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Lokationstyp {
    #[serde(rename = "MALO")]
    Marktlokation,
    #[serde(rename = "MELO")]
    Messlokation,
    #[serde(rename = "NELO")]
    Netzlokation,
    #[serde(rename = "SR")]
    SteuerbareRessource,
    #[serde(rename = "TR")]
    TechnischeRessource,
}
impl From<bo4e_core::enums::LocationType> for Lokationstyp {
    fn from(v: bo4e_core::enums::LocationType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::LocationType::MarketLocation => Lokationstyp::Marktlokation,
            bo4e_core::enums::LocationType::MeteringLocation => Lokationstyp::Messlokation,
            bo4e_core::enums::LocationType::NetworkLocation => Lokationstyp::Netzlokation,
            bo4e_core::enums::LocationType::ControllableResource => {
                Lokationstyp::SteuerbareRessource
            }
            bo4e_core::enums::LocationType::TechnicalResource => Lokationstyp::TechnischeRessource,
            _ => panic!("Unknown {} variant", stringify!(LocationType)),
        }
    }
}
impl From<Lokationstyp> for bo4e_core::enums::LocationType {
    fn from(v: Lokationstyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Lokationstyp::Marktlokation => bo4e_core::enums::LocationType::MarketLocation,
            Lokationstyp::Messlokation => bo4e_core::enums::LocationType::MeteringLocation,
            Lokationstyp::Netzlokation => bo4e_core::enums::LocationType::NetworkLocation,
            Lokationstyp::SteuerbareRessource => {
                bo4e_core::enums::LocationType::ControllableResource
            }
            Lokationstyp::TechnischeRessource => bo4e_core::enums::LocationType::TechnicalResource,
        }
    }
}
