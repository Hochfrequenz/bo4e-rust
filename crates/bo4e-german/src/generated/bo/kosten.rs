#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kosten {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "costBlocks")]
    pub kostenbloecke: Vec<crate::Kostenblock>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "marketLocation")]
    pub marktlokation: Option<Box<crate::Marktlokation>>,
}
impl From<bo4e_core::bo::Costs> for Kosten {
    fn from(v: bo4e_core::bo::Costs) -> Self {
        Self {
            meta: v.meta,
            bezeichnung: v.designation,
            beschreibung: v.description,
            sparte: v.division.map(Into::into),
            abrechnungszeitraum: v.period.map(Into::into),
            gesamtbetrag: v.total_amount.map(Into::into),
            kostenbloecke: v.cost_blocks.into_iter().map(Into::into).collect(),
            marktlokation: v.market_location.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Kosten> for bo4e_core::bo::Costs {
    fn from(v: Kosten) -> Self {
        Self {
            meta: v.meta,
            designation: v.bezeichnung,
            description: v.beschreibung,
            division: v.sparte.map(Into::into),
            period: v.abrechnungszeitraum.map(Into::into),
            total_amount: v.gesamtbetrag.map(Into::into),
            cost_blocks: v.kostenbloecke.into_iter().map(Into::into).collect(),
            market_location: v.marktlokation.map(|b| Box::new((*b).into())),
        }
    }
}
