#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Zahlungsweise {
    #[serde(rename = "SEPA_LASTSCHRIFT")]
    SEPALastschrift,
    #[serde(rename = "UEBERWEISUNG")]
    Ueberweisung,
}
impl From<bo4e_core::enums::PaymentMethod> for Zahlungsweise {
    fn from(v: bo4e_core::enums::PaymentMethod) -> Self {
        match v {
            bo4e_core::enums::PaymentMethod::SepaDirectDebit => {
                Zahlungsweise::SEPALastschrift
            }
            bo4e_core::enums::PaymentMethod::BankTransfer => Zahlungsweise::Ueberweisung,
            _ => panic!("Unknown {} variant", stringify!(PaymentMethod)),
        }
    }
}
impl From<Zahlungsweise> for bo4e_core::enums::PaymentMethod {
    fn from(v: Zahlungsweise) -> Self {
        match v {
            Zahlungsweise::SEPALastschrift => {
                bo4e_core::enums::PaymentMethod::SepaDirectDebit
            }
            Zahlungsweise::Ueberweisung => bo4e_core::enums::PaymentMethod::BankTransfer,
            _ => panic!("Unknown {} variant", stringify!(Zahlungsweise)),
        }
    }
}
