//! Invoice type (Rechnungstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of invoice in the energy sector.
///
/// German: Rechnungstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json-schema", schemars(rename = "Rechnungstyp"))]
#[non_exhaustive]
pub enum InvoiceType {
    /// End customer invoice (Endkundenrechnung)
    #[serde(rename = "ENDKUNDENRECHNUNG")]
    EndCustomerInvoice,

    /// Network usage invoice (Netznutzungsrechnung)
    #[serde(rename = "NETZNUTZUNGSRECHNUNG")]
    NetworkUsageInvoice,

    /// Surplus/deficit quantity invoice (Mehrmindermengenrechnung)
    #[serde(rename = "MEHRMINDERMENGENRECHNUNG")]
    SurplusDeficitInvoice,

    /// Metering point operation invoice (Messstellenbetriebsrechnung)
    #[serde(rename = "MESSSTELLENBETRIEBSRECHNUNG")]
    MeteringPointOperationInvoice,

    /// Procurement invoice (Beschaffungsrechnung)
    #[serde(rename = "BESCHAFFUNGSRECHNUNG")]
    ProcurementInvoice,

    /// Balancing energy invoice (Ausgleichsenergierechnung)
    #[serde(rename = "AUSGLEICHSENERGIERECHNUNG")]
    BalancingEnergyInvoice,

    /// Final invoice (Abschlussrechnung)
    #[serde(rename = "ABSCHLUSSRECHNUNG")]
    FinalInvoice,

    /// Instalment invoice (Abschlagsrechnung)
    #[serde(rename = "ABSCHLAGSRECHNUNG")]
    InstalmentInvoice,

    /// Regular/periodic invoice (Turnusrechnung)
    #[serde(rename = "TURNUSRECHNUNG")]
    PeriodicInvoice,

    /// Monthly invoice (Monatsrechnung)
    #[serde(rename = "MONATSRECHNUNG")]
    MonthlyInvoice,

    /// Interim invoice (Zwischenrechnung)
    #[serde(rename = "ZWISCHENRECHNUNG")]
    InterimInvoice,

    /// Integrated 13th invoice (Integrierte 13te Rechnung)
    #[serde(rename = "INTEGRIERTE_13TE_RECHNUNG")]
    Integrated13thInvoice,

    /// Additional 13th invoice (Zusaetzliche 13te Rechnung)
    #[serde(rename = "ZUSAETZLICHE_13TE_RECHNUNG")]
    Additional13thInvoice,
}

impl InvoiceType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::EndCustomerInvoice => "Endkundenrechnung",
            Self::NetworkUsageInvoice => "Netznutzungsrechnung",
            Self::SurplusDeficitInvoice => "Mehrmindermengenrechnung",
            Self::MeteringPointOperationInvoice => "Messstellenbetriebsrechnung",
            Self::ProcurementInvoice => "Beschaffungsrechnung",
            Self::BalancingEnergyInvoice => "Ausgleichsenergierechnung",
            Self::FinalInvoice => "Abschlussrechnung",
            Self::InstalmentInvoice => "Abschlagsrechnung",
            Self::PeriodicInvoice => "Turnusrechnung",
            Self::MonthlyInvoice => "Monatsrechnung",
            Self::InterimInvoice => "Zwischenrechnung",
            Self::Integrated13thInvoice => "Integrierte 13te Rechnung",
            Self::Additional13thInvoice => "Zusaetzliche 13te Rechnung",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&InvoiceType::EndCustomerInvoice).unwrap(),
            r#""ENDKUNDENRECHNUNG""#
        );
        assert_eq!(
            serde_json::to_string(&InvoiceType::MonthlyInvoice).unwrap(),
            r#""MONATSRECHNUNG""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<InvoiceType>(r#""ENDKUNDENRECHNUNG""#).unwrap(),
            InvoiceType::EndCustomerInvoice
        );
        assert_eq!(
            serde_json::from_str::<InvoiceType>(r#""ABSCHLAGSRECHNUNG""#).unwrap(),
            InvoiceType::InstalmentInvoice
        );
    }

    #[test]
    fn test_roundtrip() {
        for invoice_type in [
            InvoiceType::EndCustomerInvoice,
            InvoiceType::NetworkUsageInvoice,
            InvoiceType::SurplusDeficitInvoice,
            InvoiceType::MeteringPointOperationInvoice,
            InvoiceType::ProcurementInvoice,
            InvoiceType::BalancingEnergyInvoice,
            InvoiceType::FinalInvoice,
            InvoiceType::InstalmentInvoice,
            InvoiceType::PeriodicInvoice,
            InvoiceType::MonthlyInvoice,
            InvoiceType::InterimInvoice,
            InvoiceType::Integrated13thInvoice,
            InvoiceType::Additional13thInvoice,
        ] {
            let json = serde_json::to_string(&invoice_type).unwrap();
            let parsed: InvoiceType = serde_json::from_str(&json).unwrap();
            assert_eq!(invoice_type, parsed);
        }
    }
}
