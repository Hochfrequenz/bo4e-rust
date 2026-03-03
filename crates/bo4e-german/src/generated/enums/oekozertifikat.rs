#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Oekozertifikat {
    #[serde(rename = "CMS_EE01")]
    CmsEe01,
    #[serde(rename = "CMS_EE02")]
    CmsEe02,
    #[serde(rename = "EECS")]
    EuropeanEnergyCertificateSystem,
    #[serde(rename = "FRAUNHOFER")]
    Fraunhofer,
    #[serde(rename = "BET")]
    Bet,
    #[serde(rename = "KLIMA_INVEST")]
    KlimaInvest,
    #[serde(rename = "LGA")]
    Lga,
    #[serde(rename = "FREIBERG")]
    Freiberg,
    #[serde(rename = "RECS")]
    RenewableEnergyCertificateSystem,
    #[serde(rename = "REGS_EGL")]
    RegsEgl,
    #[serde(rename = "TUEV")]
    Tuev,
    #[serde(rename = "TUEV_HESSEN")]
    TuevHessen,
    #[serde(rename = "TUEV_NORD")]
    TuevNord,
    #[serde(rename = "TUEV_RHEINLAND")]
    TuevRheinland,
    #[serde(rename = "TUEV_SUED")]
    TuevSued,
    #[serde(rename = "TUEV_SUED_EE01")]
    TuevSuedEe01,
    #[serde(rename = "TUEV_SUED_EE02")]
    TuevSuedEe02,
}
impl From<bo4e_core::enums::EcoCertificate> for Oekozertifikat {
    fn from(v: bo4e_core::enums::EcoCertificate) -> Self {
        match v {
            bo4e_core::enums::EcoCertificate::CmsEe01 => Oekozertifikat::CmsEe01,
            bo4e_core::enums::EcoCertificate::CmsEe02 => Oekozertifikat::CmsEe02,
            bo4e_core::enums::EcoCertificate::Eecs => {
                Oekozertifikat::EuropeanEnergyCertificateSystem
            }
            bo4e_core::enums::EcoCertificate::Fraunhofer => Oekozertifikat::Fraunhofer,
            bo4e_core::enums::EcoCertificate::Bet => Oekozertifikat::Bet,
            bo4e_core::enums::EcoCertificate::KlimaInvest => Oekozertifikat::KlimaInvest,
            bo4e_core::enums::EcoCertificate::Lga => Oekozertifikat::Lga,
            bo4e_core::enums::EcoCertificate::Freiberg => Oekozertifikat::Freiberg,
            bo4e_core::enums::EcoCertificate::Recs => {
                Oekozertifikat::RenewableEnergyCertificateSystem
            }
            bo4e_core::enums::EcoCertificate::RegsEgl => Oekozertifikat::RegsEgl,
            bo4e_core::enums::EcoCertificate::Tuev => Oekozertifikat::Tuev,
            bo4e_core::enums::EcoCertificate::TuevHessen => Oekozertifikat::TuevHessen,
            bo4e_core::enums::EcoCertificate::TuevNord => Oekozertifikat::TuevNord,
            bo4e_core::enums::EcoCertificate::TuevRheinland => {
                Oekozertifikat::TuevRheinland
            }
            bo4e_core::enums::EcoCertificate::TuevSued => Oekozertifikat::TuevSued,
            bo4e_core::enums::EcoCertificate::TuevSuedEe01 => {
                Oekozertifikat::TuevSuedEe01
            }
            bo4e_core::enums::EcoCertificate::TuevSuedEe02 => {
                Oekozertifikat::TuevSuedEe02
            }
            _ => panic!("Unknown {} variant", stringify!(EcoCertificate)),
        }
    }
}
impl From<Oekozertifikat> for bo4e_core::enums::EcoCertificate {
    fn from(v: Oekozertifikat) -> Self {
        match v {
            Oekozertifikat::CmsEe01 => bo4e_core::enums::EcoCertificate::CmsEe01,
            Oekozertifikat::CmsEe02 => bo4e_core::enums::EcoCertificate::CmsEe02,
            Oekozertifikat::EuropeanEnergyCertificateSystem => {
                bo4e_core::enums::EcoCertificate::Eecs
            }
            Oekozertifikat::Fraunhofer => bo4e_core::enums::EcoCertificate::Fraunhofer,
            Oekozertifikat::Bet => bo4e_core::enums::EcoCertificate::Bet,
            Oekozertifikat::KlimaInvest => bo4e_core::enums::EcoCertificate::KlimaInvest,
            Oekozertifikat::Lga => bo4e_core::enums::EcoCertificate::Lga,
            Oekozertifikat::Freiberg => bo4e_core::enums::EcoCertificate::Freiberg,
            Oekozertifikat::RenewableEnergyCertificateSystem => {
                bo4e_core::enums::EcoCertificate::Recs
            }
            Oekozertifikat::RegsEgl => bo4e_core::enums::EcoCertificate::RegsEgl,
            Oekozertifikat::Tuev => bo4e_core::enums::EcoCertificate::Tuev,
            Oekozertifikat::TuevHessen => bo4e_core::enums::EcoCertificate::TuevHessen,
            Oekozertifikat::TuevNord => bo4e_core::enums::EcoCertificate::TuevNord,
            Oekozertifikat::TuevRheinland => {
                bo4e_core::enums::EcoCertificate::TuevRheinland
            }
            Oekozertifikat::TuevSued => bo4e_core::enums::EcoCertificate::TuevSued,
            Oekozertifikat::TuevSuedEe01 => {
                bo4e_core::enums::EcoCertificate::TuevSuedEe01
            }
            Oekozertifikat::TuevSuedEe02 => {
                bo4e_core::enums::EcoCertificate::TuevSuedEe02
            }
            _ => panic!("Unknown {} variant", stringify!(Oekozertifikat)),
        }
    }
}
