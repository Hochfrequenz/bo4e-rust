//! Eco certificate (Oekozertifikat) enumeration.

use serde::{Deserialize, Serialize};

/// Eco certificate for green electricity.
///
/// Certificates for eco-electricity from various issuers.
///
/// German: Oekozertifikat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Oekozertifikat"))]
#[non_exhaustive]
pub enum EcoCertificate {
    /// CMS EE01 certificate
    #[serde(rename = "CMS_EE01")]
    CmsEe01,

    /// CMS EE02 certificate
    #[serde(rename = "CMS_EE02")]
    CmsEe02,

    /// EECS (European Energy Certificate System)
    #[serde(rename = "EECS")]
    Eecs,

    /// Fraunhofer certificate
    #[serde(rename = "FRAUNHOFER")]
    Fraunhofer,

    /// BET certificate
    #[serde(rename = "BET")]
    Bet,

    /// KlimaINVEST certificate
    #[serde(rename = "KLIMA_INVEST")]
    KlimaInvest,

    /// LGA certificate
    #[serde(rename = "LGA")]
    Lga,

    /// Freiberg certificate
    #[serde(rename = "FREIBERG")]
    Freiberg,

    /// RECS (Renewable Energy Certificate System)
    #[serde(rename = "RECS")]
    Recs,

    /// REGS EGL certificate
    #[serde(rename = "REGS_EGL")]
    RegsEgl,

    /// TÜV certificate
    #[serde(rename = "TUEV")]
    Tuev,

    /// TÜV Hessen certificate
    #[serde(rename = "TUEV_HESSEN")]
    TuevHessen,

    /// TÜV Nord certificate
    #[serde(rename = "TUEV_NORD")]
    TuevNord,

    /// TÜV Rheinland certificate
    #[serde(rename = "TUEV_RHEINLAND")]
    TuevRheinland,

    /// TÜV Süd certificate
    #[serde(rename = "TUEV_SUED")]
    TuevSued,

    /// TÜV Süd EE01 certificate
    #[serde(rename = "TUEV_SUED_EE01")]
    TuevSuedEe01,

    /// TÜV Süd EE02 certificate
    #[serde(rename = "TUEV_SUED_EE02")]
    TuevSuedEe02,
}

impl EcoCertificate {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::CmsEe01 => "CMS EE01",
            Self::CmsEe02 => "CMS EE02",
            Self::Eecs => "EECS",
            Self::Fraunhofer => "Fraunhofer",
            Self::Bet => "BET",
            Self::KlimaInvest => "KlimaINVEST",
            Self::Lga => "LGA",
            Self::Freiberg => "Freiberg",
            Self::Recs => "RECS",
            Self::RegsEgl => "REGS EGL",
            Self::Tuev => "TÜV",
            Self::TuevHessen => "TÜV Hessen",
            Self::TuevNord => "TÜV Nord",
            Self::TuevRheinland => "TÜV Rheinland",
            Self::TuevSued => "TÜV Süd",
            Self::TuevSuedEe01 => "TÜV Süd EE01",
            Self::TuevSuedEe02 => "TÜV Süd EE02",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&EcoCertificate::Eecs).unwrap(),
            r#""EECS""#
        );
        assert_eq!(
            serde_json::to_string(&EcoCertificate::TuevRheinland).unwrap(),
            r#""TUEV_RHEINLAND""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for cert in [
            EcoCertificate::CmsEe01,
            EcoCertificate::CmsEe02,
            EcoCertificate::Eecs,
            EcoCertificate::Fraunhofer,
            EcoCertificate::Bet,
            EcoCertificate::KlimaInvest,
            EcoCertificate::Lga,
            EcoCertificate::Freiberg,
            EcoCertificate::Recs,
            EcoCertificate::RegsEgl,
            EcoCertificate::Tuev,
            EcoCertificate::TuevHessen,
            EcoCertificate::TuevNord,
            EcoCertificate::TuevRheinland,
            EcoCertificate::TuevSued,
            EcoCertificate::TuevSuedEe01,
            EcoCertificate::TuevSuedEe02,
        ] {
            let json = serde_json::to_string(&cert).unwrap();
            let parsed: EcoCertificate = serde_json::from_str(&json).unwrap();
            assert_eq!(cert, parsed);
        }
    }
}
