#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Messart {
    #[serde(rename = "AKTUELLERWERT")]
    AktuellerWert,
    #[serde(rename = "MITTELWERT")]
    Mittelwert,
    #[serde(rename = "MAXIMALWERT")]
    Maximalwert,
}
impl From<bo4e_core::enums::MeasurementType> for Messart {
    fn from(v: bo4e_core::enums::MeasurementType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::MeasurementType::CurrentValue => Messart::AktuellerWert,
            bo4e_core::enums::MeasurementType::MeanValue => Messart::Mittelwert,
            bo4e_core::enums::MeasurementType::MaximumValue => Messart::Maximalwert,
            _ => panic!("Unknown {} variant", stringify!(MeasurementType)),
        }
    }
}
impl From<Messart> for bo4e_core::enums::MeasurementType {
    fn from(v: Messart) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Messart::AktuellerWert => bo4e_core::enums::MeasurementType::CurrentValue,
            Messart::Mittelwert => bo4e_core::enums::MeasurementType::MeanValue,
            Messart::Maximalwert => bo4e_core::enums::MeasurementType::MaximumValue,
        }
    }
}
