#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Regionskriteriumtyp {
    #[serde(rename = "BUNDESLANDKENNZIFFER")]
    Bundeslandkennziffer,
    #[serde(rename = "BUNDESLAND_NAME")]
    BundeslandName,
    #[serde(rename = "MARKTGEBIET_NUMMER")]
    MarktgebietCodenummer,
    #[serde(rename = "MARKTGEBIET_NAME")]
    MarktgebietName,
    #[serde(rename = "REGELGEBIET_NUMMER")]
    RegelgebietNummer,
    #[serde(rename = "REGELGEBIET_NAME")]
    RegelgebietName,
    #[serde(rename = "NETZ_STROM")]
    NetzStrom,
    #[serde(rename = "NETZ_GAS")]
    NetzGas,
    #[serde(rename = "NETZBETREIBER_NUMMER_STROM")]
    NetzbetreiberCodenummerStrom,
    #[serde(rename = "NETZBETREIBER_NUMMER_GAS")]
    NetzbetreiberCodenummerGas,
    #[serde(rename = "NETZBETREIBER_NAME_STROM")]
    NetzbetreiberNameStrom,
    #[serde(rename = "NETZBETREIBER_NAME_GAS")]
    NetzbetreiberNameGas,
    #[serde(rename = "BILANZIERUNGS_GEBIET_NUMMER")]
    ElectricityBilanzierungsgebietsnummerGasNetzkontonummer,
    #[serde(rename = "MSB_NUMMER")]
    MSBCodenummer,
    #[serde(rename = "MSB_NAME")]
    MSBName,
    #[serde(rename = "VERSORGER_NUMMER")]
    LieferantenCodenummer,
    #[serde(rename = "VERSORGER_NAME")]
    VersorgerName,
    #[serde(rename = "GRUNDVERSORGER_NUMMER_STROM")]
    StromGrundversorgerCodenummer,
    #[serde(rename = "GRUNDVERSORGER_NAME_STROM")]
    StromGrundversorgerName,
    #[serde(rename = "GRUNDVERSORGER_NUMMER_GAS")]
    GasGrundversorgerCodenummer,
    #[serde(rename = "GRUNDVERSORGER_NAME_GAS")]
    GasGrundversorgerName,
    #[serde(rename = "KREIS_NAME")]
    Kreis,
    #[serde(rename = "KREISKENNZIFFER")]
    Kreiskennziffer,
    #[serde(rename = "GEMEINDE_NAME")]
    Gemeinde,
    #[serde(rename = "GEMEINDEKENNZIFFER")]
    Gemeindekennziffer,
    #[serde(rename = "POSTLEITZAHL")]
    Postleitzahl,
    #[serde(rename = "ORT")]
    Ort,
    #[serde(rename = "POSTORT")]
    PostleitzahlUndOrt,
    #[serde(rename = "EINWOHNERZAHL_GEMEINDE")]
    EinwohnerzahlGemeinde,
    #[serde(rename = "EINWOHNERZAHL_ORT")]
    EinwohnerzahlOrt,
    #[serde(rename = "KM_UMKREIS")]
    KmUmkreis,
    #[serde(rename = "BUNDESWEIT")]
    Bundesweit,
    #[serde(rename = "PLZ_BEREICH")]
    Postleitzahlenbereich,
}
impl From<bo4e_core::enums::RegionCriterionType> for Regionskriteriumtyp {
    fn from(v: bo4e_core::enums::RegionCriterionType) -> Self {
        match v {
            bo4e_core::enums::RegionCriterionType::FederalStateCode => {
                Regionskriteriumtyp::Bundeslandkennziffer
            }
            bo4e_core::enums::RegionCriterionType::FederalStateName => {
                Regionskriteriumtyp::BundeslandName
            }
            bo4e_core::enums::RegionCriterionType::MarketAreaNumber => {
                Regionskriteriumtyp::MarktgebietCodenummer
            }
            bo4e_core::enums::RegionCriterionType::MarketAreaName => {
                Regionskriteriumtyp::MarktgebietName
            }
            bo4e_core::enums::RegionCriterionType::ControlAreaNumber => {
                Regionskriteriumtyp::RegelgebietNummer
            }
            bo4e_core::enums::RegionCriterionType::ControlAreaName => {
                Regionskriteriumtyp::RegelgebietName
            }
            bo4e_core::enums::RegionCriterionType::ElectricityNetwork => {
                Regionskriteriumtyp::NetzStrom
            }
            bo4e_core::enums::RegionCriterionType::GasNetwork => {
                Regionskriteriumtyp::NetzGas
            }
            bo4e_core::enums::RegionCriterionType::ElectricityNetworkOperatorNumber => {
                Regionskriteriumtyp::NetzbetreiberCodenummerStrom
            }
            bo4e_core::enums::RegionCriterionType::GasNetworkOperatorNumber => {
                Regionskriteriumtyp::NetzbetreiberCodenummerGas
            }
            bo4e_core::enums::RegionCriterionType::ElectricityNetworkOperatorName => {
                Regionskriteriumtyp::NetzbetreiberNameStrom
            }
            bo4e_core::enums::RegionCriterionType::GasNetworkOperatorName => {
                Regionskriteriumtyp::NetzbetreiberNameGas
            }
            bo4e_core::enums::RegionCriterionType::BalancingAreaNumber => {
                Regionskriteriumtyp::ElectricityBilanzierungsgebietsnummerGasNetzkontonummer
            }
            bo4e_core::enums::RegionCriterionType::MeteringServiceOperatorNumber => {
                Regionskriteriumtyp::MSBCodenummer
            }
            bo4e_core::enums::RegionCriterionType::MeteringServiceOperatorName => {
                Regionskriteriumtyp::MSBName
            }
            bo4e_core::enums::RegionCriterionType::SupplierNumber => {
                Regionskriteriumtyp::LieferantenCodenummer
            }
            bo4e_core::enums::RegionCriterionType::SupplierName => {
                Regionskriteriumtyp::VersorgerName
            }
            bo4e_core::enums::RegionCriterionType::ElectricityBasicSupplierNumber => {
                Regionskriteriumtyp::StromGrundversorgerCodenummer
            }
            bo4e_core::enums::RegionCriterionType::ElectricityBasicSupplierName => {
                Regionskriteriumtyp::StromGrundversorgerName
            }
            bo4e_core::enums::RegionCriterionType::GasBasicSupplierNumber => {
                Regionskriteriumtyp::GasGrundversorgerCodenummer
            }
            bo4e_core::enums::RegionCriterionType::GasBasicSupplierName => {
                Regionskriteriumtyp::GasGrundversorgerName
            }
            bo4e_core::enums::RegionCriterionType::DistrictName => {
                Regionskriteriumtyp::Kreis
            }
            bo4e_core::enums::RegionCriterionType::DistrictCode => {
                Regionskriteriumtyp::Kreiskennziffer
            }
            bo4e_core::enums::RegionCriterionType::MunicipalityName => {
                Regionskriteriumtyp::Gemeinde
            }
            bo4e_core::enums::RegionCriterionType::MunicipalityCode => {
                Regionskriteriumtyp::Gemeindekennziffer
            }
            bo4e_core::enums::RegionCriterionType::PostalCode => {
                Regionskriteriumtyp::Postleitzahl
            }
            bo4e_core::enums::RegionCriterionType::City => Regionskriteriumtyp::Ort,
            bo4e_core::enums::RegionCriterionType::PostalCity => {
                Regionskriteriumtyp::PostleitzahlUndOrt
            }
            bo4e_core::enums::RegionCriterionType::MunicipalityPopulation => {
                Regionskriteriumtyp::EinwohnerzahlGemeinde
            }
            bo4e_core::enums::RegionCriterionType::CityPopulation => {
                Regionskriteriumtyp::EinwohnerzahlOrt
            }
            bo4e_core::enums::RegionCriterionType::RadiusKm => {
                Regionskriteriumtyp::KmUmkreis
            }
            bo4e_core::enums::RegionCriterionType::Nationwide => {
                Regionskriteriumtyp::Bundesweit
            }
            bo4e_core::enums::RegionCriterionType::PostalCodeRange => {
                Regionskriteriumtyp::Postleitzahlenbereich
            }
            _ => panic!("Unknown {} variant", stringify!(RegionCriterionType)),
        }
    }
}
impl From<Regionskriteriumtyp> for bo4e_core::enums::RegionCriterionType {
    fn from(v: Regionskriteriumtyp) -> Self {
        match v {
            Regionskriteriumtyp::Bundeslandkennziffer => {
                bo4e_core::enums::RegionCriterionType::FederalStateCode
            }
            Regionskriteriumtyp::BundeslandName => {
                bo4e_core::enums::RegionCriterionType::FederalStateName
            }
            Regionskriteriumtyp::MarktgebietCodenummer => {
                bo4e_core::enums::RegionCriterionType::MarketAreaNumber
            }
            Regionskriteriumtyp::MarktgebietName => {
                bo4e_core::enums::RegionCriterionType::MarketAreaName
            }
            Regionskriteriumtyp::RegelgebietNummer => {
                bo4e_core::enums::RegionCriterionType::ControlAreaNumber
            }
            Regionskriteriumtyp::RegelgebietName => {
                bo4e_core::enums::RegionCriterionType::ControlAreaName
            }
            Regionskriteriumtyp::NetzStrom => {
                bo4e_core::enums::RegionCriterionType::ElectricityNetwork
            }
            Regionskriteriumtyp::NetzGas => {
                bo4e_core::enums::RegionCriterionType::GasNetwork
            }
            Regionskriteriumtyp::NetzbetreiberCodenummerStrom => {
                bo4e_core::enums::RegionCriterionType::ElectricityNetworkOperatorNumber
            }
            Regionskriteriumtyp::NetzbetreiberCodenummerGas => {
                bo4e_core::enums::RegionCriterionType::GasNetworkOperatorNumber
            }
            Regionskriteriumtyp::NetzbetreiberNameStrom => {
                bo4e_core::enums::RegionCriterionType::ElectricityNetworkOperatorName
            }
            Regionskriteriumtyp::NetzbetreiberNameGas => {
                bo4e_core::enums::RegionCriterionType::GasNetworkOperatorName
            }
            Regionskriteriumtyp::ElectricityBilanzierungsgebietsnummerGasNetzkontonummer => {
                bo4e_core::enums::RegionCriterionType::BalancingAreaNumber
            }
            Regionskriteriumtyp::MSBCodenummer => {
                bo4e_core::enums::RegionCriterionType::MeteringServiceOperatorNumber
            }
            Regionskriteriumtyp::MSBName => {
                bo4e_core::enums::RegionCriterionType::MeteringServiceOperatorName
            }
            Regionskriteriumtyp::LieferantenCodenummer => {
                bo4e_core::enums::RegionCriterionType::SupplierNumber
            }
            Regionskriteriumtyp::VersorgerName => {
                bo4e_core::enums::RegionCriterionType::SupplierName
            }
            Regionskriteriumtyp::StromGrundversorgerCodenummer => {
                bo4e_core::enums::RegionCriterionType::ElectricityBasicSupplierNumber
            }
            Regionskriteriumtyp::StromGrundversorgerName => {
                bo4e_core::enums::RegionCriterionType::ElectricityBasicSupplierName
            }
            Regionskriteriumtyp::GasGrundversorgerCodenummer => {
                bo4e_core::enums::RegionCriterionType::GasBasicSupplierNumber
            }
            Regionskriteriumtyp::GasGrundversorgerName => {
                bo4e_core::enums::RegionCriterionType::GasBasicSupplierName
            }
            Regionskriteriumtyp::Kreis => {
                bo4e_core::enums::RegionCriterionType::DistrictName
            }
            Regionskriteriumtyp::Kreiskennziffer => {
                bo4e_core::enums::RegionCriterionType::DistrictCode
            }
            Regionskriteriumtyp::Gemeinde => {
                bo4e_core::enums::RegionCriterionType::MunicipalityName
            }
            Regionskriteriumtyp::Gemeindekennziffer => {
                bo4e_core::enums::RegionCriterionType::MunicipalityCode
            }
            Regionskriteriumtyp::Postleitzahl => {
                bo4e_core::enums::RegionCriterionType::PostalCode
            }
            Regionskriteriumtyp::Ort => bo4e_core::enums::RegionCriterionType::City,
            Regionskriteriumtyp::PostleitzahlUndOrt => {
                bo4e_core::enums::RegionCriterionType::PostalCity
            }
            Regionskriteriumtyp::EinwohnerzahlGemeinde => {
                bo4e_core::enums::RegionCriterionType::MunicipalityPopulation
            }
            Regionskriteriumtyp::EinwohnerzahlOrt => {
                bo4e_core::enums::RegionCriterionType::CityPopulation
            }
            Regionskriteriumtyp::KmUmkreis => {
                bo4e_core::enums::RegionCriterionType::RadiusKm
            }
            Regionskriteriumtyp::Bundesweit => {
                bo4e_core::enums::RegionCriterionType::Nationwide
            }
            Regionskriteriumtyp::Postleitzahlenbereich => {
                bo4e_core::enums::RegionCriterionType::PostalCodeRange
            }
            _ => panic!("Unknown {} variant", stringify!(Regionskriteriumtyp)),
        }
    }
}
