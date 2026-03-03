#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tarif {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "tariffName")]
    pub tarifname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub tarifbeschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "customerType")]
    pub kundentyp: Option<crate::Kundentyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validityPeriod")]
    pub gueltigkeitszeitraum: Option<crate::Zeitraum>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "basePrice")]
    pub grundpreis: Option<crate::Preis>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "workingPrice")]
    pub arbeitspreis: Option<crate::Preis>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "priceTiers")]
    pub preisstaffeln: Vec<crate::Preisstaffel>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        alias = "calculationParameters"
    )]
    pub tarifberechnungsparameter: Option<crate::Tarifberechnungsparameter>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyMix")]
    pub energiemix: Option<crate::Energiemix>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "supplier")]
    pub anbieter: Option<Box<crate::Geschaeftspartner>>,
}
impl From<bo4e_core::bo::Tariff> for Tarif {
    fn from(v: bo4e_core::bo::Tariff) -> Self {
        Self {
            meta: v.meta,
            tarifname: v.tariff_name,
            tarifbeschreibung: v.description,
            sparte: v.division.map(Into::into),
            kundentyp: v.customer_type.map(Into::into),
            gueltigkeitszeitraum: v.validity_period.map(Into::into),
            grundpreis: v.base_price.map(Into::into),
            arbeitspreis: v.working_price.map(Into::into),
            preisstaffeln: v.price_tiers.into_iter().map(Into::into).collect(),
            tarifberechnungsparameter: v.calculation_parameters.map(Into::into),
            energiemix: v.energy_mix.map(Into::into),
            anbieter: v.supplier.map(|b| Box::new((*b).into())),
        }
    }
}
impl From<Tarif> for bo4e_core::bo::Tariff {
    fn from(v: Tarif) -> Self {
        Self {
            meta: v.meta,
            tariff_name: v.tarifname,
            description: v.tarifbeschreibung,
            division: v.sparte.map(Into::into),
            customer_type: v.kundentyp.map(Into::into),
            validity_period: v.gueltigkeitszeitraum.map(Into::into),
            base_price: v.grundpreis.map(Into::into),
            working_price: v.arbeitspreis.map(Into::into),
            price_tiers: v.preisstaffeln.into_iter().map(Into::into).collect(),
            calculation_parameters: v.tarifberechnungsparameter.map(Into::into),
            energy_mix: v.energiemix.map(Into::into),
            supplier: v.anbieter.map(|b| Box::new((*b).into())),
        }
    }
}
