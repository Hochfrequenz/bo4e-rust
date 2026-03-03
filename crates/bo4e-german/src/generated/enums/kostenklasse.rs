#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Kostenklasse {
    #[serde(rename = "FREMDKOSTEN")]
    Fremdkosten,
    #[serde(rename = "BESCHAFFUNG")]
    Beschaffung,
    #[serde(rename = "SELBSTKOSTEN")]
    Selbstkosten,
    #[serde(rename = "MARGEN")]
    Margen,
    #[serde(rename = "ENERGIEVERSORGUNGSKOSTEN")]
    Energieversorgungskosten,
}
impl From<bo4e_core::enums::CostClass> for Kostenklasse {
    fn from(v: bo4e_core::enums::CostClass) -> Self {
        match v {
            bo4e_core::enums::CostClass::ExternalCosts => Kostenklasse::Fremdkosten,
            bo4e_core::enums::CostClass::Procurement => Kostenklasse::Beschaffung,
            bo4e_core::enums::CostClass::InternalCosts => Kostenklasse::Selbstkosten,
            bo4e_core::enums::CostClass::Margins => Kostenklasse::Margen,
            bo4e_core::enums::CostClass::EnergySupplyCosts => {
                Kostenklasse::Energieversorgungskosten
            }
            _ => panic!("Unknown {} variant", stringify!(CostClass)),
        }
    }
}
impl From<Kostenklasse> for bo4e_core::enums::CostClass {
    fn from(v: Kostenklasse) -> Self {
        match v {
            Kostenklasse::Fremdkosten => bo4e_core::enums::CostClass::ExternalCosts,
            Kostenklasse::Beschaffung => bo4e_core::enums::CostClass::Procurement,
            Kostenklasse::Selbstkosten => bo4e_core::enums::CostClass::InternalCosts,
            Kostenklasse::Margen => bo4e_core::enums::CostClass::Margins,
            Kostenklasse::Energieversorgungskosten => {
                bo4e_core::enums::CostClass::EnergySupplyCosts
            }
            _ => panic!("Unknown {} variant", stringify!(Kostenklasse)),
        }
    }
}
