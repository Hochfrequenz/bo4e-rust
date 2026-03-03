#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Qualitaetsindikator {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "qualityCode")]
    pub qualitaetscode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::Messwertstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "qualityDescription")]
    pub qualitaetsbeschreibung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "confidencePercent")]
    pub konfidenz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "dataSource")]
    pub datenquelle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isSubstituted")]
    pub ersetzt: Option<bool>,
}
impl From<bo4e_core::com::QualityIndicator> for Qualitaetsindikator {
    fn from(v: bo4e_core::com::QualityIndicator) -> Self {
        Self {
            meta: v.meta,
            qualitaetscode: v.quality_code,
            status: v.status.map(Into::into),
            qualitaetsbeschreibung: v.quality_description,
            konfidenz: v.confidence_percent,
            datenquelle: v.data_source,
            ersetzt: v.is_substituted,
        }
    }
}
impl From<Qualitaetsindikator> for bo4e_core::com::QualityIndicator {
    fn from(v: Qualitaetsindikator) -> Self {
        Self {
            meta: v.meta,
            quality_code: v.qualitaetscode,
            status: v.status.map(Into::into),
            quality_description: v.qualitaetsbeschreibung,
            confidence_percent: v.konfidenz,
            data_source: v.datenquelle,
            is_substituted: v.ersetzt,
        }
    }
}
