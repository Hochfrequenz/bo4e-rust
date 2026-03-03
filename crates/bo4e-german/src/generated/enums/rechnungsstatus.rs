#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Rechnungsstatus {
    #[serde(rename = "UNGEPRUEFT")]
    Ungeprueft,
    #[serde(rename = "GEPRUEFT_OK")]
    GeprueftOk,
    #[serde(rename = "GEPRUEFT_FEHLERHAFT")]
    GeprueftFehlerhaft,
    #[serde(rename = "GEBUCHT")]
    Gebucht,
    #[serde(rename = "BEZAHLT")]
    Bezahlt,
}
impl From<bo4e_core::enums::InvoiceStatus> for Rechnungsstatus {
    fn from(v: bo4e_core::enums::InvoiceStatus) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::InvoiceStatus::Unchecked => Rechnungsstatus::Ungeprueft,
            bo4e_core::enums::InvoiceStatus::CheckedOk => Rechnungsstatus::GeprueftOk,
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
        #[allow(unreachable_patterns)]
        match v {
            Rechnungsstatus::Ungeprueft => bo4e_core::enums::InvoiceStatus::Unchecked,
            Rechnungsstatus::GeprueftOk => bo4e_core::enums::InvoiceStatus::CheckedOk,
            Rechnungsstatus::GeprueftFehlerhaft => {
                bo4e_core::enums::InvoiceStatus::CheckedWithErrors
            }
            Rechnungsstatus::Gebucht => bo4e_core::enums::InvoiceStatus::Booked,
            Rechnungsstatus::Bezahlt => bo4e_core::enums::InvoiceStatus::Paid,
        }
    }
}
