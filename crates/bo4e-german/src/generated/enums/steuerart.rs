#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Steuerart {
    #[serde(rename = "RCV")]
    UmkehrungDerSteuerpflicht,
    #[serde(rename = "UST")]
    Umsatzsteuer,
    #[serde(rename = "VST")]
    Vorsteuer,
}
impl From<bo4e_core::enums::TaxType> for Steuerart {
    fn from(v: bo4e_core::enums::TaxType) -> Self {
        match v {
            bo4e_core::enums::TaxType::ReverseCharge => {
                Steuerart::UmkehrungDerSteuerpflicht
            }
            bo4e_core::enums::TaxType::ValueAddedTax => Steuerart::Umsatzsteuer,
            bo4e_core::enums::TaxType::InputTax => Steuerart::Vorsteuer,
            _ => panic!("Unknown {} variant", stringify!(TaxType)),
        }
    }
}
impl From<Steuerart> for bo4e_core::enums::TaxType {
    fn from(v: Steuerart) -> Self {
        match v {
            Steuerart::UmkehrungDerSteuerpflicht => {
                bo4e_core::enums::TaxType::ReverseCharge
            }
            Steuerart::Umsatzsteuer => bo4e_core::enums::TaxType::ValueAddedTax,
            Steuerart::Vorsteuer => bo4e_core::enums::TaxType::InputTax,
            _ => panic!("Unknown {} variant", stringify!(Steuerart)),
        }
    }
}
