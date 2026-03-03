#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Vertragsform {
    #[serde(rename = "ONLINE")]
    Online,
    #[serde(rename = "DIREKT")]
    Direct,
    #[serde(rename = "FAX")]
    Fax,
}
impl From<bo4e_core::enums::ContractForm> for Vertragsform {
    fn from(v: bo4e_core::enums::ContractForm) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ContractForm::Online => Vertragsform::Online,
            bo4e_core::enums::ContractForm::Direct => Vertragsform::Direct,
            bo4e_core::enums::ContractForm::Fax => Vertragsform::Fax,
            _ => panic!("Unknown {} variant", stringify!(ContractForm)),
        }
    }
}
impl From<Vertragsform> for bo4e_core::enums::ContractForm {
    fn from(v: Vertragsform) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Vertragsform::Online => bo4e_core::enums::ContractForm::Online,
            Vertragsform::Direct => bo4e_core::enums::ContractForm::Direct,
            Vertragsform::Fax => bo4e_core::enums::ContractForm::Fax,
        }
    }
}
