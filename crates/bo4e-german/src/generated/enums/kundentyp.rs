#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Kundentyp {
    #[serde(rename = "GEWERBE")]
    Gewerbe,
    #[serde(rename = "PRIVAT")]
    Privat,
    #[serde(rename = "LANDWIRT")]
    Landwirt,
    #[serde(rename = "SONSTIGE")]
    Sonstige,
    #[serde(rename = "HAUSHALT")]
    Haushalt,
    #[serde(rename = "DIREKTHEIZUNG")]
    Direktheizung,
    #[serde(rename = "GEMEINSCHAFT_MFH")]
    GemeinschaftMFH,
    #[serde(rename = "KIRCHE")]
    Kirche,
    #[serde(rename = "KWK")]
    KWKAnlagen,
    #[serde(rename = "LADESAEULE")]
    Ladesaeule,
    #[serde(rename = "BELEUCHTUNG_OEFFENTLICH")]
    OeffentlicheBeleuchtung,
    #[serde(rename = "BELEUCHTUNG_STRASSE")]
    Strassenbeleuchtung,
    #[serde(rename = "SPEICHERHEIZUNG")]
    Speicherheizung,
    #[serde(rename = "UNTERBR_EINRICHTUNG")]
    UnterbrechbareEinrichtung,
    #[serde(rename = "WAERMEPUMPE")]
    Waermepumpe,
}
impl From<bo4e_core::enums::CustomerType> for Kundentyp {
    fn from(v: bo4e_core::enums::CustomerType) -> Self {
        match v {
            bo4e_core::enums::CustomerType::Commercial => Kundentyp::Gewerbe,
            bo4e_core::enums::CustomerType::Private => Kundentyp::Privat,
            bo4e_core::enums::CustomerType::Farmer => Kundentyp::Landwirt,
            bo4e_core::enums::CustomerType::Other => Kundentyp::Sonstige,
            bo4e_core::enums::CustomerType::Household => Kundentyp::Haushalt,
            bo4e_core::enums::CustomerType::DirectHeating => Kundentyp::Direktheizung,
            bo4e_core::enums::CustomerType::CommonFacilitiesMfh => {
                Kundentyp::GemeinschaftMFH
            }
            bo4e_core::enums::CustomerType::Church => Kundentyp::Kirche,
            bo4e_core::enums::CustomerType::Chp => Kundentyp::KWKAnlagen,
            bo4e_core::enums::CustomerType::ChargingStation => Kundentyp::Ladesaeule,
            bo4e_core::enums::CustomerType::PublicLighting => {
                Kundentyp::OeffentlicheBeleuchtung
            }
            bo4e_core::enums::CustomerType::StreetLighting => {
                Kundentyp::Strassenbeleuchtung
            }
            bo4e_core::enums::CustomerType::StorageHeating => Kundentyp::Speicherheizung,
            bo4e_core::enums::CustomerType::InterruptibleDevice => {
                Kundentyp::UnterbrechbareEinrichtung
            }
            bo4e_core::enums::CustomerType::HeatPump => Kundentyp::Waermepumpe,
            _ => panic!("Unknown {} variant", stringify!(CustomerType)),
        }
    }
}
impl From<Kundentyp> for bo4e_core::enums::CustomerType {
    fn from(v: Kundentyp) -> Self {
        match v {
            Kundentyp::Gewerbe => bo4e_core::enums::CustomerType::Commercial,
            Kundentyp::Privat => bo4e_core::enums::CustomerType::Private,
            Kundentyp::Landwirt => bo4e_core::enums::CustomerType::Farmer,
            Kundentyp::Sonstige => bo4e_core::enums::CustomerType::Other,
            Kundentyp::Haushalt => bo4e_core::enums::CustomerType::Household,
            Kundentyp::Direktheizung => bo4e_core::enums::CustomerType::DirectHeating,
            Kundentyp::GemeinschaftMFH => {
                bo4e_core::enums::CustomerType::CommonFacilitiesMfh
            }
            Kundentyp::Kirche => bo4e_core::enums::CustomerType::Church,
            Kundentyp::KWKAnlagen => bo4e_core::enums::CustomerType::Chp,
            Kundentyp::Ladesaeule => bo4e_core::enums::CustomerType::ChargingStation,
            Kundentyp::OeffentlicheBeleuchtung => {
                bo4e_core::enums::CustomerType::PublicLighting
            }
            Kundentyp::Strassenbeleuchtung => {
                bo4e_core::enums::CustomerType::StreetLighting
            }
            Kundentyp::Speicherheizung => bo4e_core::enums::CustomerType::StorageHeating,
            Kundentyp::UnterbrechbareEinrichtung => {
                bo4e_core::enums::CustomerType::InterruptibleDevice
            }
            Kundentyp::Waermepumpe => bo4e_core::enums::CustomerType::HeatPump,
            _ => panic!("Unknown {} variant", stringify!(Kundentyp)),
        }
    }
}
