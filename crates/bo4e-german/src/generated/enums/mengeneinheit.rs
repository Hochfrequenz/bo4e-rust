#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Mengeneinheit {
    #[serde(rename = "W")]
    Watt,
    #[serde(rename = "KW")]
    Kilowatt,
    #[serde(rename = "MW")]
    Megawatt,
    #[serde(rename = "WH")]
    WattHour,
    #[serde(rename = "KWH")]
    KilowattHour,
    #[serde(rename = "MWH")]
    MegawattHour,
    #[serde(rename = "VAR")]
    VoltAmpereReactive,
    #[serde(rename = "KVAR")]
    KilovoltAmpereReactive,
    #[serde(rename = "VARH")]
    VoltAmpereReactiveHour,
    #[serde(rename = "KVARH")]
    KilovoltAmpereReactiveHour,
    #[serde(rename = "KUBIKMETER")]
    ForGas,
    #[serde(rename = "STUECK")]
    Piece,
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
    #[serde(rename = "PROZENT")]
    Percent,
    #[serde(rename = "KWHK")]
    KilowattHourPerKelvin,
}
impl From<bo4e_core::enums::Unit> for Mengeneinheit {
    fn from(v: bo4e_core::enums::Unit) -> Self {
        match v {
            bo4e_core::enums::Unit::Watt => Mengeneinheit::Watt,
            bo4e_core::enums::Unit::Kilowatt => Mengeneinheit::Kilowatt,
            bo4e_core::enums::Unit::Megawatt => Mengeneinheit::Megawatt,
            bo4e_core::enums::Unit::WattHour => Mengeneinheit::WattHour,
            bo4e_core::enums::Unit::KilowattHour => Mengeneinheit::KilowattHour,
            bo4e_core::enums::Unit::MegawattHour => Mengeneinheit::MegawattHour,
            bo4e_core::enums::Unit::VoltAmpereReactive => {
                Mengeneinheit::VoltAmpereReactive
            }
            bo4e_core::enums::Unit::KilovoltAmpereReactive => {
                Mengeneinheit::KilovoltAmpereReactive
            }
            bo4e_core::enums::Unit::VoltAmpereReactiveHour => {
                Mengeneinheit::VoltAmpereReactiveHour
            }
            bo4e_core::enums::Unit::KilovoltAmpereReactiveHour => {
                Mengeneinheit::KilovoltAmpereReactiveHour
            }
            bo4e_core::enums::Unit::CubicMeter => Mengeneinheit::ForGas,
            bo4e_core::enums::Unit::Piece => Mengeneinheit::Piece,
            bo4e_core::enums::Unit::Second => Mengeneinheit::Second,
            bo4e_core::enums::Unit::Minute => Mengeneinheit::Minute,
            bo4e_core::enums::Unit::Hour => Mengeneinheit::Hour,
            bo4e_core::enums::Unit::QuarterHour => Mengeneinheit::QuarterHour,
            bo4e_core::enums::Unit::Day => Mengeneinheit::Day,
            bo4e_core::enums::Unit::Week => Mengeneinheit::Week,
            bo4e_core::enums::Unit::Month => Mengeneinheit::Month,
            bo4e_core::enums::Unit::Quarter => Mengeneinheit::Quarter,
            bo4e_core::enums::Unit::HalfYear => Mengeneinheit::HalfYear,
            bo4e_core::enums::Unit::Year => Mengeneinheit::Year,
            bo4e_core::enums::Unit::Percent => Mengeneinheit::Percent,
            bo4e_core::enums::Unit::KilowattHourPerKelvin => {
                Mengeneinheit::KilowattHourPerKelvin
            }
            _ => panic!("Unknown {} variant", stringify!(Unit)),
        }
    }
}
impl From<Mengeneinheit> for bo4e_core::enums::Unit {
    fn from(v: Mengeneinheit) -> Self {
        match v {
            Mengeneinheit::Watt => bo4e_core::enums::Unit::Watt,
            Mengeneinheit::Kilowatt => bo4e_core::enums::Unit::Kilowatt,
            Mengeneinheit::Megawatt => bo4e_core::enums::Unit::Megawatt,
            Mengeneinheit::WattHour => bo4e_core::enums::Unit::WattHour,
            Mengeneinheit::KilowattHour => bo4e_core::enums::Unit::KilowattHour,
            Mengeneinheit::MegawattHour => bo4e_core::enums::Unit::MegawattHour,
            Mengeneinheit::VoltAmpereReactive => {
                bo4e_core::enums::Unit::VoltAmpereReactive
            }
            Mengeneinheit::KilovoltAmpereReactive => {
                bo4e_core::enums::Unit::KilovoltAmpereReactive
            }
            Mengeneinheit::VoltAmpereReactiveHour => {
                bo4e_core::enums::Unit::VoltAmpereReactiveHour
            }
            Mengeneinheit::KilovoltAmpereReactiveHour => {
                bo4e_core::enums::Unit::KilovoltAmpereReactiveHour
            }
            Mengeneinheit::ForGas => bo4e_core::enums::Unit::CubicMeter,
            Mengeneinheit::Piece => bo4e_core::enums::Unit::Piece,
            Mengeneinheit::Second => bo4e_core::enums::Unit::Second,
            Mengeneinheit::Minute => bo4e_core::enums::Unit::Minute,
            Mengeneinheit::Hour => bo4e_core::enums::Unit::Hour,
            Mengeneinheit::QuarterHour => bo4e_core::enums::Unit::QuarterHour,
            Mengeneinheit::Day => bo4e_core::enums::Unit::Day,
            Mengeneinheit::Week => bo4e_core::enums::Unit::Week,
            Mengeneinheit::Month => bo4e_core::enums::Unit::Month,
            Mengeneinheit::Quarter => bo4e_core::enums::Unit::Quarter,
            Mengeneinheit::HalfYear => bo4e_core::enums::Unit::HalfYear,
            Mengeneinheit::Year => bo4e_core::enums::Unit::Year,
            Mengeneinheit::Percent => bo4e_core::enums::Unit::Percent,
            Mengeneinheit::KilowattHourPerKelvin => {
                bo4e_core::enums::Unit::KilowattHourPerKelvin
            }
            _ => panic!("Unknown {} variant", stringify!(Mengeneinheit)),
        }
    }
}
