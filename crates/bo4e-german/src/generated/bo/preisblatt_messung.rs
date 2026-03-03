#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreisblattMessung {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "meterType")]
    pub zaehlerart: Option<crate::Zaehlertyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "priceSheetNumber")]
    pub preisblattnummer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validFrom")]
    pub gueltig_ab: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validUntil")]
    pub gueltig_bis: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "meteringPointOperationPrice"
    )]
    pub messstellenbetrieb: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "readingPrice")]
    pub ablesepreis: Option<crate::Preis>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "positions")]
    pub preispositionen: Vec<crate::Preisposition>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "operator")]
    pub messstellenbetreiber: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::MeteringPriceSheet> for PreisblattMessung {
    fn from(v: bo4e_core::bo::MeteringPriceSheet) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            zaehlerart: v.meter_type.map(Into::into),
            preisblattnummer: v.price_sheet_number,
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            gueltig_ab: v.valid_from,
            gueltig_bis: v.valid_until,
            messstellenbetrieb: v.metering_point_operation_price.map(Into::into),
            ablesepreis: v.reading_price.map(Into::into),
            preispositionen: v.positions.into_iter().map(Into::into).collect(),
            messstellenbetreiber: v.operator.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<PreisblattMessung> for bo4e_core::bo::MeteringPriceSheet {
    fn from(v: PreisblattMessung) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            meter_type: v.zaehlerart.map(Into::into),
            price_sheet_number: v.preisblattnummer,
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            valid_from: v.gueltig_ab,
            valid_until: v.gueltig_bis,
            metering_point_operation_price: v.messstellenbetrieb.map(Into::into),
            reading_price: v.ablesepreis.map(Into::into),
            positions: v.preispositionen.into_iter().map(Into::into).collect(),
            operator: v.messstellenbetreiber.map(|b| Box::new((*b).into())),
        }
    }
}
