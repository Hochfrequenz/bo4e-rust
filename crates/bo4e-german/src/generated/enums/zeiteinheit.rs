#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Zeiteinheit {
    #[serde(rename = "SEKUNDE")]
    Second,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "STUNDE")]
    Hour,
    #[serde(rename = "VIERTEL_STUNDE")]
    QuarterHour,
    #[serde(rename = "TAG")]
    Day,
    #[serde(rename = "WOCHE")]
    Week,
    #[serde(rename = "MONAT")]
    Month,
    #[serde(rename = "QUARTAL")]
    Quarter,
    #[serde(rename = "HALBJAHR")]
    HalfYear,
    #[serde(rename = "JAHR")]
    Year,
}
impl From<bo4e_core::enums::TimeUnit> for Zeiteinheit {
    fn from(v: bo4e_core::enums::TimeUnit) -> Self {
        match v {
            bo4e_core::enums::TimeUnit::Second => Zeiteinheit::Second,
            bo4e_core::enums::TimeUnit::Minute => Zeiteinheit::Minute,
            bo4e_core::enums::TimeUnit::Hour => Zeiteinheit::Hour,
            bo4e_core::enums::TimeUnit::QuarterHour => Zeiteinheit::QuarterHour,
            bo4e_core::enums::TimeUnit::Day => Zeiteinheit::Day,
            bo4e_core::enums::TimeUnit::Week => Zeiteinheit::Week,
            bo4e_core::enums::TimeUnit::Month => Zeiteinheit::Month,
            bo4e_core::enums::TimeUnit::Quarter => Zeiteinheit::Quarter,
            bo4e_core::enums::TimeUnit::HalfYear => Zeiteinheit::HalfYear,
            bo4e_core::enums::TimeUnit::Year => Zeiteinheit::Year,
            _ => panic!("Unknown {} variant", stringify!(TimeUnit)),
        }
    }
}
impl From<Zeiteinheit> for bo4e_core::enums::TimeUnit {
    fn from(v: Zeiteinheit) -> Self {
        match v {
            Zeiteinheit::Second => bo4e_core::enums::TimeUnit::Second,
            Zeiteinheit::Minute => bo4e_core::enums::TimeUnit::Minute,
            Zeiteinheit::Hour => bo4e_core::enums::TimeUnit::Hour,
            Zeiteinheit::QuarterHour => bo4e_core::enums::TimeUnit::QuarterHour,
            Zeiteinheit::Day => bo4e_core::enums::TimeUnit::Day,
            Zeiteinheit::Week => bo4e_core::enums::TimeUnit::Week,
            Zeiteinheit::Month => bo4e_core::enums::TimeUnit::Month,
            Zeiteinheit::Quarter => bo4e_core::enums::TimeUnit::Quarter,
            Zeiteinheit::HalfYear => bo4e_core::enums::TimeUnit::HalfYear,
            Zeiteinheit::Year => bo4e_core::enums::TimeUnit::Year,
            _ => panic!("Unknown {} variant", stringify!(Zeiteinheit)),
        }
    }
}
