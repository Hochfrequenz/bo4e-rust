#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validierungsergebnis {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validationTimestamp")]
    pub validierungszeitpunkt: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "isValid")]
    pub gueltig: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validationRuleId")]
    pub validierungsregel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "validationRuleName")]
    pub regelbezeichnung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "errorCode")]
    pub fehlercode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "errorMessage")]
    pub fehlermeldung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "severity")]
    pub schweregrad: Option<String>,
}
impl From<bo4e_core::com::ValidationResult> for Validierungsergebnis {
    fn from(v: bo4e_core::com::ValidationResult) -> Self {
        Self {
            meta: v.meta,
            validierungszeitpunkt: v.validation_timestamp,
            gueltig: v.is_valid,
            validierungsregel: v.validation_rule_id,
            regelbezeichnung: v.validation_rule_name,
            fehlercode: v.error_code,
            fehlermeldung: v.error_message,
            schweregrad: v.severity,
        }
    }
}
impl From<Validierungsergebnis> for bo4e_core::com::ValidationResult {
    fn from(v: Validierungsergebnis) -> Self {
        Self {
            meta: v.meta,
            validation_timestamp: v.validierungszeitpunkt,
            is_valid: v.gueltig,
            validation_rule_id: v.validierungsregel,
            validation_rule_name: v.regelbezeichnung,
            error_code: v.fehlercode,
            error_message: v.fehlermeldung,
            severity: v.schweregrad,
        }
    }
}
