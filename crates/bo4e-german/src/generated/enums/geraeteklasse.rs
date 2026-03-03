#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Geraeteklasse {
    #[serde(rename = "WANDLER")]
    Wandler,
    #[serde(rename = "KOMMUNIKATIONSEINRICHTUNG")]
    Kommunikationseinrichtung,
    #[serde(rename = "TECHNISCHE_STEUEREINRICHTUNG")]
    TechnischeSteuereinrichtung,
    #[serde(rename = "MENGENUMWERTER")]
    Mengenumwerter,
    #[serde(rename = "SMARTMETER_GATEWAY")]
    SmartMeterGateway,
    #[serde(rename = "STEUERBOX")]
    Steuerbox,
    #[serde(rename = "ZAEHLEINRICHTUNG")]
    ZHleinrichtung,
}
impl From<bo4e_core::enums::DeviceCategory> for Geraeteklasse {
    fn from(v: bo4e_core::enums::DeviceCategory) -> Self {
        match v {
            bo4e_core::enums::DeviceCategory::Transformer => Geraeteklasse::Wandler,
            bo4e_core::enums::DeviceCategory::CommunicationEquipment => {
                Geraeteklasse::Kommunikationseinrichtung
            }
            bo4e_core::enums::DeviceCategory::TechnicalControlEquipment => {
                Geraeteklasse::TechnischeSteuereinrichtung
            }
            bo4e_core::enums::DeviceCategory::VolumeConverter => {
                Geraeteklasse::Mengenumwerter
            }
            bo4e_core::enums::DeviceCategory::SmartMeterGateway => {
                Geraeteklasse::SmartMeterGateway
            }
            bo4e_core::enums::DeviceCategory::ControlBox => Geraeteklasse::Steuerbox,
            bo4e_core::enums::DeviceCategory::MeteringDevice => {
                Geraeteklasse::ZHleinrichtung
            }
            _ => panic!("Unknown {} variant", stringify!(DeviceCategory)),
        }
    }
}
impl From<Geraeteklasse> for bo4e_core::enums::DeviceCategory {
    fn from(v: Geraeteklasse) -> Self {
        match v {
            Geraeteklasse::Wandler => bo4e_core::enums::DeviceCategory::Transformer,
            Geraeteklasse::Kommunikationseinrichtung => {
                bo4e_core::enums::DeviceCategory::CommunicationEquipment
            }
            Geraeteklasse::TechnischeSteuereinrichtung => {
                bo4e_core::enums::DeviceCategory::TechnicalControlEquipment
            }
            Geraeteklasse::Mengenumwerter => {
                bo4e_core::enums::DeviceCategory::VolumeConverter
            }
            Geraeteklasse::SmartMeterGateway => {
                bo4e_core::enums::DeviceCategory::SmartMeterGateway
            }
            Geraeteklasse::Steuerbox => bo4e_core::enums::DeviceCategory::ControlBox,
            Geraeteklasse::ZHleinrichtung => {
                bo4e_core::enums::DeviceCategory::MeteringDevice
            }
            _ => panic!("Unknown {} variant", stringify!(Geraeteklasse)),
        }
    }
}
