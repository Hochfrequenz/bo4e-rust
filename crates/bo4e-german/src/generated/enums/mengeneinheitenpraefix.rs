#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Mengeneinheitenpraefix {
    #[serde(rename = "EXA")]
    Exa,
    #[serde(rename = "PETA")]
    Peta,
    #[serde(rename = "TERA")]
    Tera,
    #[serde(rename = "GIGA")]
    Giga,
    #[serde(rename = "MEGA")]
    Mega,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "HEKTO")]
    Hecto,
    #[serde(rename = "DEKA")]
    Deca,
    #[serde(rename = "OHNE")]
    None,
    #[serde(rename = "DEZI")]
    Deci,
    #[serde(rename = "ZENTI")]
    Centi,
    #[serde(rename = "MILLI")]
    Milli,
    #[serde(rename = "MIKRO")]
    Micro,
    #[serde(rename = "NANO")]
    Nano,
    #[serde(rename = "PIKO")]
    Pico,
    #[serde(rename = "FEMTO")]
    Femto,
    #[serde(rename = "ATTO")]
    Atto,
}
impl From<bo4e_core::enums::UnitPrefix> for Mengeneinheitenpraefix {
    fn from(v: bo4e_core::enums::UnitPrefix) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::UnitPrefix::Exa => Mengeneinheitenpraefix::Exa,
            bo4e_core::enums::UnitPrefix::Peta => Mengeneinheitenpraefix::Peta,
            bo4e_core::enums::UnitPrefix::Tera => Mengeneinheitenpraefix::Tera,
            bo4e_core::enums::UnitPrefix::Giga => Mengeneinheitenpraefix::Giga,
            bo4e_core::enums::UnitPrefix::Mega => Mengeneinheitenpraefix::Mega,
            bo4e_core::enums::UnitPrefix::Kilo => Mengeneinheitenpraefix::Kilo,
            bo4e_core::enums::UnitPrefix::Hecto => Mengeneinheitenpraefix::Hecto,
            bo4e_core::enums::UnitPrefix::Deca => Mengeneinheitenpraefix::Deca,
            bo4e_core::enums::UnitPrefix::None => Mengeneinheitenpraefix::None,
            bo4e_core::enums::UnitPrefix::Deci => Mengeneinheitenpraefix::Deci,
            bo4e_core::enums::UnitPrefix::Centi => Mengeneinheitenpraefix::Centi,
            bo4e_core::enums::UnitPrefix::Milli => Mengeneinheitenpraefix::Milli,
            bo4e_core::enums::UnitPrefix::Micro => Mengeneinheitenpraefix::Micro,
            bo4e_core::enums::UnitPrefix::Nano => Mengeneinheitenpraefix::Nano,
            bo4e_core::enums::UnitPrefix::Pico => Mengeneinheitenpraefix::Pico,
            bo4e_core::enums::UnitPrefix::Femto => Mengeneinheitenpraefix::Femto,
            bo4e_core::enums::UnitPrefix::Atto => Mengeneinheitenpraefix::Atto,
            _ => panic!("Unknown {} variant", stringify!(UnitPrefix)),
        }
    }
}
impl From<Mengeneinheitenpraefix> for bo4e_core::enums::UnitPrefix {
    fn from(v: Mengeneinheitenpraefix) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Mengeneinheitenpraefix::Exa => bo4e_core::enums::UnitPrefix::Exa,
            Mengeneinheitenpraefix::Peta => bo4e_core::enums::UnitPrefix::Peta,
            Mengeneinheitenpraefix::Tera => bo4e_core::enums::UnitPrefix::Tera,
            Mengeneinheitenpraefix::Giga => bo4e_core::enums::UnitPrefix::Giga,
            Mengeneinheitenpraefix::Mega => bo4e_core::enums::UnitPrefix::Mega,
            Mengeneinheitenpraefix::Kilo => bo4e_core::enums::UnitPrefix::Kilo,
            Mengeneinheitenpraefix::Hecto => bo4e_core::enums::UnitPrefix::Hecto,
            Mengeneinheitenpraefix::Deca => bo4e_core::enums::UnitPrefix::Deca,
            Mengeneinheitenpraefix::None => bo4e_core::enums::UnitPrefix::None,
            Mengeneinheitenpraefix::Deci => bo4e_core::enums::UnitPrefix::Deci,
            Mengeneinheitenpraefix::Centi => bo4e_core::enums::UnitPrefix::Centi,
            Mengeneinheitenpraefix::Milli => bo4e_core::enums::UnitPrefix::Milli,
            Mengeneinheitenpraefix::Micro => bo4e_core::enums::UnitPrefix::Micro,
            Mengeneinheitenpraefix::Nano => bo4e_core::enums::UnitPrefix::Nano,
            Mengeneinheitenpraefix::Pico => bo4e_core::enums::UnitPrefix::Pico,
            Mengeneinheitenpraefix::Femto => bo4e_core::enums::UnitPrefix::Femto,
            Mengeneinheitenpraefix::Atto => bo4e_core::enums::UnitPrefix::Atto,
        }
    }
}
