#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Adresse {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "street")]
    pub strasse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "houseNumber")]
    pub hausnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "postalCode")]
    pub postleitzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "city")]
    pub ort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "district")]
    pub ortsteil: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "poBox")]
    pub postfach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "addressAddition")]
    pub adresszusatz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co_ergaenzung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "countryCode")]
    pub landescode: Option<crate::Landescode>,
}
impl From<bo4e_core::com::Address> for Adresse {
    fn from(v: bo4e_core::com::Address) -> Self {
        Self {
            meta: v.meta,
            strasse: v.street,
            hausnummer: v.house_number,
            postleitzahl: v.postal_code,
            ort: v.city,
            ortsteil: v.district,
            postfach: v.po_box,
            adresszusatz: v.address_addition,
            co_ergaenzung: v.co_ergaenzung,
            landescode: v.country_code.map(Into::into),
        }
    }
}
impl From<Adresse> for bo4e_core::com::Address {
    fn from(v: Adresse) -> Self {
        Self {
            meta: v.meta,
            street: v.strasse,
            house_number: v.hausnummer,
            postal_code: v.postleitzahl,
            city: v.ort,
            district: v.ortsteil,
            po_box: v.postfach,
            address_addition: v.adresszusatz,
            co_ergaenzung: v.co_ergaenzung,
            country_code: v.landescode.map(Into::into),
        }
    }
}
