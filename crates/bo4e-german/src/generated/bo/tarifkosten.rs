#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarifkosten {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub beschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "period")]
    pub abrechnungszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "totalAmount")]
    pub gesamtbetrag: Option<crate::Betrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "basePrice")]
    pub grundpreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "basePriceCost")]
    pub grundpreiskosten: Option<crate::Betrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "workingPrice")]
    pub arbeitspreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "workingPriceCost")]
    pub arbeitspreiskosten: Option<crate::Betrag>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "consumption")]
    pub verbrauchsmenge: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "costBlocks")]
    pub kostenbloecke: Vec<crate::Kostenblock>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariff")]
    pub tarif: Option<Box<crate::Tarif>>,
}
impl From<bo4e_core::bo::TariffCosts> for Tarifkosten {
    fn from(v: bo4e_core::bo::TariffCosts) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            abrechnungszeitraum: v.period.map(Into::into),
            gesamtbetrag: v.total_amount.map(Into::into),
            grundpreis: v.base_price.map(Into::into),
            grundpreiskosten: v.base_price_cost.map(Into::into),
            arbeitspreis: v.working_price.map(Into::into),
            arbeitspreiskosten: v.working_price_cost.map(Into::into),
            verbrauchsmenge: v.consumption,
            kostenbloecke: v.cost_blocks.into_iter().map(Into::into).collect(),
            tarif: v.tariff.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Tarifkosten> for bo4e_core::bo::TariffCosts {
    fn from(v: Tarifkosten) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            period: v.abrechnungszeitraum.map(Into::into),
            total_amount: v.gesamtbetrag.map(Into::into),
            base_price: v.grundpreis.map(Into::into),
            base_price_cost: v.grundpreiskosten.map(Into::into),
            working_price: v.arbeitspreis.map(Into::into),
            working_price_cost: v.arbeitspreiskosten.map(Into::into),
            consumption: v.verbrauchsmenge,
            cost_blocks: v.kostenbloecke.into_iter().map(Into::into).collect(),
            tariff: v.tarif.map(|b| Box::new((*b).into())),
        }
    }
}
