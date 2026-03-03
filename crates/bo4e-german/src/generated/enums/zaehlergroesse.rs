#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Zaehlergroesse {
    #[serde(rename = "G2KOMMA5")]
    G25,
    #[serde(rename = "G4")]
    G4,
    #[serde(rename = "G6")]
    G6,
    #[serde(rename = "G10")]
    G10,
    #[serde(rename = "G16")]
    G16,
    #[serde(rename = "G40")]
    G40,
    #[serde(rename = "G65")]
    G65,
    #[serde(rename = "G100")]
    G100,
    #[serde(rename = "G160")]
    G160,
    #[serde(rename = "G250")]
    G250,
    #[serde(rename = "G400")]
    G400,
    #[serde(rename = "G650")]
    G650,
    #[serde(rename = "G1000")]
    G1000,
    #[serde(rename = "G1600")]
    G1600,
    #[serde(rename = "G2500")]
    G2500,
    #[serde(rename = "G4000")]
    G4000,
    #[serde(rename = "G6500")]
    G6500,
    #[serde(rename = "G10000")]
    G10000,
    #[serde(rename = "G12500")]
    G12500,
    #[serde(rename = "G16000")]
    G16000,
}
impl From<bo4e_core::enums::MeterSize> for Zaehlergroesse {
    fn from(v: bo4e_core::enums::MeterSize) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::MeterSize::G2_5 => Zaehlergroesse::G25,
            bo4e_core::enums::MeterSize::G4 => Zaehlergroesse::G4,
            bo4e_core::enums::MeterSize::G6 => Zaehlergroesse::G6,
            bo4e_core::enums::MeterSize::G10 => Zaehlergroesse::G10,
            bo4e_core::enums::MeterSize::G16 => Zaehlergroesse::G16,
            bo4e_core::enums::MeterSize::G25 => Zaehlergroesse::G25,
            bo4e_core::enums::MeterSize::G40 => Zaehlergroesse::G40,
            bo4e_core::enums::MeterSize::G65 => Zaehlergroesse::G65,
            bo4e_core::enums::MeterSize::G100 => Zaehlergroesse::G100,
            bo4e_core::enums::MeterSize::G160 => Zaehlergroesse::G160,
            bo4e_core::enums::MeterSize::G250 => Zaehlergroesse::G250,
            bo4e_core::enums::MeterSize::G400 => Zaehlergroesse::G400,
            bo4e_core::enums::MeterSize::G650 => Zaehlergroesse::G650,
            bo4e_core::enums::MeterSize::G1000 => Zaehlergroesse::G1000,
            bo4e_core::enums::MeterSize::G1600 => Zaehlergroesse::G1600,
            bo4e_core::enums::MeterSize::G2500 => Zaehlergroesse::G2500,
            bo4e_core::enums::MeterSize::G4000 => Zaehlergroesse::G4000,
            bo4e_core::enums::MeterSize::G6500 => Zaehlergroesse::G6500,
            bo4e_core::enums::MeterSize::G10000 => Zaehlergroesse::G10000,
            bo4e_core::enums::MeterSize::G12500 => Zaehlergroesse::G12500,
            bo4e_core::enums::MeterSize::G16000 => Zaehlergroesse::G16000,
            _ => panic!("Unknown {} variant", stringify!(MeterSize)),
        }
    }
}
impl From<Zaehlergroesse> for bo4e_core::enums::MeterSize {
    fn from(v: Zaehlergroesse) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Zaehlergroesse::G25 => bo4e_core::enums::MeterSize::G2_5,
            Zaehlergroesse::G4 => bo4e_core::enums::MeterSize::G4,
            Zaehlergroesse::G6 => bo4e_core::enums::MeterSize::G6,
            Zaehlergroesse::G10 => bo4e_core::enums::MeterSize::G10,
            Zaehlergroesse::G16 => bo4e_core::enums::MeterSize::G16,
            Zaehlergroesse::G40 => bo4e_core::enums::MeterSize::G40,
            Zaehlergroesse::G65 => bo4e_core::enums::MeterSize::G65,
            Zaehlergroesse::G100 => bo4e_core::enums::MeterSize::G100,
            Zaehlergroesse::G160 => bo4e_core::enums::MeterSize::G160,
            Zaehlergroesse::G250 => bo4e_core::enums::MeterSize::G250,
            Zaehlergroesse::G400 => bo4e_core::enums::MeterSize::G400,
            Zaehlergroesse::G650 => bo4e_core::enums::MeterSize::G650,
            Zaehlergroesse::G1000 => bo4e_core::enums::MeterSize::G1000,
            Zaehlergroesse::G1600 => bo4e_core::enums::MeterSize::G1600,
            Zaehlergroesse::G2500 => bo4e_core::enums::MeterSize::G2500,
            Zaehlergroesse::G4000 => bo4e_core::enums::MeterSize::G4000,
            Zaehlergroesse::G6500 => bo4e_core::enums::MeterSize::G6500,
            Zaehlergroesse::G10000 => bo4e_core::enums::MeterSize::G10000,
            Zaehlergroesse::G12500 => bo4e_core::enums::MeterSize::G12500,
            Zaehlergroesse::G16000 => bo4e_core::enums::MeterSize::G16000,
        }
    }
}
