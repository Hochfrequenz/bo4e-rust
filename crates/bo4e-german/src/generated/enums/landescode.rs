#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Landescode {
    #[serde(rename = "DE")]
    Deutschland,
    #[serde(rename = "AT")]
    Oesterreich,
    #[serde(rename = "CH")]
    Schweiz,
    #[serde(rename = "NL")]
    Niederlande,
    #[serde(rename = "BE")]
    Belgien,
    #[serde(rename = "FR")]
    Frankreich,
    #[serde(rename = "LU")]
    Luxemburg,
    #[serde(rename = "PL")]
    Polen,
    #[serde(rename = "CZ")]
    Tschechien,
    #[serde(rename = "DK")]
    Daenemark,
    #[serde(rename = "IT")]
    Italien,
    #[serde(rename = "ES")]
    Spanien,
    #[serde(rename = "GB")]
    VereinigtesKoenigreich,
    #[serde(rename = "SE")]
    Schweden,
    #[serde(rename = "NO")]
    Norwegen,
    #[serde(rename = "FI")]
    Finnland,
    #[serde(rename = "PT")]
    Portugal,
    #[serde(rename = "GR")]
    Griechenland,
    #[serde(rename = "IE")]
    Irland,
    #[serde(rename = "HU")]
    Ungarn,
    #[serde(rename = "SK")]
    Slowakei,
    #[serde(rename = "SI")]
    Slowenien,
    #[serde(rename = "HR")]
    Kroatien,
    #[serde(rename = "RO")]
    Rumaenien,
    #[serde(rename = "BG")]
    Bulgarien,
    #[serde(rename = "EE")]
    Estland,
    #[serde(rename = "LV")]
    Lettland,
    #[serde(rename = "LT")]
    Litauen,
    #[serde(rename = "CY")]
    Zypern,
    #[serde(rename = "MT")]
    Malta,
    #[serde(rename = "LI")]
    Liechtenstein,
    #[serde(rename = "IS")]
    Island,
}
impl From<bo4e_core::enums::Country> for Landescode {
    fn from(v: bo4e_core::enums::Country) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::Country::Germany => Landescode::Deutschland,
            bo4e_core::enums::Country::Austria => Landescode::Oesterreich,
            bo4e_core::enums::Country::Switzerland => Landescode::Schweiz,
            bo4e_core::enums::Country::Netherlands => Landescode::Niederlande,
            bo4e_core::enums::Country::Belgium => Landescode::Belgien,
            bo4e_core::enums::Country::France => Landescode::Frankreich,
            bo4e_core::enums::Country::Luxembourg => Landescode::Luxemburg,
            bo4e_core::enums::Country::Poland => Landescode::Polen,
            bo4e_core::enums::Country::CzechRepublic => Landescode::Tschechien,
            bo4e_core::enums::Country::Denmark => Landescode::Daenemark,
            bo4e_core::enums::Country::Italy => Landescode::Italien,
            bo4e_core::enums::Country::Spain => Landescode::Spanien,
            bo4e_core::enums::Country::UnitedKingdom => Landescode::VereinigtesKoenigreich,
            bo4e_core::enums::Country::Sweden => Landescode::Schweden,
            bo4e_core::enums::Country::Norway => Landescode::Norwegen,
            bo4e_core::enums::Country::Finland => Landescode::Finnland,
            bo4e_core::enums::Country::Portugal => Landescode::Portugal,
            bo4e_core::enums::Country::Greece => Landescode::Griechenland,
            bo4e_core::enums::Country::Ireland => Landescode::Irland,
            bo4e_core::enums::Country::Hungary => Landescode::Ungarn,
            bo4e_core::enums::Country::Slovakia => Landescode::Slowakei,
            bo4e_core::enums::Country::Slovenia => Landescode::Slowenien,
            bo4e_core::enums::Country::Croatia => Landescode::Kroatien,
            bo4e_core::enums::Country::Romania => Landescode::Rumaenien,
            bo4e_core::enums::Country::Bulgaria => Landescode::Bulgarien,
            bo4e_core::enums::Country::Estonia => Landescode::Estland,
            bo4e_core::enums::Country::Latvia => Landescode::Lettland,
            bo4e_core::enums::Country::Lithuania => Landescode::Litauen,
            bo4e_core::enums::Country::Cyprus => Landescode::Zypern,
            bo4e_core::enums::Country::Malta => Landescode::Malta,
            bo4e_core::enums::Country::Liechtenstein => Landescode::Liechtenstein,
            bo4e_core::enums::Country::Iceland => Landescode::Island,
            _ => panic!("Unknown {} variant", stringify!(Country)),
        }
    }
}
impl From<Landescode> for bo4e_core::enums::Country {
    fn from(v: Landescode) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Landescode::Deutschland => bo4e_core::enums::Country::Germany,
            Landescode::Oesterreich => bo4e_core::enums::Country::Austria,
            Landescode::Schweiz => bo4e_core::enums::Country::Switzerland,
            Landescode::Niederlande => bo4e_core::enums::Country::Netherlands,
            Landescode::Belgien => bo4e_core::enums::Country::Belgium,
            Landescode::Frankreich => bo4e_core::enums::Country::France,
            Landescode::Luxemburg => bo4e_core::enums::Country::Luxembourg,
            Landescode::Polen => bo4e_core::enums::Country::Poland,
            Landescode::Tschechien => bo4e_core::enums::Country::CzechRepublic,
            Landescode::Daenemark => bo4e_core::enums::Country::Denmark,
            Landescode::Italien => bo4e_core::enums::Country::Italy,
            Landescode::Spanien => bo4e_core::enums::Country::Spain,
            Landescode::VereinigtesKoenigreich => bo4e_core::enums::Country::UnitedKingdom,
            Landescode::Schweden => bo4e_core::enums::Country::Sweden,
            Landescode::Norwegen => bo4e_core::enums::Country::Norway,
            Landescode::Finnland => bo4e_core::enums::Country::Finland,
            Landescode::Portugal => bo4e_core::enums::Country::Portugal,
            Landescode::Griechenland => bo4e_core::enums::Country::Greece,
            Landescode::Irland => bo4e_core::enums::Country::Ireland,
            Landescode::Ungarn => bo4e_core::enums::Country::Hungary,
            Landescode::Slowakei => bo4e_core::enums::Country::Slovakia,
            Landescode::Slowenien => bo4e_core::enums::Country::Slovenia,
            Landescode::Kroatien => bo4e_core::enums::Country::Croatia,
            Landescode::Rumaenien => bo4e_core::enums::Country::Romania,
            Landescode::Bulgarien => bo4e_core::enums::Country::Bulgaria,
            Landescode::Estland => bo4e_core::enums::Country::Estonia,
            Landescode::Lettland => bo4e_core::enums::Country::Latvia,
            Landescode::Litauen => bo4e_core::enums::Country::Lithuania,
            Landescode::Zypern => bo4e_core::enums::Country::Cyprus,
            Landescode::Malta => bo4e_core::enums::Country::Malta,
            Landescode::Liechtenstein => bo4e_core::enums::Country::Liechtenstein,
            Landescode::Island => bo4e_core::enums::Country::Iceland,
        }
    }
}
