#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifpreisblatt {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customerType")]
    pub kundentyp: Option<crate::Kundentyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceSheetNumber")]
    pub preisblattnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validFrom")]
    pub gueltig_ab: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub gueltig_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "positions")]
    pub tarifpreispositionen: Vec<crate::Tarifpreisposition>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "publisher")]
    pub herausgeber: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::TariffPriceSheet> for Tarifpreisblatt {
    fn from(v: bo4e_core::bo::TariffPriceSheet) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            kundentyp: v.customer_type.map(Into::into),
            preisblattnummer: v.price_sheet_number,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            gueltig_ab: v.valid_from,
            gueltig_bis: v.valid_until,
            tarifpreispositionen: v.positions.into_iter().map(Into::into).collect(),
            herausgeber: v.publisher.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Tarifpreisblatt> for bo4e_core::bo::TariffPriceSheet {
    fn from(v: Tarifpreisblatt) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            customer_type: v.kundentyp.map(Into::into),
            price_sheet_number: v.preisblattnummer,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            valid_from: v.gueltig_ab,
            valid_until: v.gueltig_bis,
            positions: v.tarifpreispositionen.into_iter().map(Into::into).collect(),
            publisher: v.herausgeber.map(|b| Box::new((*b).into())),
        }
    }
}
