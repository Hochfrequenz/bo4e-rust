#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Energiemix {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyMixNumber")]
    pub energiemixnummer: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "division")]
    pub sparte: Option<crate::Sparte>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "designation")]
    pub bezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validYear")]
    pub gueltigkeitsjahr: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "sources")]
    pub anteil: Vec<crate::Energieherkunft>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "notes")]
    pub bemerkung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_emission: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "nuclearWaste")]
    pub atommuell: Option<f64>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        alias = "ecoCertificates"
    )]
    pub oekozertifikate: Vec<crate::Oekozertifikat>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", alias = "ecoLabels")]
    pub oekolabel: Vec<crate::Oekolabel>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "inEcoTopTen")]
    pub ist_in_oeko_top_ten: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}
impl From<bo4e_core::com::EnergyMix> for Energiemix {
    fn from(v: bo4e_core::com::EnergyMix) -> Self {
        Self {
            meta: v.meta,
            energiemixnummer: v.energy_mix_number,
            sparte: v.division.map(Into::into),
            bezeichnung: v.designation,
            gueltigkeitsjahr: v.valid_year,
            anteil: v.sources.into_iter().map(Into::into).collect(),
            bemerkung: v.notes,
            co2_emission: v.co2_emission,
            atommuell: v.nuclear_waste,
            oekozertifikate: v.eco_certificates.into_iter().map(Into::into).collect(),
            oekolabel: v.eco_labels.into_iter().map(Into::into).collect(),
            ist_in_oeko_top_ten: v.in_eco_top_ten,
            website: v.website,
        }
    }
}
impl From<Energiemix> for bo4e_core::com::EnergyMix {
    fn from(v: Energiemix) -> Self {
        Self {
            meta: v.meta,
            energy_mix_number: v.energiemixnummer,
            division: v.sparte.map(Into::into),
            designation: v.bezeichnung,
            valid_year: v.gueltigkeitsjahr,
            sources: v.anteil.into_iter().map(Into::into).collect(),
            notes: v.bemerkung,
            co2_emission: v.co2_emission,
            nuclear_waste: v.atommuell,
            eco_certificates: v.oekozertifikate.into_iter().map(Into::into).collect(),
            eco_labels: v.oekolabel.into_iter().map(Into::into).collect(),
            in_eco_top_ten: v.ist_in_oeko_top_ten,
            website: v.website,
        }
    }
}
