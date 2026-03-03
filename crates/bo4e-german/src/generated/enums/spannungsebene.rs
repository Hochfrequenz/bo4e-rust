#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Spannungsebene {
    #[serde(rename = "HOECHSTSPANNUNG")]
    ExtraHighVoltage,
    #[serde(rename = "HOCHSPANNUNG")]
    HighVoltage,
    #[serde(rename = "MITTELSPANNUNG")]
    MediumVoltage,
    #[serde(rename = "NIEDERSPANNUNG")]
    LowVoltage,
}
impl From<bo4e_core::enums::VoltageLevel> for Spannungsebene {
    fn from(v: bo4e_core::enums::VoltageLevel) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::VoltageLevel::ExtraHighVoltage => Spannungsebene::ExtraHighVoltage,
            bo4e_core::enums::VoltageLevel::HighVoltage => Spannungsebene::HighVoltage,
            bo4e_core::enums::VoltageLevel::MediumVoltage => Spannungsebene::MediumVoltage,
            bo4e_core::enums::VoltageLevel::LowVoltage => Spannungsebene::LowVoltage,
            _ => panic!("Unknown {} variant", stringify!(VoltageLevel)),
        }
    }
}
impl From<Spannungsebene> for bo4e_core::enums::VoltageLevel {
    fn from(v: Spannungsebene) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Spannungsebene::ExtraHighVoltage => bo4e_core::enums::VoltageLevel::ExtraHighVoltage,
            Spannungsebene::HighVoltage => bo4e_core::enums::VoltageLevel::HighVoltage,
            Spannungsebene::MediumVoltage => bo4e_core::enums::VoltageLevel::MediumVoltage,
            Spannungsebene::LowVoltage => bo4e_core::enums::VoltageLevel::LowVoltage,
        }
    }
}
