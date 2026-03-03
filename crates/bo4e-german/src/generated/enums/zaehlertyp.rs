#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Zaehlertyp {
    #[serde(rename = "DREHSTROMZAEHLER")]
    FerrarisMeterForThreePhase,
    #[serde(rename = "BALGENGASZAEHLER")]
    BalgengaszHler,
    #[serde(rename = "DREHKOLBENZAEHLER")]
    DrehkolbengaszHler,
    #[serde(rename = "LEISTUNGSZAEHLER")]
    LeistungszHler,
    #[serde(rename = "MAXIMUMZAEHLER")]
    MaximumzHler,
    #[serde(rename = "TURBINENRADGASZAEHLER")]
    TurbineWheelGasMeter,
    #[serde(rename = "ULTRASCHALLGASZAEHLER")]
    UltrasonicGasMeter,
    #[serde(rename = "WECHSELSTROMZAEHLER")]
    FerrarisMeterForSinglePhase,
    #[serde(rename = "MODERNE_MESSEINRICHTUNG")]
    ModerneMesseinrichtung,
    #[serde(rename = "INTELLIGENTES_MESSSYSTEM")]
    IntelligentesMesssystem,
    #[serde(rename = "ELEKTRONISCHER_ZAEHLER")]
    ElektronischerZHler,
    #[serde(rename = "WIRBELGASZAEHLER")]
    WirbelgaszHler,
    #[serde(rename = "WASSERZAEHLER")]
    WasserzHler,
}
impl From<bo4e_core::enums::MeterType> for Zaehlertyp {
    fn from(v: bo4e_core::enums::MeterType) -> Self {
        match v {
            bo4e_core::enums::MeterType::ThreePhaseRotatingMeter => {
                Zaehlertyp::FerrarisMeterForThreePhase
            }
            bo4e_core::enums::MeterType::BellowsGasMeter => Zaehlertyp::BalgengaszHler,
            bo4e_core::enums::MeterType::RotaryPistonGasMeter => {
                Zaehlertyp::DrehkolbengaszHler
            }
            bo4e_core::enums::MeterType::PowerMeter => Zaehlertyp::LeistungszHler,
            bo4e_core::enums::MeterType::MaximumDemandMeter => Zaehlertyp::MaximumzHler,
            bo4e_core::enums::MeterType::TurbineWheelGasMeter => {
                Zaehlertyp::TurbineWheelGasMeter
            }
            bo4e_core::enums::MeterType::UltrasonicGasMeter => {
                Zaehlertyp::UltrasonicGasMeter
            }
            bo4e_core::enums::MeterType::SinglePhaseAlternatingMeter => {
                Zaehlertyp::FerrarisMeterForSinglePhase
            }
            bo4e_core::enums::MeterType::ModernMeasuringDevice => {
                Zaehlertyp::ModerneMesseinrichtung
            }
            bo4e_core::enums::MeterType::IntelligentMeasuringSystem => {
                Zaehlertyp::IntelligentesMesssystem
            }
            bo4e_core::enums::MeterType::ElectronicMeter => {
                Zaehlertyp::ElektronischerZHler
            }
            bo4e_core::enums::MeterType::VortexGasMeter => Zaehlertyp::WirbelgaszHler,
            bo4e_core::enums::MeterType::WaterMeter => Zaehlertyp::WasserzHler,
            _ => panic!("Unknown {} variant", stringify!(MeterType)),
        }
    }
}
impl From<Zaehlertyp> for bo4e_core::enums::MeterType {
    fn from(v: Zaehlertyp) -> Self {
        match v {
            Zaehlertyp::FerrarisMeterForThreePhase => {
                bo4e_core::enums::MeterType::ThreePhaseRotatingMeter
            }
            Zaehlertyp::BalgengaszHler => bo4e_core::enums::MeterType::BellowsGasMeter,
            Zaehlertyp::DrehkolbengaszHler => {
                bo4e_core::enums::MeterType::RotaryPistonGasMeter
            }
            Zaehlertyp::LeistungszHler => bo4e_core::enums::MeterType::PowerMeter,
            Zaehlertyp::MaximumzHler => bo4e_core::enums::MeterType::MaximumDemandMeter,
            Zaehlertyp::TurbineWheelGasMeter => {
                bo4e_core::enums::MeterType::TurbineWheelGasMeter
            }
            Zaehlertyp::UltrasonicGasMeter => {
                bo4e_core::enums::MeterType::UltrasonicGasMeter
            }
            Zaehlertyp::FerrarisMeterForSinglePhase => {
                bo4e_core::enums::MeterType::SinglePhaseAlternatingMeter
            }
            Zaehlertyp::ModerneMesseinrichtung => {
                bo4e_core::enums::MeterType::ModernMeasuringDevice
            }
            Zaehlertyp::IntelligentesMesssystem => {
                bo4e_core::enums::MeterType::IntelligentMeasuringSystem
            }
            Zaehlertyp::ElektronischerZHler => {
                bo4e_core::enums::MeterType::ElectronicMeter
            }
            Zaehlertyp::WirbelgaszHler => bo4e_core::enums::MeterType::VortexGasMeter,
            Zaehlertyp::WasserzHler => bo4e_core::enums::MeterType::WaterMeter,
            _ => panic!("Unknown {} variant", stringify!(Zaehlertyp)),
        }
    }
}
