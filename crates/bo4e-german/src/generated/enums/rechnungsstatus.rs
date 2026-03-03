#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Rechnungsstatus {
    #[serde(rename = "UNGEPRUEFT")]
    Ungeprueft,
    #[serde(rename = "GEPRUEFT_OK")]
    GeprueftOK,
    #[serde(rename = "GEPRUEFT_FEHLERHAFT")]
    GeprueftFehlerhaft,
    #[serde(rename = "GEBUCHT")]
    Gebucht,
    #[serde(rename = "BEZAHLT")]
    Bezahlt,
}
impl From<bo4e_core::enums::InvoiceStatus> for Rechnungsstatus {
    fn from(v: bo4e_core::enums::InvoiceStatus) -> Self {
        match v {
            bo4e_core::enums::InvoiceStatus::Unchecked => Rechnungsstatus::Ungeprueft,
            bo4e_core::enums::InvoiceStatus::CheckedOk => Rechnungsstatus::GeprueftOK,
            bo4e_core::enums::InvoiceStatus::CheckedWithErrors => {
                Rechnungsstatus::GeprueftFehlerhaft
            }
            bo4e_core::enums::InvoiceStatus::Booked => Rechnungsstatus::Gebucht,
            bo4e_core::enums::InvoiceStatus::Paid => Rechnungsstatus::Bezahlt,
            _ => panic!("Unknown {} variant", stringify!(InvoiceStatus)),
        }
    }
}
impl From<Rechnungsstatus> for bo4e_core::enums::InvoiceStatus {
    fn from(v: Rechnungsstatus) -> Self {
        match v {
            Rechnungsstatus::Ungeprueft => bo4e_core::enums::InvoiceStatus::Unchecked,
            Rechnungsstatus::GeprueftOK => bo4e_core::enums::InvoiceStatus::CheckedOk,
            Rechnungsstatus::GeprueftFehlerhaft => {
                bo4e_core::enums::InvoiceStatus::CheckedWithErrors
            }
            Rechnungsstatus::Gebucht => bo4e_core::enums::InvoiceStatus::Booked,
            Rechnungsstatus::Bezahlt => bo4e_core::enums::InvoiceStatus::Paid,
            _ => panic!("Unknown {} variant", stringify!(Rechnungsstatus)),
        }
    }
}
