#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Netzebene {
    #[serde(rename = "NSP")]
    LowVoltage,
    #[serde(rename = "MSP")]
    MediumVoltage,
    #[serde(rename = "HSP")]
    HighVoltage,
    #[serde(rename = "HSS")]
    ExtraHighVoltage,
    #[serde(rename = "MSP_NSP_UMSP")]
    MediumLowVoltageTransformation,
    #[serde(rename = "HSP_MSP_UMSP")]
    HighMediumVoltageTransformation,
    #[serde(rename = "HSS_HSP_UMSP")]
    ExtraHighHighVoltageTransformation,
    #[serde(rename = "HD")]
    HighPressure,
    #[serde(rename = "MD")]
    MediumPressure,
    #[serde(rename = "ND")]
    LowPressure,
}
impl From<bo4e_core::enums::NetworkLevel> for Netzebene {
    fn from(v: bo4e_core::enums::NetworkLevel) -> Self {
        match v {
            bo4e_core::enums::NetworkLevel::LowVoltage => Netzebene::LowVoltage,
            bo4e_core::enums::NetworkLevel::MediumVoltage => Netzebene::MediumVoltage,
            bo4e_core::enums::NetworkLevel::HighVoltage => Netzebene::HighVoltage,
            bo4e_core::enums::NetworkLevel::ExtraHighVoltage => {
                Netzebene::ExtraHighVoltage
            }
            bo4e_core::enums::NetworkLevel::MediumLowVoltageTransformation => {
                Netzebene::MediumLowVoltageTransformation
            }
            bo4e_core::enums::NetworkLevel::HighMediumVoltageTransformation => {
                Netzebene::HighMediumVoltageTransformation
            }
            bo4e_core::enums::NetworkLevel::ExtraHighHighVoltageTransformation => {
                Netzebene::ExtraHighHighVoltageTransformation
            }
            bo4e_core::enums::NetworkLevel::HighPressure => Netzebene::HighPressure,
            bo4e_core::enums::NetworkLevel::MediumPressure => Netzebene::MediumPressure,
            bo4e_core::enums::NetworkLevel::LowPressure => Netzebene::LowPressure,
            _ => panic!("Unknown {} variant", stringify!(NetworkLevel)),
        }
    }
}
impl From<Netzebene> for bo4e_core::enums::NetworkLevel {
    fn from(v: Netzebene) -> Self {
        match v {
            Netzebene::LowVoltage => bo4e_core::enums::NetworkLevel::LowVoltage,
            Netzebene::MediumVoltage => bo4e_core::enums::NetworkLevel::MediumVoltage,
            Netzebene::HighVoltage => bo4e_core::enums::NetworkLevel::HighVoltage,
            Netzebene::ExtraHighVoltage => {
                bo4e_core::enums::NetworkLevel::ExtraHighVoltage
            }
            Netzebene::MediumLowVoltageTransformation => {
                bo4e_core::enums::NetworkLevel::MediumLowVoltageTransformation
            }
            Netzebene::HighMediumVoltageTransformation => {
                bo4e_core::enums::NetworkLevel::HighMediumVoltageTransformation
            }
            Netzebene::ExtraHighHighVoltageTransformation => {
                bo4e_core::enums::NetworkLevel::ExtraHighHighVoltageTransformation
            }
            Netzebene::HighPressure => bo4e_core::enums::NetworkLevel::HighPressure,
            Netzebene::MediumPressure => bo4e_core::enums::NetworkLevel::MediumPressure,
            Netzebene::LowPressure => bo4e_core::enums::NetworkLevel::LowPressure,
            _ => panic!("Unknown {} variant", stringify!(Netzebene)),
        }
    }
}
