#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ausschreibung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tenderNumber")]
    pub ausschreibungsnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tenderType")]
    pub ausschreibungstyp: Option<crate::Ausschreibungstyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "status")]
    pub ausschreibungsstatus: Option<crate::Ausschreibungsstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "publicationDate")]
    pub veroeffentlichungsdatum: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "submissionDeadline")]
    pub abgabefrist: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "deliveryPeriod")]
    pub lieferzeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tenderingParty")]
    pub ausschreibender: Option<Box<crate::Geschaeftspartner>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "estimatedAnnualConsumption"
    )]
    pub jahresverbrauch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "numberOfDeliveryPoints")]
    pub anzahl_lieferstellen: Option<i32>,
}
impl From<bo4e_core::bo::Tender> for Ausschreibung {
    fn from(v: bo4e_core::bo::Tender) -> Self {
        Self {
            meta: v.meta,
            ausschreibungsnummer: v.tender_number,
            beschreibung: v.description,
            ausschreibungstyp: v.tender_type.map(Into::into),
            ausschreibungsstatus: v.status.map(Into::into),
            sparte: v.division.map(Into::into),
            veroeffentlichungsdatum: v.publication_date,
            abgabefrist: v.submission_deadline,
            lieferzeitraum: v.delivery_period.map(Into::into),
            ausschreibender: v.tendering_party.map(|b| Box::new((*b).into())),
            jahresverbrauch: v.estimated_annual_consumption,
            anzahl_lieferstellen: v.number_of_delivery_points,
        }
    }
}
impl From<Ausschreibung> for bo4e_core::bo::Tender {
    fn from(v: Ausschreibung) -> Self {
        Self {
            meta: v.meta,
            tender_number: v.ausschreibungsnummer,
            description: v.beschreibung,
            tender_type: v.ausschreibungstyp.map(Into::into),
            status: v.ausschreibungsstatus.map(Into::into),
            division: v.sparte.map(Into::into),
            publication_date: v.veroeffentlichungsdatum,
            submission_deadline: v.abgabefrist,
            delivery_period: v.lieferzeitraum.map(Into::into),
            tendering_party: v.ausschreibender.map(|b| Box::new((*b).into())),
            estimated_annual_consumption: v.jahresverbrauch,
            number_of_delivery_points: v.anzahl_lieferstellen,
        }
    }
}
