#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Zahlungsweise {
    #[serde(rename = "SEPA_LASTSCHRIFT")]
    SepaLastschrift,
    #[serde(rename = "UEBERWEISUNG")]
    Ueberweisung,
}
impl From<bo4e_core::enums::PaymentMethod> for Zahlungsweise {
    fn from(v: bo4e_core::enums::PaymentMethod) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::PaymentMethod::SepaDirectDebit => Zahlungsweise::SepaLastschrift,
            bo4e_core::enums::PaymentMethod::BankTransfer => Zahlungsweise::Ueberweisung,
            _ => panic!("Unknown {} variant", stringify!(PaymentMethod)),
        }
    }
}
impl From<Zahlungsweise> for bo4e_core::enums::PaymentMethod {
    fn from(v: Zahlungsweise) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Zahlungsweise::SepaLastschrift => bo4e_core::enums::PaymentMethod::SepaDirectDebit,
            Zahlungsweise::Ueberweisung => bo4e_core::enums::PaymentMethod::BankTransfer,
        }
    }
}
