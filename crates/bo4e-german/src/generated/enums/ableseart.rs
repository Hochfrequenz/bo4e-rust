#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Ableseart {
    #[serde(rename = "KUNDENSELBSTABLESUNG")]
    Kundenselbstablesung,
    #[serde(rename = "FERNAUSLESUNG")]
    Fernauslesung,
    #[serde(rename = "MSB_ABLESUNG")]
    AblesungDurchMsb,
    #[serde(rename = "SCHAETZUNG")]
    SchTzung,
    #[serde(rename = "NB_ABLESUNG")]
    AblesungDurchNb,
}
impl From<bo4e_core::enums::ReadingType> for Ableseart {
    fn from(v: bo4e_core::enums::ReadingType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ReadingType::CustomerSelfReading => Ableseart::Kundenselbstablesung,
            bo4e_core::enums::ReadingType::RemoteReading => Ableseart::Fernauslesung,
            bo4e_core::enums::ReadingType::MeteringOperatorReading => Ableseart::AblesungDurchMsb,
            bo4e_core::enums::ReadingType::Estimated => Ableseart::SchTzung,
            bo4e_core::enums::ReadingType::NetworkOperatorReading => Ableseart::AblesungDurchNb,
            _ => panic!("Unknown {} variant", stringify!(ReadingType)),
        }
    }
}
impl From<Ableseart> for bo4e_core::enums::ReadingType {
    fn from(v: Ableseart) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Ableseart::Kundenselbstablesung => bo4e_core::enums::ReadingType::CustomerSelfReading,
            Ableseart::Fernauslesung => bo4e_core::enums::ReadingType::RemoteReading,
            Ableseart::AblesungDurchMsb => bo4e_core::enums::ReadingType::MeteringOperatorReading,
            Ableseart::SchTzung => bo4e_core::enums::ReadingType::Estimated,
            Ableseart::AblesungDurchNb => bo4e_core::enums::ReadingType::NetworkOperatorReading,
        }
    }
}
