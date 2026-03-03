#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
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
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::TaxType::ReverseCharge => Steuerart::UmkehrungDerSteuerpflicht,
            bo4e_core::enums::TaxType::ValueAddedTax => Steuerart::Umsatzsteuer,
            bo4e_core::enums::TaxType::InputTax => Steuerart::Vorsteuer,
            _ => panic!("Unknown {} variant", stringify!(TaxType)),
        }
    }
}
impl From<Steuerart> for bo4e_core::enums::TaxType {
    fn from(v: Steuerart) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Steuerart::UmkehrungDerSteuerpflicht => bo4e_core::enums::TaxType::ReverseCharge,
            Steuerart::Umsatzsteuer => bo4e_core::enums::TaxType::ValueAddedTax,
            Steuerart::Vorsteuer => bo4e_core::enums::TaxType::InputTax,
        }
    }
}
