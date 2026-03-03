#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Geschaeftspartnerrolle {
    #[serde(rename = "LIEFERANT")]
    Lieferant,
    #[serde(rename = "DIENSTLEISTER")]
    Dienstleister,
    #[serde(rename = "KUNDE")]
    Kunde,
    #[serde(rename = "INTERESSENT")]
    Interessent,
    #[serde(rename = "MARKTPARTNER")]
    Marktpartner,
    #[serde(rename = "NETZBETREIBER")]
    Netzbetreiber,
}
impl From<bo4e_core::enums::BusinessPartnerRole> for Geschaeftspartnerrolle {
    fn from(v: bo4e_core::enums::BusinessPartnerRole) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::BusinessPartnerRole::Supplier => Geschaeftspartnerrolle::Lieferant,
            bo4e_core::enums::BusinessPartnerRole::ServiceProvider => {
                Geschaeftspartnerrolle::Dienstleister
            }
            bo4e_core::enums::BusinessPartnerRole::Customer => Geschaeftspartnerrolle::Kunde,
            bo4e_core::enums::BusinessPartnerRole::InterestedParty => {
                Geschaeftspartnerrolle::Interessent
            }
            bo4e_core::enums::BusinessPartnerRole::MarketPartner => {
                Geschaeftspartnerrolle::Marktpartner
            }
            bo4e_core::enums::BusinessPartnerRole::NetworkOperator => {
                Geschaeftspartnerrolle::Netzbetreiber
            }
            _ => panic!("Unknown {} variant", stringify!(BusinessPartnerRole)),
        }
    }
}
impl From<Geschaeftspartnerrolle> for bo4e_core::enums::BusinessPartnerRole {
    fn from(v: Geschaeftspartnerrolle) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Geschaeftspartnerrolle::Lieferant => bo4e_core::enums::BusinessPartnerRole::Supplier,
            Geschaeftspartnerrolle::Dienstleister => {
                bo4e_core::enums::BusinessPartnerRole::ServiceProvider
            }
            Geschaeftspartnerrolle::Kunde => bo4e_core::enums::BusinessPartnerRole::Customer,
            Geschaeftspartnerrolle::Interessent => {
                bo4e_core::enums::BusinessPartnerRole::InterestedParty
            }
            Geschaeftspartnerrolle::Marktpartner => {
                bo4e_core::enums::BusinessPartnerRole::MarketPartner
            }
            Geschaeftspartnerrolle::Netzbetreiber => {
                bo4e_core::enums::BusinessPartnerRole::NetworkOperator
            }
        }
    }
}
