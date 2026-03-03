#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marktteilnehmer {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketPartnerId")]
    pub marktpartner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketRole")]
    pub marktrolle: Option<crate::Marktrolle>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "contactMethods"
    )]
    pub kontaktwege: Vec<crate::Kontaktweg>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "businessPartner")]
    pub geschaeftspartner: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::MarketParticipant> for Marktteilnehmer {
    fn from(v: bo4e_core::bo::MarketParticipant) -> Self {
        Self {
            meta: v.meta,
            marktpartner_id: v.market_partner_id,
            name: v.name,
            marktrolle: v.market_role.map(Into::into),
            sparte: v.division.map(Into::into),
            adresse: v.address.map(Into::into),
            kontaktwege: v.contact_methods.into_iter().map(Into::into).collect(),
            geschaeftspartner: v.business_partner.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Marktteilnehmer> for bo4e_core::bo::MarketParticipant {
    fn from(v: Marktteilnehmer) -> Self {
        Self {
            meta: v.meta,
            market_partner_id: v.marktpartner_id,
            name: v.name,
            market_role: v.marktrolle.map(Into::into),
            division: v.sparte.map(Into::into),
            address: v.adresse.map(Into::into),
            contact_methods: v.kontaktwege.into_iter().map(Into::into).collect(),
            business_partner: v.geschaeftspartner.map(|b| Box::new((*b).into())),
        }
    }
}
