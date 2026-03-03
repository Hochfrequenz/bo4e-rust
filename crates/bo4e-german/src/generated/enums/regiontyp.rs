#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Regiontyp {
    #[serde(rename = "REGELZONE")]
    Regelzone,
    #[serde(rename = "MARKTGEBIET")]
    Marktgebiet,
    #[serde(rename = "BILANZIERUNGSGEBIET")]
    Bilanzierungsgebiet,
    #[serde(rename = "VERTEILNETZ")]
    Verteilnetz,
    #[serde(rename = "TRANSPORTNETZ")]
    Transportnetz,
    #[serde(rename = "REGIONALNETZ")]
    Regionalnetz,
    #[serde(rename = "AREALNETZ")]
    Arealnetz,
    #[serde(rename = "GRUNDVERSORGUNGSGEBIET")]
    Grundversorgungsgebiet,
    #[serde(rename = "VERSORGUNGSGEBIET")]
    Versorgungsgebiet,
}
impl From<bo4e_core::enums::RegionType> for Regiontyp {
    fn from(v: bo4e_core::enums::RegionType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::RegionType::ControlArea => Regiontyp::Regelzone,
            bo4e_core::enums::RegionType::MarketArea => Regiontyp::Marktgebiet,
            bo4e_core::enums::RegionType::BalancingArea => Regiontyp::Bilanzierungsgebiet,
            bo4e_core::enums::RegionType::DistributionNetwork => Regiontyp::Verteilnetz,
            bo4e_core::enums::RegionType::TransmissionNetwork => Regiontyp::Transportnetz,
            bo4e_core::enums::RegionType::RegionalNetwork => Regiontyp::Regionalnetz,
            bo4e_core::enums::RegionType::AreaNetwork => Regiontyp::Arealnetz,
            bo4e_core::enums::RegionType::BasicSupplyArea => Regiontyp::Grundversorgungsgebiet,
            bo4e_core::enums::RegionType::SupplyArea => Regiontyp::Versorgungsgebiet,
            _ => panic!("Unknown {} variant", stringify!(RegionType)),
        }
    }
}
impl From<Regiontyp> for bo4e_core::enums::RegionType {
    fn from(v: Regiontyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Regiontyp::Regelzone => bo4e_core::enums::RegionType::ControlArea,
            Regiontyp::Marktgebiet => bo4e_core::enums::RegionType::MarketArea,
            Regiontyp::Bilanzierungsgebiet => bo4e_core::enums::RegionType::BalancingArea,
            Regiontyp::Verteilnetz => bo4e_core::enums::RegionType::DistributionNetwork,
            Regiontyp::Transportnetz => bo4e_core::enums::RegionType::TransmissionNetwork,
            Regiontyp::Regionalnetz => bo4e_core::enums::RegionType::RegionalNetwork,
            Regiontyp::Arealnetz => bo4e_core::enums::RegionType::AreaNetwork,
            Regiontyp::Grundversorgungsgebiet => bo4e_core::enums::RegionType::BasicSupplyArea,
            Regiontyp::Versorgungsgebiet => bo4e_core::enums::RegionType::SupplyArea,
        }
    }
}
