#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geschaeftspartner {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "partnerId")]
    pub geschaeftspartner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name3: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "roles")]
    pub geschaeftspartnerrollen: Vec<crate::Geschaeftspartnerrolle>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "contactMethods")]
    pub kontaktwege: Vec<crate::Kontaktweg>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "commercialRegisterNumber")]
    pub handelsregisternummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "taxId")]
    pub steuernummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "vatId")]
    pub umsatzsteuer_id: Option<String>,
}
impl From<bo4e_core::bo::BusinessPartner> for Geschaeftspartner {
    fn from(v: bo4e_core::bo::BusinessPartner) -> Self {
        Self {
            meta: v.meta,
            geschaeftspartner_id: v.partner_id,
            name1: v.name1,
            name2: v.name2,
            name3: v.name3,
            geschaeftspartnerrollen: v.roles.into_iter().map(Into::into).collect(),
            adresse: v.address.map(Into::into),
            kontaktwege: v.contact_methods.into_iter().map(Into::into).collect(),
            handelsregisternummer: v.commercial_register_number,
            steuernummer: v.tax_id,
            umsatzsteuer_id: v.vat_id,
        }
    }
}
impl From<Geschaeftspartner> for bo4e_core::bo::BusinessPartner {
    fn from(v: Geschaeftspartner) -> Self {
        Self {
            meta: v.meta,
            partner_id: v.geschaeftspartner_id,
            name1: v.name1,
            name2: v.name2,
            name3: v.name3,
            roles: v.geschaeftspartnerrollen.into_iter().map(Into::into).collect(),
            address: v.adresse.map(Into::into),
            contact_methods: v.kontaktwege.into_iter().map(Into::into).collect(),
            commercial_register_number: v.handelsregisternummer,
            tax_id: v.steuernummer,
            vat_id: v.umsatzsteuer_id,
        }
    }
}
