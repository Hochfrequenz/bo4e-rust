//! Meter type (Zaehlertyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of metering device.
///
/// Types of meters for electricity and gas sectors.
///
/// German: Zaehlertyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Zaehlertyp"))]
#[non_exhaustive]
pub enum MeterType {
    /// Three-phase rotating meter (Ferraris meter for three-phase)
    #[serde(rename = "DREHSTROMZAEHLER")]
    ThreePhaseRotatingMeter,

    /// Bellows gas meter (Balgengaszähler)
    #[serde(rename = "BALGENGASZAEHLER")]
    BellowsGasMeter,

    /// Rotary piston gas meter (Drehkolbengaszähler)
    #[serde(rename = "DREHKOLBENZAEHLER")]
    RotaryPistonGasMeter,

    /// Power measuring meter (Leistungszähler)
    #[serde(rename = "LEISTUNGSZAEHLER")]
    PowerMeter,

    /// Maximum demand meter (Maximumzähler)
    #[serde(rename = "MAXIMUMZAEHLER")]
    MaximumDemandMeter,

    /// Turbine wheel gas meter
    #[serde(rename = "TURBINENRADGASZAEHLER")]
    TurbineWheelGasMeter,

    /// Ultrasonic gas meter
    #[serde(rename = "ULTRASCHALLGASZAEHLER")]
    UltrasonicGasMeter,

    /// Single-phase alternating current meter (Ferraris meter for single-phase)
    #[serde(rename = "WECHSELSTROMZAEHLER")]
    SinglePhaseAlternatingMeter,

    /// Modern measuring device (Moderne Messeinrichtung)
    #[serde(rename = "MODERNE_MESSEINRICHTUNG")]
    ModernMeasuringDevice,

    /// Intelligent measuring system / Smart meter (Intelligentes Messsystem)
    #[serde(rename = "INTELLIGENTES_MESSSYSTEM")]
    IntelligentMeasuringSystem,

    /// Electronic meter (Elektronischer Zähler)
    #[serde(rename = "ELEKTRONISCHER_ZAEHLER")]
    ElectronicMeter,

    /// Vortex gas meter (Wirbelgaszähler)
    #[serde(rename = "WIRBELGASZAEHLER")]
    VortexGasMeter,

    /// Water meter (Wasserzähler)
    #[serde(rename = "WASSERZAEHLER")]
    WaterMeter,
}

impl MeterType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::ThreePhaseRotatingMeter => "Drehstromzähler",
            Self::BellowsGasMeter => "Balgengaszähler",
            Self::RotaryPistonGasMeter => "Drehkolbengaszähler",
            Self::PowerMeter => "Leistungszähler",
            Self::MaximumDemandMeter => "Maximumzähler",
            Self::TurbineWheelGasMeter => "Turbinenradgaszähler",
            Self::UltrasonicGasMeter => "Ultraschallgaszähler",
            Self::SinglePhaseAlternatingMeter => "Wechselstromzähler",
            Self::ModernMeasuringDevice => "Moderne Messeinrichtung",
            Self::IntelligentMeasuringSystem => "Intelligentes Messsystem",
            Self::ElectronicMeter => "Elektronischer Zähler",
            Self::VortexGasMeter => "Wirbelgaszähler",
            Self::WaterMeter => "Wasserzähler",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&MeterType::IntelligentMeasuringSystem).unwrap(),
            r#""INTELLIGENTES_MESSSYSTEM""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<MeterType>(r#""MODERNE_MESSEINRICHTUNG""#).unwrap(),
            MeterType::ModernMeasuringDevice
        );
    }

    #[test]
    fn test_roundtrip() {
        for mtype in [
            MeterType::ThreePhaseRotatingMeter,
            MeterType::BellowsGasMeter,
            MeterType::RotaryPistonGasMeter,
            MeterType::PowerMeter,
            MeterType::MaximumDemandMeter,
            MeterType::TurbineWheelGasMeter,
            MeterType::UltrasonicGasMeter,
            MeterType::SinglePhaseAlternatingMeter,
            MeterType::ModernMeasuringDevice,
            MeterType::IntelligentMeasuringSystem,
            MeterType::ElectronicMeter,
            MeterType::VortexGasMeter,
            MeterType::WaterMeter,
        ] {
            let json = serde_json::to_string(&mtype).unwrap();
            let parsed: MeterType = serde_json::from_str(&json).unwrap();
            assert_eq!(mtype, parsed);
        }
    }
}
