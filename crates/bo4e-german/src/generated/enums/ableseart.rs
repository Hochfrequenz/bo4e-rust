#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Ableseart {
    #[serde(rename = "KUNDENSELBSTABLESUNG")]
    Kundenselbstablesung,
    #[serde(rename = "FERNAUSLESUNG")]
    Fernauslesung,
    #[serde(rename = "MSB_ABLESUNG")]
    AblesungDurchMSB,
    #[serde(rename = "SCHAETZUNG")]
    SchTzung,
    #[serde(rename = "NB_ABLESUNG")]
    AblesungDurchNB,
}
impl From<bo4e_core::enums::ReadingType> for Ableseart {
    fn from(v: bo4e_core::enums::ReadingType) -> Self {
        match v {
            bo4e_core::enums::ReadingType::CustomerSelfReading => {
                Ableseart::Kundenselbstablesung
            }
            bo4e_core::enums::ReadingType::RemoteReading => Ableseart::Fernauslesung,
            bo4e_core::enums::ReadingType::MeteringOperatorReading => {
                Ableseart::AblesungDurchMSB
            }
            bo4e_core::enums::ReadingType::Estimated => Ableseart::SchTzung,
            bo4e_core::enums::ReadingType::NetworkOperatorReading => {
                Ableseart::AblesungDurchNB
            }
            _ => panic!("Unknown {} variant", stringify!(ReadingType)),
        }
    }
}
impl From<Ableseart> for bo4e_core::enums::ReadingType {
    fn from(v: Ableseart) -> Self {
        match v {
            Ableseart::Kundenselbstablesung => {
                bo4e_core::enums::ReadingType::CustomerSelfReading
            }
            Ableseart::Fernauslesung => bo4e_core::enums::ReadingType::RemoteReading,
            Ableseart::AblesungDurchMSB => {
                bo4e_core::enums::ReadingType::MeteringOperatorReading
            }
            Ableseart::SchTzung => bo4e_core::enums::ReadingType::Estimated,
            Ableseart::AblesungDurchNB => {
                bo4e_core::enums::ReadingType::NetworkOperatorReading
            }
            _ => panic!("Unknown {} variant", stringify!(Ableseart)),
        }
    }
}
