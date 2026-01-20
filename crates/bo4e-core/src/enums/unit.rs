//! Unit of measurement (Mengeneinheit) enumeration.

use serde::{Deserialize, Serialize};

/// Unit of measurement.
///
/// Measurement units that can be determined by measurement or specification.
///
/// German: Mengeneinheit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Unit {
    // Power units
    /// Watt
    #[serde(rename = "W")]
    Watt,

    /// Kilowatt
    #[serde(rename = "KW")]
    Kilowatt,

    /// Megawatt
    #[serde(rename = "MW")]
    Megawatt,

    // Energy units
    /// Watt hour
    #[serde(rename = "WH")]
    WattHour,

    /// Kilowatt hour
    #[serde(rename = "KWH")]
    KilowattHour,

    /// Megawatt hour
    #[serde(rename = "MWH")]
    MegawattHour,

    // Reactive power units
    /// Volt-ampere reactive
    #[serde(rename = "VAR")]
    VoltAmpereReactive,

    /// Kilovolt-ampere reactive
    #[serde(rename = "KVAR")]
    KilovoltAmpereReactive,

    /// Volt-ampere reactive hour
    #[serde(rename = "VARH")]
    VoltAmpereReactiveHour,

    /// Kilovolt-ampere reactive hour
    #[serde(rename = "KVARH")]
    KilovoltAmpereReactiveHour,

    // Volume units
    /// Cubic meter (for gas)
    #[serde(rename = "KUBIKMETER")]
    CubicMeter,

    // Count unit
    /// Piece/unit count
    #[serde(rename = "STUECK")]
    Piece,

    // Time units
    /// Second
    #[serde(rename = "SEKUNDE")]
    Second,

    /// Minute
    #[serde(rename = "MINUTE")]
    Minute,

    /// Hour
    #[serde(rename = "STUNDE")]
    Hour,

    /// Quarter hour
    #[serde(rename = "VIERTEL_STUNDE")]
    QuarterHour,

    /// Day
    #[serde(rename = "TAG")]
    Day,

    /// Week
    #[serde(rename = "WOCHE")]
    Week,

    /// Month
    #[serde(rename = "MONAT")]
    Month,

    /// Quarter (3 months)
    #[serde(rename = "QUARTAL")]
    Quarter,

    /// Half year
    #[serde(rename = "HALBJAHR")]
    HalfYear,

    /// Year
    #[serde(rename = "JAHR")]
    Year,

    // Other units
    /// Percent
    #[serde(rename = "PROZENT")]
    Percent,

    /// Kilowatt hour per Kelvin
    #[serde(rename = "KWHK")]
    KilowattHourPerKelvin,
}

impl Unit {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::Watt => "Watt",
            Self::Kilowatt => "Kilowatt",
            Self::Megawatt => "Megawatt",
            Self::WattHour => "Wattstunde",
            Self::KilowattHour => "Kilowattstunde",
            Self::MegawattHour => "Megawattstunde",
            Self::VoltAmpereReactive => "Var",
            Self::KilovoltAmpereReactive => "Kilovar",
            Self::VoltAmpereReactiveHour => "Varstunde",
            Self::KilovoltAmpereReactiveHour => "Kilovarstunde",
            Self::CubicMeter => "Kubikmeter",
            Self::Piece => "Stück",
            Self::Second => "Sekunde",
            Self::Minute => "Minute",
            Self::Hour => "Stunde",
            Self::QuarterHour => "Viertelstunde",
            Self::Day => "Tag",
            Self::Week => "Woche",
            Self::Month => "Monat",
            Self::Quarter => "Quartal",
            Self::HalfYear => "Halbjahr",
            Self::Year => "Jahr",
            Self::Percent => "Prozent",
            Self::KilowattHourPerKelvin => "Kilowattstunde pro Kelvin",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&Unit::KilowattHour).unwrap(),
            r#""KWH""#
        );
        assert_eq!(
            serde_json::to_string(&Unit::CubicMeter).unwrap(),
            r#""KUBIKMETER""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Unit>(r#""MWH""#).unwrap(),
            Unit::MegawattHour
        );
    }

    #[test]
    fn test_roundtrip() {
        for unit in [
            Unit::Watt,
            Unit::Kilowatt,
            Unit::Megawatt,
            Unit::WattHour,
            Unit::KilowattHour,
            Unit::MegawattHour,
            Unit::VoltAmpereReactive,
            Unit::KilovoltAmpereReactive,
            Unit::VoltAmpereReactiveHour,
            Unit::KilovoltAmpereReactiveHour,
            Unit::CubicMeter,
            Unit::Piece,
            Unit::Second,
            Unit::Minute,
            Unit::Hour,
            Unit::QuarterHour,
            Unit::Day,
            Unit::Week,
            Unit::Month,
            Unit::Quarter,
            Unit::HalfYear,
            Unit::Year,
            Unit::Percent,
            Unit::KilowattHourPerKelvin,
        ] {
            let json = serde_json::to_string(&unit).unwrap();
            let parsed: Unit = serde_json::from_str(&json).unwrap();
            assert_eq!(unit, parsed);
        }
    }
}
