#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "salutation")]
    pub anrede: Option<crate::Anrede>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    pub titel: Option<crate::Titel>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "firstName")]
    pub vorname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "lastName")]
    pub nachname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "nameSuffix")]
    pub namenszusatz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "namePrefix")]
    pub namenspraefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "companyName")]
    pub firma: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "birthDate")]
    pub geburtsdatum: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "address")]
    pub adresse: Option<crate::Adresse>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "contactMethods")]
    pub kontaktwege: Vec<crate::Kontaktweg>,
}
impl From<bo4e_core::bo::Person> for Person {
    fn from(v: bo4e_core::bo::Person) -> Self {
        Self {
            meta: v.meta,
            anrede: v.salutation.map(Into::into),
            titel: v.title.map(Into::into),
            vorname: v.first_name,
            nachname: v.last_name,
            namenszusatz: v.name_suffix,
            namenspraefix: v.name_prefix,
            firma: v.company_name,
            geburtsdatum: v.birth_date,
            adresse: v.address.map(Into::into),
            kontaktwege: v.contact_methods.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<Person> for bo4e_core::bo::Person {
    fn from(v: Person) -> Self {
        Self {
            meta: v.meta,
            salutation: v.anrede.map(Into::into),
            title: v.titel.map(Into::into),
            first_name: v.vorname,
            last_name: v.nachname,
            name_suffix: v.namenszusatz,
            name_prefix: v.namenspraefix,
            company_name: v.firma,
            birth_date: v.geburtsdatum,
            address: v.adresse.map(Into::into),
            contact_methods: v.kontaktwege.into_iter().map(Into::into).collect(),
        }
    }
}
