#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Berechnungsformel {
    #[serde(rename = "HOECHSTWERT")]
    HChstwertDerMaximalwerte,
    #[serde(rename = "MINIMALWERT")]
    Minimalwert,
    #[serde(rename = "MITTELWERT")]
    Mittelwert,
    #[serde(rename = "SUMMENWERT")]
    Summenwert,
}
impl From<bo4e_core::enums::CalculationFormula> for Berechnungsformel {
    fn from(v: bo4e_core::enums::CalculationFormula) -> Self {
        match v {
            bo4e_core::enums::CalculationFormula::HighestValue => {
                Berechnungsformel::HChstwertDerMaximalwerte
            }
            bo4e_core::enums::CalculationFormula::MinimumValue => {
                Berechnungsformel::Minimalwert
            }
            bo4e_core::enums::CalculationFormula::AverageValue => {
                Berechnungsformel::Mittelwert
            }
            bo4e_core::enums::CalculationFormula::SumValue => {
                Berechnungsformel::Summenwert
            }
            _ => panic!("Unknown {} variant", stringify!(CalculationFormula)),
        }
    }
}
impl From<Berechnungsformel> for bo4e_core::enums::CalculationFormula {
    fn from(v: Berechnungsformel) -> Self {
        match v {
            Berechnungsformel::HChstwertDerMaximalwerte => {
                bo4e_core::enums::CalculationFormula::HighestValue
            }
            Berechnungsformel::Minimalwert => {
                bo4e_core::enums::CalculationFormula::MinimumValue
            }
            Berechnungsformel::Mittelwert => {
                bo4e_core::enums::CalculationFormula::AverageValue
            }
            Berechnungsformel::Summenwert => {
                bo4e_core::enums::CalculationFormula::SumValue
            }
            _ => panic!("Unknown {} variant", stringify!(Berechnungsformel)),
        }
    }
}
