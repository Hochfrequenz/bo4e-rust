#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geokoordinaten {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "latitude")]
    pub breitengrad: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "longitude")]
    pub laengengrad: Option<f64>,
}
impl From<bo4e_core::com::GeoCoordinates> for Geokoordinaten {
    fn from(v: bo4e_core::com::GeoCoordinates) -> Self {
        Self {
            meta: v.meta,
            breitengrad: v.latitude,
            laengengrad: v.longitude,
        }
    }
}
impl From<Geokoordinaten> for bo4e_core::com::GeoCoordinates {
    fn from(v: Geokoordinaten) -> Self {
        Self {
            meta: v.meta,
            latitude: v.breitengrad,
            longitude: v.laengengrad,
        }
    }
}
