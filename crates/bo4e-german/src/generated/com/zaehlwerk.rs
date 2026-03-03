#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zaehlwerk {
    #[serde(flatten)]
    pub meta: bo4e_core::Bo4eMeta,
    #[serde(skip_serializing_if = "Option::is_none", alias = "registerId")]
    pub zaehlwerkskennung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "obisCode")]
    pub obis_kennzahl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "registerType")]
    pub registerart: Option<crate::Registertyp>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "energyDirection")]
    pub energierichtung: Option<crate::Energierichtung>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "unit")]
    pub einheit: Option<crate::Mengeneinheit>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "decimalPlaces")]
    pub nachkommastellen: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "transformerRatio")]
    pub wandlerfaktor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "description")]
    pub bezeichnung: Option<String>,
}
impl From<bo4e_core::com::MeterRegister> for Zaehlwerk {
    fn from(v: bo4e_core::com::MeterRegister) -> Self {
        Self {
            meta: v.meta,
            zaehlwerkskennung: v.register_id,
            obis_kennzahl: v.obis_code,
            registerart: v.register_type.map(Into::into),
            energierichtung: v.energy_direction.map(Into::into),
            einheit: v.unit.map(Into::into),
            nachkommastellen: v.decimal_places,
            wandlerfaktor: v.transformer_ratio,
            bezeichnung: v.description,
        }
    }
}
impl From<Zaehlwerk> for bo4e_core::com::MeterRegister {
    fn from(v: Zaehlwerk) -> Self {
        Self {
            meta: v.meta,
            register_id: v.zaehlwerkskennung,
            obis_code: v.obis_kennzahl,
            register_type: v.registerart.map(Into::into),
            energy_direction: v.energierichtung.map(Into::into),
            unit: v.einheit.map(Into::into),
            decimal_places: v.nachkommastellen,
            transformer_ratio: v.wandlerfaktor,
            description: v.bezeichnung,
        }
    }
}
