#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Vertragsstatus {
    #[serde(rename = "IN_ARBEIT")]
    InArbeit,
    #[serde(rename = "UEBERMITTELT")]
    Uebermittelt,
    #[serde(rename = "ANGENOMMEN")]
    Angenommen,
    #[serde(rename = "AKTIV")]
    Aktiv,
    #[serde(rename = "ABGELEHNT")]
    Abgelehnt,
    #[serde(rename = "WIDERRUFEN")]
    Widerrufen,
    #[serde(rename = "STORNIERT")]
    Storniert,
    #[serde(rename = "GEKUENDIGT")]
    Gekuendigt,
    #[serde(rename = "BEENDET")]
    Beendet,
}
impl From<bo4e_core::enums::ContractStatus> for Vertragsstatus {
    fn from(v: bo4e_core::enums::ContractStatus) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ContractStatus::InProgress => Vertragsstatus::InArbeit,
            bo4e_core::enums::ContractStatus::Transmitted => Vertragsstatus::Uebermittelt,
            bo4e_core::enums::ContractStatus::Accepted => Vertragsstatus::Angenommen,
            bo4e_core::enums::ContractStatus::Active => Vertragsstatus::Aktiv,
            bo4e_core::enums::ContractStatus::Rejected => Vertragsstatus::Abgelehnt,
            bo4e_core::enums::ContractStatus::Revoked => Vertragsstatus::Widerrufen,
            bo4e_core::enums::ContractStatus::Cancelled => Vertragsstatus::Storniert,
            bo4e_core::enums::ContractStatus::Terminated => Vertragsstatus::Gekuendigt,
            bo4e_core::enums::ContractStatus::Ended => Vertragsstatus::Beendet,
            _ => panic!("Unknown {} variant", stringify!(ContractStatus)),
        }
    }
}
impl From<Vertragsstatus> for bo4e_core::enums::ContractStatus {
    fn from(v: Vertragsstatus) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Vertragsstatus::InArbeit => bo4e_core::enums::ContractStatus::InProgress,
            Vertragsstatus::Uebermittelt => bo4e_core::enums::ContractStatus::Transmitted,
            Vertragsstatus::Angenommen => bo4e_core::enums::ContractStatus::Accepted,
            Vertragsstatus::Aktiv => bo4e_core::enums::ContractStatus::Active,
            Vertragsstatus::Abgelehnt => bo4e_core::enums::ContractStatus::Rejected,
            Vertragsstatus::Widerrufen => bo4e_core::enums::ContractStatus::Revoked,
            Vertragsstatus::Storniert => bo4e_core::enums::ContractStatus::Cancelled,
            Vertragsstatus::Gekuendigt => bo4e_core::enums::ContractStatus::Terminated,
            Vertragsstatus::Beendet => bo4e_core::enums::ContractStatus::Ended,
        }
    }
}
