#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Messgroesse {
    #[serde(rename = "STROM")]
    Strom,
    #[serde(rename = "SPANNUNG")]
    Spannung,
    #[serde(rename = "WIRKLEISTUNG")]
    Wirkleistung,
    #[serde(rename = "BLINDLEISTUNG")]
    Blindleistung,
    #[serde(rename = "DRUCK")]
    Druck,
    #[serde(rename = "LASTGANG")]
    Lastgang,
    #[serde(rename = "LASTPROFIL")]
    Lastprofil,
    #[serde(rename = "TEMPERATUR")]
    Temperatur,
    #[serde(rename = "ZZAHL")]
    Zustandszahl,
    #[serde(rename = "BRENNWERT")]
    Brennwert,
    #[serde(rename = "GRADTZAGSZAHLEN")]
    Gradtagszahlen,
    #[serde(rename = "VOLUMENSTROM")]
    Volumenstrom,
    #[serde(rename = "PREISE")]
    Preise,
}
impl From<bo4e_core::enums::MeasuredQuantity> for Messgroesse {
    fn from(v: bo4e_core::enums::MeasuredQuantity) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::MeasuredQuantity::Current => Messgroesse::Strom,
            bo4e_core::enums::MeasuredQuantity::Voltage => Messgroesse::Spannung,
            bo4e_core::enums::MeasuredQuantity::ActivePower => Messgroesse::Wirkleistung,
            bo4e_core::enums::MeasuredQuantity::ReactivePower => Messgroesse::Blindleistung,
            bo4e_core::enums::MeasuredQuantity::Pressure => Messgroesse::Druck,
            bo4e_core::enums::MeasuredQuantity::LoadProfile => Messgroesse::Lastgang,
            bo4e_core::enums::MeasuredQuantity::StandardLoadProfile => Messgroesse::Lastprofil,
            bo4e_core::enums::MeasuredQuantity::Temperature => Messgroesse::Temperatur,
            bo4e_core::enums::MeasuredQuantity::StateNumber => Messgroesse::Zustandszahl,
            bo4e_core::enums::MeasuredQuantity::CalorificValue => Messgroesse::Brennwert,
            bo4e_core::enums::MeasuredQuantity::DegreeDays => Messgroesse::Gradtagszahlen,
            bo4e_core::enums::MeasuredQuantity::VolumeFlow => Messgroesse::Volumenstrom,
            bo4e_core::enums::MeasuredQuantity::Prices => Messgroesse::Preise,
            _ => panic!("Unknown {} variant", stringify!(MeasuredQuantity)),
        }
    }
}
impl From<Messgroesse> for bo4e_core::enums::MeasuredQuantity {
    fn from(v: Messgroesse) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Messgroesse::Strom => bo4e_core::enums::MeasuredQuantity::Current,
            Messgroesse::Spannung => bo4e_core::enums::MeasuredQuantity::Voltage,
            Messgroesse::Wirkleistung => bo4e_core::enums::MeasuredQuantity::ActivePower,
            Messgroesse::Blindleistung => bo4e_core::enums::MeasuredQuantity::ReactivePower,
            Messgroesse::Druck => bo4e_core::enums::MeasuredQuantity::Pressure,
            Messgroesse::Lastgang => bo4e_core::enums::MeasuredQuantity::LoadProfile,
            Messgroesse::Lastprofil => bo4e_core::enums::MeasuredQuantity::StandardLoadProfile,
            Messgroesse::Temperatur => bo4e_core::enums::MeasuredQuantity::Temperature,
            Messgroesse::Zustandszahl => bo4e_core::enums::MeasuredQuantity::StateNumber,
            Messgroesse::Brennwert => bo4e_core::enums::MeasuredQuantity::CalorificValue,
            Messgroesse::Gradtagszahlen => bo4e_core::enums::MeasuredQuantity::DegreeDays,
            Messgroesse::Volumenstrom => bo4e_core::enums::MeasuredQuantity::VolumeFlow,
            Messgroesse::Preise => bo4e_core::enums::MeasuredQuantity::Prices,
        }
    }
}
