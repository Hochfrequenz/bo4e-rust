#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Organisationstyp {
    #[serde(rename = "PRIVATPERSON")]
    B2C,
    #[serde(rename = "UNTERNEHMEN")]
    B2B,
    #[serde(rename = "KOMMUNALE_EINRICHTUNG")]
    B2A,
    #[serde(rename = "STAATLICHE_BEHOERDE")]
    B2G,
}
impl From<bo4e_core::enums::OrganizationType> for Organisationstyp {
    fn from(v: bo4e_core::enums::OrganizationType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::OrganizationType::PrivatePerson => Organisationstyp::B2C,
            bo4e_core::enums::OrganizationType::Company => Organisationstyp::B2B,
            bo4e_core::enums::OrganizationType::MunicipalInstitution => Organisationstyp::B2A,
            bo4e_core::enums::OrganizationType::GovernmentAuthority => Organisationstyp::B2G,
            _ => panic!("Unknown {} variant", stringify!(OrganizationType)),
        }
    }
}
impl From<Organisationstyp> for bo4e_core::enums::OrganizationType {
    fn from(v: Organisationstyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Organisationstyp::B2C => bo4e_core::enums::OrganizationType::PrivatePerson,
            Organisationstyp::B2B => bo4e_core::enums::OrganizationType::Company,
            Organisationstyp::B2A => bo4e_core::enums::OrganizationType::MunicipalInstitution,
            Organisationstyp::B2G => bo4e_core::enums::OrganizationType::GovernmentAuthority,
        }
    }
}
