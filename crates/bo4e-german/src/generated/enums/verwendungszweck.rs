#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Verwendungszweck {
    #[serde(rename = "NETZNUTZUNGSABRECHNUNG")]
    Netznutzungsabrechnung,
    #[serde(rename = "BILANZKREISABRECHNUNG")]
    Bilanzkreisabrechnung,
    #[serde(rename = "MEHRMINDERMENGENABRECHNUNG")]
    Mehrmindermengenabrechnung,
    #[serde(rename = "ENDKUNDENABRECHNUNG")]
    Endkundenabrechnung,
    #[serde(rename = "UEBERMITTLUNG_AN_DAS_HKNR")]
    BermittlungAnDasHknr,
    #[serde(rename = "ERMITTLUNG_AUSGEGLICHENHEIT_BILANZKREIS")]
    ErmittlungAusgeglichenheitBilanzkreis,
}
impl From<bo4e_core::enums::UsageType> for Verwendungszweck {
    fn from(v: bo4e_core::enums::UsageType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::UsageType::NetworkUsageBilling => {
                Verwendungszweck::Netznutzungsabrechnung
            }
            bo4e_core::enums::UsageType::BalancingGroupBilling => {
                Verwendungszweck::Bilanzkreisabrechnung
            }
            bo4e_core::enums::UsageType::MoreLessQuantityBilling => {
                Verwendungszweck::Mehrmindermengenabrechnung
            }
            bo4e_core::enums::UsageType::EndCustomerBilling => {
                Verwendungszweck::Endkundenabrechnung
            }
            bo4e_core::enums::UsageType::TransmissionToOriginRegistry => {
                Verwendungszweck::BermittlungAnDasHknr
            }
            bo4e_core::enums::UsageType::BalancingGroupBalanceDetermination => {
                Verwendungszweck::ErmittlungAusgeglichenheitBilanzkreis
            }
            _ => panic!("Unknown {} variant", stringify!(UsageType)),
        }
    }
}
impl From<Verwendungszweck> for bo4e_core::enums::UsageType {
    fn from(v: Verwendungszweck) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Verwendungszweck::Netznutzungsabrechnung => {
                bo4e_core::enums::UsageType::NetworkUsageBilling
            }
            Verwendungszweck::Bilanzkreisabrechnung => {
                bo4e_core::enums::UsageType::BalancingGroupBilling
            }
            Verwendungszweck::Mehrmindermengenabrechnung => {
                bo4e_core::enums::UsageType::MoreLessQuantityBilling
            }
            Verwendungszweck::Endkundenabrechnung => {
                bo4e_core::enums::UsageType::EndCustomerBilling
            }
            Verwendungszweck::BermittlungAnDasHknr => {
                bo4e_core::enums::UsageType::TransmissionToOriginRegistry
            }
            Verwendungszweck::ErmittlungAusgeglichenheitBilanzkreis => {
                bo4e_core::enums::UsageType::BalancingGroupBalanceDetermination
            }
        }
    }
}
