#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zaehlwerksstand {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "timestamp")]
    pub ablesezeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "value")]
    pub zaehlwerksstand: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "readingType")]
    pub ableseart: Option<crate::Ableseart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::Messwertstatus>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "registerId")]
    pub zaehlwerkskennung: Option<String>,
}
impl From<bo4e_core::com::MeterReading> for Zaehlwerksstand {
    fn from(v: bo4e_core::com::MeterReading) -> Self {
        Self {
            meta: v.meta,
            ablesezeitpunkt: v.timestamp,
            zaehlwerksstand: v.value,
            einheit: v.unit.map(Into::into),
            ableseart: v.reading_type.map(Into::into),
            status: v.status.map(Into::into),
            obis_kennzahl: v.obis_code,
            zaehlwerkskennung: v.register_id,
        }
    }
}
impl From<Zaehlwerksstand> for bo4e_core::com::MeterReading {
    fn from(v: Zaehlwerksstand) -> Self {
        Self {
            meta: v.meta,
            timestamp: v.ablesezeitpunkt,
            value: v.zaehlwerksstand,
            unit: v.einheit.map(Into::into),
            reading_type: v.ableseart.map(Into::into),
            status: v.status.map(Into::into),
            obis_code: v.obis_kennzahl,
            register_id: v.zaehlwerkskennung,
        }
    }
}
