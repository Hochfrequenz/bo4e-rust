#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Ausschreibungstyp {
    #[serde(rename = "PRIVATRECHTLICH")]
    Privatrechtlich,
    #[serde(rename = "OEFFENTLICHRECHTLICH")]
    Oeffentlichrechtlich,
    #[serde(rename = "EUROPAWEIT")]
    Europaweit,
}
impl From<bo4e_core::enums::TenderType> for Ausschreibungstyp {
    fn from(v: bo4e_core::enums::TenderType) -> Self {
        match v {
            bo4e_core::enums::TenderType::PrivateLaw => {
                Ausschreibungstyp::Privatrechtlich
            }
            bo4e_core::enums::TenderType::PublicLaw => {
                Ausschreibungstyp::Oeffentlichrechtlich
            }
            bo4e_core::enums::TenderType::EuropeWide => Ausschreibungstyp::Europaweit,
            _ => panic!("Unknown {} variant", stringify!(TenderType)),
        }
    }
}
impl From<Ausschreibungstyp> for bo4e_core::enums::TenderType {
    fn from(v: Ausschreibungstyp) -> Self {
        match v {
            Ausschreibungstyp::Privatrechtlich => {
                bo4e_core::enums::TenderType::PrivateLaw
            }
            Ausschreibungstyp::Oeffentlichrechtlich => {
                bo4e_core::enums::TenderType::PublicLaw
            }
            Ausschreibungstyp::Europaweit => bo4e_core::enums::TenderType::EuropeWide,
            _ => panic!("Unknown {} variant", stringify!(Ausschreibungstyp)),
        }
    }
}
