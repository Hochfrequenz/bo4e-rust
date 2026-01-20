//! Usage type (Verwendungszweck) enumeration.

use serde::{Deserialize, Serialize};

/// Usage purpose for market location values.
///
/// Specifies the intended use of the values.
///
/// German: Verwendungszweck
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Verwendungszweck"))]
#[non_exhaustive]
pub enum UsageType {
    /// Network usage billing (Netznutzungsabrechnung)
    #[serde(rename = "NETZNUTZUNGSABRECHNUNG")]
    NetworkUsageBilling,

    /// Balancing group billing (Bilanzkreisabrechnung)
    #[serde(rename = "BILANZKREISABRECHNUNG")]
    BalancingGroupBilling,

    /// More/less quantity billing (Mehrmindermengenabrechnung)
    #[serde(rename = "MEHRMINDERMENGENABRECHNUNG")]
    MoreLessQuantityBilling,

    /// End customer billing (Endkundenabrechnung)
    #[serde(rename = "ENDKUNDENABRECHNUNG")]
    EndCustomerBilling,

    /// Transmission to origin registry (Übermittlung an das HKNR)
    #[serde(rename = "UEBERMITTLUNG_AN_DAS_HKNR")]
    TransmissionToOriginRegistry,

    /// Determination of balancing group balance (Ermittlung Ausgeglichenheit Bilanzkreis)
    #[serde(rename = "ERMITTLUNG_AUSGEGLICHENHEIT_BILANZKREIS")]
    BalancingGroupBalanceDetermination,
}

impl UsageType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::NetworkUsageBilling => "Netznutzungsabrechnung",
            Self::BalancingGroupBilling => "Bilanzkreisabrechnung",
            Self::MoreLessQuantityBilling => "Mehrmindermengenabrechnung",
            Self::EndCustomerBilling => "Endkundenabrechnung",
            Self::TransmissionToOriginRegistry => "Übermittlung an das HKNR",
            Self::BalancingGroupBalanceDetermination => "Ermittlung Ausgeglichenheit Bilanzkreis",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&UsageType::NetworkUsageBilling).unwrap(),
            r#""NETZNUTZUNGSABRECHNUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for usage in [
            UsageType::NetworkUsageBilling,
            UsageType::BalancingGroupBilling,
            UsageType::MoreLessQuantityBilling,
            UsageType::EndCustomerBilling,
            UsageType::TransmissionToOriginRegistry,
            UsageType::BalancingGroupBalanceDetermination,
        ] {
            let json = serde_json::to_string(&usage).unwrap();
            let parsed: UsageType = serde_json::from_str(&json).unwrap();
            assert_eq!(usage, parsed);
        }
    }
}
