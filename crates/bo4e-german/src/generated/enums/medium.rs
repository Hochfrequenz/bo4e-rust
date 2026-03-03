#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Medium {
    #[serde(rename = "STROM")]
    Strom,
    #[serde(rename = "GAS")]
    Gas,
    #[serde(rename = "WASSER")]
    Wasser,
    #[serde(rename = "DAMPF")]
    Dampf,
}
impl From<bo4e_core::enums::Medium> for Medium {
    fn from(v: bo4e_core::enums::Medium) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::Medium::Electricity => Medium::Strom,
            bo4e_core::enums::Medium::Gas => Medium::Gas,
            bo4e_core::enums::Medium::Water => Medium::Wasser,
            bo4e_core::enums::Medium::Steam => Medium::Dampf,
            _ => panic!("Unknown {} variant", stringify!(Medium)),
        }
    }
}
impl From<Medium> for bo4e_core::enums::Medium {
    fn from(v: Medium) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Medium::Strom => bo4e_core::enums::Medium::Electricity,
            Medium::Gas => bo4e_core::enums::Medium::Gas,
            Medium::Wasser => bo4e_core::enums::Medium::Water,
            Medium::Dampf => bo4e_core::enums::Medium::Steam,
        }
    }
}
