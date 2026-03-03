#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Zaehlerauspraegung {
    #[serde(rename = "EINRICHTUNGSZAEHLER")]
    EinrichtungszHler,
    #[serde(rename = "ZWEIRICHTUNGSZAEHLER")]
    ZweirichtungszHler,
}
impl From<bo4e_core::enums::MeterCategory> for Zaehlerauspraegung {
    fn from(v: bo4e_core::enums::MeterCategory) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::MeterCategory::Unidirectional => {
                Zaehlerauspraegung::EinrichtungszHler
            }
            bo4e_core::enums::MeterCategory::Bidirectional => {
                Zaehlerauspraegung::ZweirichtungszHler
            }
            _ => panic!("Unknown {} variant", stringify!(MeterCategory)),
        }
    }
}
impl From<Zaehlerauspraegung> for bo4e_core::enums::MeterCategory {
    fn from(v: Zaehlerauspraegung) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Zaehlerauspraegung::EinrichtungszHler => {
                bo4e_core::enums::MeterCategory::Unidirectional
            }
            Zaehlerauspraegung::ZweirichtungszHler => {
                bo4e_core::enums::MeterCategory::Bidirectional
            }
        }
    }
}
