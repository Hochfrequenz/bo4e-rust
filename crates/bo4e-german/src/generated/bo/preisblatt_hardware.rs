#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreisblattHardware {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceSheetNumber")]
    pub preisblattnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validFrom")]
    pub gueltig_ab: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub gueltig_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "hardwareItems"
    )]
    pub hardware: Vec<crate::Hardware>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "installationPrice")]
    pub installationspreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "rentalPrice")]
    pub mietpreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "purchasePrice")]
    pub kaufpreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "provider")]
    pub hardwareanbieter: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::HardwarePriceSheet> for PreisblattHardware {
    fn from(v: bo4e_core::bo::HardwarePriceSheet) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            preisblattnummer: v.price_sheet_number,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            gueltig_ab: v.valid_from,
            gueltig_bis: v.valid_until,
            hardware: v.hardware_items.into_iter().map(Into::into).collect(),
            installationspreis: v.installation_price.map(Into::into),
            mietpreis: v.rental_price.map(Into::into),
            kaufpreis: v.purchase_price.map(Into::into),
            hardwareanbieter: v.provider.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<PreisblattHardware> for bo4e_core::bo::HardwarePriceSheet {
    fn from(v: PreisblattHardware) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            price_sheet_number: v.preisblattnummer,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            valid_from: v.gueltig_ab,
            valid_until: v.gueltig_bis,
            hardware_items: v.hardware.into_iter().map(Into::into).collect(),
            installation_price: v.installationspreis.map(Into::into),
            rental_price: v.mietpreis.map(Into::into),
            purchase_price: v.kaufpreis.map(Into::into),
            provider: v.hardwareanbieter.map(|b| Box::new((*b).into())),
        }
    }
}
