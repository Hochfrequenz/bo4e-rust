#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Rundungsverfahren {
    #[serde(rename = "KEINE")]
    KeineRundung,
    #[serde(rename = "KAUFMAENNISCH")]
    KaufmNnischeRundung,
    #[serde(rename = "ABRUNDEN")]
    Abrunden,
    #[serde(rename = "AUFRUNDEN")]
    Aufrunden,
}
impl From<bo4e_core::enums::RoundingMode> for Rundungsverfahren {
    fn from(v: bo4e_core::enums::RoundingMode) -> Self {
        match v {
            bo4e_core::enums::RoundingMode::None => Rundungsverfahren::KeineRundung,
            bo4e_core::enums::RoundingMode::Commercial => {
                Rundungsverfahren::KaufmNnischeRundung
            }
            bo4e_core::enums::RoundingMode::Floor => Rundungsverfahren::Abrunden,
            bo4e_core::enums::RoundingMode::Ceiling => Rundungsverfahren::Aufrunden,
            _ => panic!("Unknown {} variant", stringify!(RoundingMode)),
        }
    }
}
impl From<Rundungsverfahren> for bo4e_core::enums::RoundingMode {
    fn from(v: Rundungsverfahren) -> Self {
        match v {
            Rundungsverfahren::KeineRundung => bo4e_core::enums::RoundingMode::None,
            Rundungsverfahren::KaufmNnischeRundung => {
                bo4e_core::enums::RoundingMode::Commercial
            }
            Rundungsverfahren::Abrunden => bo4e_core::enums::RoundingMode::Floor,
            Rundungsverfahren::Aufrunden => bo4e_core::enums::RoundingMode::Ceiling,
            _ => panic!("Unknown {} variant", stringify!(Rundungsverfahren)),
        }
    }
}
