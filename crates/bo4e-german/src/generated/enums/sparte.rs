#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Sparte {
    #[serde(rename = "STROM")]
    Strom,
    #[serde(rename = "GAS")]
    Gas,
    #[serde(rename = "FERNWAERME")]
    Fernwaerme,
    #[serde(rename = "NAHWAERME")]
    Nahwaerme,
    #[serde(rename = "WASSER")]
    Wasser,
    #[serde(rename = "ABWASSER")]
    Abwasser,
    #[serde(rename = "STROM_UND_GAS")]
    StromUndGas,
}
impl From<bo4e_core::enums::Division> for Sparte {
    fn from(v: bo4e_core::enums::Division) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::Division::Electricity => Sparte::Strom,
            bo4e_core::enums::Division::Gas => Sparte::Gas,
            bo4e_core::enums::Division::DistrictHeating => Sparte::Fernwaerme,
            bo4e_core::enums::Division::LocalHeating => Sparte::Nahwaerme,
            bo4e_core::enums::Division::Water => Sparte::Wasser,
            bo4e_core::enums::Division::Wastewater => Sparte::Abwasser,
            bo4e_core::enums::Division::ElectricityAndGas => Sparte::StromUndGas,
            _ => panic!("Unknown {} variant", stringify!(Division)),
        }
    }
}
impl From<Sparte> for bo4e_core::enums::Division {
    fn from(v: Sparte) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Sparte::Strom => bo4e_core::enums::Division::Electricity,
            Sparte::Gas => bo4e_core::enums::Division::Gas,
            Sparte::Fernwaerme => bo4e_core::enums::Division::DistrictHeating,
            Sparte::Nahwaerme => bo4e_core::enums::Division::LocalHeating,
            Sparte::Wasser => bo4e_core::enums::Division::Water,
            Sparte::Abwasser => bo4e_core::enums::Division::Wastewater,
            Sparte::StromUndGas => bo4e_core::enums::Division::ElectricityAndGas,
        }
    }
}
