#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Angebot {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "offerNumber")]
    pub angebotsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "status")]
    pub angebotsstatus: Option<crate::Angebotsstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "offerDate")]
    pub angebotsdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub gueltig_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deliveryPeriod")]
    pub lieferzeitraum: Option<crate::Zeitraum>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "variants")]
    pub angebotsvarianten: Vec<crate::Angebotsvariante>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "bidder")]
    pub anbieter: Option<Box<crate::Geschaeftspartner>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customer")]
    pub kunde: Option<Box<crate::Geschaeftspartner>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tenderId")]
    pub ausschreibung_id: Option<String>,
}
impl From<bo4e_core::bo::Offer> for Angebot {
    fn from(v: bo4e_core::bo::Offer) -> Self {
        Self {
            meta: v.meta,
            angebotsnummer: v.offer_number,
            beschreibung: v.description,
            angebotsstatus: v.status.map(Into::into),
            sparte: v.division.map(Into::into),
            angebotsdatum: v.offer_date,
            gueltig_bis: v.valid_until,
            lieferzeitraum: v.delivery_period.map(Into::into),
            angebotsvarianten: v.variants.into_iter().map(Into::into).collect(),
            anbieter: v.bidder.map(|b| Box::new((*b).into())),
            kunde: v.customer.map(|b| Box::new((*b).into())),
            ausschreibung_id: v.tender_id,
        }
    }
}
impl From<Angebot> for bo4e_core::bo::Offer {
    fn from(v: Angebot) -> Self {
        Self {
            meta: v.meta,
            offer_number: v.angebotsnummer,
            description: v.beschreibung,
            status: v.angebotsstatus.map(Into::into),
            division: v.sparte.map(Into::into),
            offer_date: v.angebotsdatum,
            valid_until: v.gueltig_bis,
            delivery_period: v.lieferzeitraum.map(Into::into),
            variants: v.angebotsvarianten.into_iter().map(Into::into).collect(),
            bidder: v.anbieter.map(|b| Box::new((*b).into())),
            customer: v.kunde.map(|b| Box::new((*b).into())),
            tender_id: v.ausschreibung_id,
        }
    }
}
