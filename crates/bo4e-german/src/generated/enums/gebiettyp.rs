#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Gebiettyp {
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
impl From<bo4e_core::enums::AreaType> for Gebiettyp {
    fn from(v: bo4e_core::enums::AreaType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::AreaType::ControlArea => Gebiettyp::Regelzone,
            bo4e_core::enums::AreaType::MarketArea => Gebiettyp::Marktgebiet,
            bo4e_core::enums::AreaType::BalancingArea => Gebiettyp::Bilanzierungsgebiet,
            bo4e_core::enums::AreaType::DistributionNetwork => Gebiettyp::Verteilnetz,
            bo4e_core::enums::AreaType::TransmissionNetwork => Gebiettyp::Transportnetz,
            bo4e_core::enums::AreaType::RegionalNetwork => Gebiettyp::Regionalnetz,
            bo4e_core::enums::AreaType::ArealNetwork => Gebiettyp::Arealnetz,
            bo4e_core::enums::AreaType::BasicSupplyArea => Gebiettyp::Grundversorgungsgebiet,
            bo4e_core::enums::AreaType::SupplyArea => Gebiettyp::Versorgungsgebiet,
            _ => panic!("Unknown {} variant", stringify!(AreaType)),
        }
    }
}
impl From<Gebiettyp> for bo4e_core::enums::AreaType {
    fn from(v: Gebiettyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Gebiettyp::Regelzone => bo4e_core::enums::AreaType::ControlArea,
            Gebiettyp::Marktgebiet => bo4e_core::enums::AreaType::MarketArea,
            Gebiettyp::Bilanzierungsgebiet => bo4e_core::enums::AreaType::BalancingArea,
            Gebiettyp::Verteilnetz => bo4e_core::enums::AreaType::DistributionNetwork,
            Gebiettyp::Transportnetz => bo4e_core::enums::AreaType::TransmissionNetwork,
            Gebiettyp::Regionalnetz => bo4e_core::enums::AreaType::RegionalNetwork,
            Gebiettyp::Arealnetz => bo4e_core::enums::AreaType::ArealNetwork,
            Gebiettyp::Grundversorgungsgebiet => bo4e_core::enums::AreaType::BasicSupplyArea,
            Gebiettyp::Versorgungsgebiet => bo4e_core::enums::AreaType::SupplyArea,
        }
    }
}
