#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Kontaktart {
    #[serde(rename = "POSTWEG")]
    Postweg,
    #[serde(rename = "TELEFON")]
    Telefon,
    #[serde(rename = "FAX")]
    Fax,
    #[serde(rename = "E_MAIL")]
    EMail,
    #[serde(rename = "SMS")]
    Sms,
}
impl From<bo4e_core::enums::ContactType> for Kontaktart {
    fn from(v: bo4e_core::enums::ContactType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ContactType::Mail => Kontaktart::Postweg,
            bo4e_core::enums::ContactType::Phone => Kontaktart::Telefon,
            bo4e_core::enums::ContactType::Fax => Kontaktart::Fax,
            bo4e_core::enums::ContactType::Email => Kontaktart::EMail,
            bo4e_core::enums::ContactType::Sms => Kontaktart::Sms,
            _ => panic!("Unknown {} variant", stringify!(ContactType)),
        }
    }
}
impl From<Kontaktart> for bo4e_core::enums::ContactType {
    fn from(v: Kontaktart) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Kontaktart::Postweg => bo4e_core::enums::ContactType::Mail,
            Kontaktart::Telefon => bo4e_core::enums::ContactType::Phone,
            Kontaktart::Fax => bo4e_core::enums::ContactType::Fax,
            Kontaktart::EMail => bo4e_core::enums::ContactType::Email,
            Kontaktart::Sms => bo4e_core::enums::ContactType::Sms,
        }
    }
}
