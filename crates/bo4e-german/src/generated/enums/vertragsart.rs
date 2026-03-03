#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Vertragsart {
    #[serde(rename = "ENERGIELIEFERVERTRAG")]
    Energieliefervertrag,
    #[serde(rename = "NETZNUTZUNGSVERTRAG")]
    Netznutzungsvertrag,
    #[serde(rename = "BILANZIERUNGSVERTRAG")]
    Bilanzierungsvertrag,
    #[serde(rename = "MESSSTELLENBETRIEBSVERTRAG")]
    Messstellenbetriebsvertrag,
    #[serde(rename = "BUENDELVERTRAG")]
    Buendelvertrag,
}
impl From<bo4e_core::enums::ContractType> for Vertragsart {
    fn from(v: bo4e_core::enums::ContractType) -> Self {
        match v {
            bo4e_core::enums::ContractType::EnergySupplyContract => {
                Vertragsart::Energieliefervertrag
            }
            bo4e_core::enums::ContractType::NetworkUsageContract => {
                Vertragsart::Netznutzungsvertrag
            }
            bo4e_core::enums::ContractType::BalancingContract => {
                Vertragsart::Bilanzierungsvertrag
            }
            bo4e_core::enums::ContractType::MeteringPointOperationContract => {
                Vertragsart::Messstellenbetriebsvertrag
            }
            bo4e_core::enums::ContractType::BundleContract => Vertragsart::Buendelvertrag,
            _ => panic!("Unknown {} variant", stringify!(ContractType)),
        }
    }
}
impl From<Vertragsart> for bo4e_core::enums::ContractType {
    fn from(v: Vertragsart) -> Self {
        match v {
            Vertragsart::Energieliefervertrag => {
                bo4e_core::enums::ContractType::EnergySupplyContract
            }
            Vertragsart::Netznutzungsvertrag => {
                bo4e_core::enums::ContractType::NetworkUsageContract
            }
            Vertragsart::Bilanzierungsvertrag => {
                bo4e_core::enums::ContractType::BalancingContract
            }
            Vertragsart::Messstellenbetriebsvertrag => {
                bo4e_core::enums::ContractType::MeteringPointOperationContract
            }
            Vertragsart::Buendelvertrag => bo4e_core::enums::ContractType::BundleContract,
            _ => panic!("Unknown {} variant", stringify!(Vertragsart)),
        }
    }
}
