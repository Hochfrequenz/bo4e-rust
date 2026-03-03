#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Anrede {
    #[serde(rename = "HERR")]
    Herr,
    #[serde(rename = "FRAU")]
    Frau,
    #[serde(rename = "EHELEUTE")]
    Eheleute,
    #[serde(rename = "FIRMA")]
    Firma,
    #[serde(rename = "FAMILIE")]
    Familie,
    #[serde(rename = "ERBENGEMEINSCHAFT")]
    Erbengemeinschaft,
    #[serde(rename = "GRUNDSTUECKSGEMEINSCHAFT")]
    Grundstuecksgemeinschaft,
}
impl From<bo4e_core::enums::Salutation> for Anrede {
    fn from(v: bo4e_core::enums::Salutation) -> Self {
        match v {
            bo4e_core::enums::Salutation::Mr => Anrede::Herr,
            bo4e_core::enums::Salutation::Ms => Anrede::Frau,
            bo4e_core::enums::Salutation::MarriedCouple => Anrede::Eheleute,
            bo4e_core::enums::Salutation::Company => Anrede::Firma,
            bo4e_core::enums::Salutation::Family => Anrede::Familie,
            bo4e_core::enums::Salutation::HeirsCommunity => Anrede::Erbengemeinschaft,
            bo4e_core::enums::Salutation::PropertyCommunity => {
                Anrede::Grundstuecksgemeinschaft
            }
            _ => panic!("Unknown {} variant", stringify!(Salutation)),
        }
    }
}
impl From<Anrede> for bo4e_core::enums::Salutation {
    fn from(v: Anrede) -> Self {
        match v {
            Anrede::Herr => bo4e_core::enums::Salutation::Mr,
            Anrede::Frau => bo4e_core::enums::Salutation::Ms,
            Anrede::Eheleute => bo4e_core::enums::Salutation::MarriedCouple,
            Anrede::Firma => bo4e_core::enums::Salutation::Company,
            Anrede::Familie => bo4e_core::enums::Salutation::Family,
            Anrede::Erbengemeinschaft => bo4e_core::enums::Salutation::HeirsCommunity,
            Anrede::Grundstuecksgemeinschaft => {
                bo4e_core::enums::Salutation::PropertyCommunity
            }
            _ => panic!("Unknown {} variant", stringify!(Anrede)),
        }
    }
}
