//! Service type (Dienstleistungstyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of billable service in the energy sector.
///
/// German: Dienstleistungstyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ServiceType {
    // Data provision services
    /// Data provision daily (Datenbereitstellung taeglich)
    #[serde(rename = "DATENBEREITSTELLUNG_TAEGLICH")]
    DataProvisionDaily,

    /// Data provision weekly (Datenbereitstellung woechentlich)
    #[serde(rename = "DATENBEREITSTELLUNG_WOECHENTLICH")]
    DataProvisionWeekly,

    /// Data provision monthly (Datenbereitstellung monatlich)
    #[serde(rename = "DATENBEREITSTELLUNG_MONATLICH")]
    DataProvisionMonthly,

    /// Data provision yearly (Datenbereitstellung jaehrlich)
    #[serde(rename = "DATENBEREITSTELLUNG_JAEHRLICH")]
    DataProvisionYearly,

    /// Data provision historical load profiles
    #[serde(rename = "DATENBEREITSTELLUNG_HISTORISCHE_LG")]
    DataProvisionHistoricalLoadProfiles,

    /// Data provision hourly (Datenbereitstellung stuendlich)
    #[serde(rename = "DATENBEREITSTELLUNG_STUENDLICH")]
    DataProvisionHourly,

    /// Data provision quarterly (Datenbereitstellung vierteljaehrlich)
    #[serde(rename = "DATENBEREITSTELLUNG_VIERTELJAEHRLICH")]
    DataProvisionQuarterly,

    /// Data provision semi-annually (Datenbereitstellung halbjaehrlich)
    #[serde(rename = "DATENBEREITSTELLUNG_HALBJAEHRLICH")]
    DataProvisionSemiAnnually,

    /// Data provision monthly additional
    #[serde(rename = "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH")]
    DataProvisionMonthlyAdditional,

    /// Data provision one-time (Datenbereitstellung einmalig)
    #[serde(rename = "DATENBEREITSTELLUNG_EINMALIG")]
    DataProvisionOneTime,

    // Remote reading services
    /// Remote reading 2x daily
    #[serde(rename = "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG")]
    RemoteReading2xDaily,

    /// Remote reading daily
    #[serde(rename = "AUSLESUNG_TAEGLICH_FERNAUSLESUNG")]
    RemoteReadingDaily,

    /// Manual reading by metering point operator
    #[serde(rename = "AUSLESUNG_MANUELL_MSB")]
    ManualReadingMsb,

    /// Remote reading monthly
    #[serde(rename = "AUSLESUNG_MONATLICH_FERNAUSLESUNG")]
    RemoteReadingMonthly,

    /// Remote reading yearly for SLP
    #[serde(rename = "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG")]
    RemoteReadingYearly,

    /// Reading with mobile data capture (MDE)
    #[serde(rename = "AUSLESUNG_MDE")]
    ReadingMde,

    /// Remote reading general
    #[serde(rename = "AUSLESUNG_FERNAUSLESUNG")]
    RemoteReading,

    /// Remote reading additional by MSB
    #[serde(rename = "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB")]
    RemoteReadingAdditionalMsb,

    /// Remote reading monthly (alternate spelling)
    #[serde(rename = "AUSLESUNG_MOATLICH_FERNAUSLESUNG")]
    RemoteReadingMonthlyAlt,

    /// Remote reading hourly
    #[serde(rename = "AUSLESUNG_STUENDLICH_FERNAUSLESUNG")]
    RemoteReadingHourly,

    // Meter reading (manual)
    /// Manual reading monthly
    #[serde(rename = "ABLESUNG_MONATLICH")]
    ManualReadingMonthly,

    /// Manual reading quarterly
    #[serde(rename = "ABLESUNG_VIERTELJAEHRLICH")]
    ManualReadingQuarterly,

    /// Manual reading semi-annually
    #[serde(rename = "ABLESUNG_HALBJAEHRLICH")]
    ManualReadingSemiAnnually,

    /// Manual reading yearly
    #[serde(rename = "ABLESUNG_JAEHRLICH")]
    ManualReadingYearly,

    /// Additional reading by MSB
    #[serde(rename = "ABLESUNG_ZUSAETZLICH_MSB")]
    AdditionalReadingMsb,

    /// Additional reading by customer
    #[serde(rename = "ABLESUNG_ZUSAETZLICH_KUNDE")]
    AdditionalReadingCustomer,

    // Converter readings
    /// Temperature volume converter reading
    #[serde(rename = "AUSLESUNG_TEMPERATURMENGENUMWERTER")]
    TemperatureVolumeConverterReading,

    /// State volume converter reading
    #[serde(rename = "AUSLESUNG_ZUSTANDSMENGENUMWERTER")]
    StateVolumeConverterReading,

    /// System volume converter reading
    #[serde(rename = "AUSLESUNG_SYSTEMMENGENUMWERTER")]
    SystemVolumeConverterReading,

    /// Per transaction reading
    #[serde(rename = "AUSLESUNG_VORGANG")]
    PerTransactionReading,

    /// Compact volume converter reading
    #[serde(rename = "AUSLESUNG_KOMPAKTMENGENUMWERTER")]
    CompactVolumeConverterReading,

    // Other services
    /// Disconnection (Sperrung)
    #[serde(rename = "SPERRUNG")]
    Disconnection,

    /// Reconnection (Entsperrung)
    #[serde(rename = "ENTSPERRUNG")]
    Reconnection,

    /// Reminder fees (Mahnkosten)
    #[serde(rename = "MAHNKOSTEN")]
    ReminderFees,

    /// Collection costs (Inkassokosten)
    #[serde(rename = "INKASSOKOSTEN")]
    CollectionCosts,
}

impl ServiceType {
    /// Returns the German name.
    pub fn german_name(&self) -> &'static str {
        match self {
            Self::DataProvisionDaily => "Datenbereitstellung taeglich",
            Self::DataProvisionWeekly => "Datenbereitstellung woechentlich",
            Self::DataProvisionMonthly => "Datenbereitstellung monatlich",
            Self::DataProvisionYearly => "Datenbereitstellung jaehrlich",
            Self::DataProvisionHistoricalLoadProfiles => {
                "Datenbereitstellung historischer Lastgaenge"
            }
            Self::DataProvisionHourly => "Datenbereitstellung stuendlich",
            Self::DataProvisionQuarterly => "Datenbereitstellung vierteljaehrlich",
            Self::DataProvisionSemiAnnually => "Datenbereitstellung halbjaehrlich",
            Self::DataProvisionMonthlyAdditional => "Datenbereitstellung monatlich zusaetzlich",
            Self::DataProvisionOneTime => "Datenbereitstellung einmalig",
            Self::RemoteReading2xDaily => "Auslesung 2x taeglich mittels Fernauslesung",
            Self::RemoteReadingDaily => "Auslesung taeglich mittels Fernauslesung",
            Self::ManualReadingMsb => "Auslesung manuell vom MSB vorgenommen",
            Self::RemoteReadingMonthly => "Auslesung monatlich mittels Fernauslesung",
            Self::RemoteReadingYearly => "Auslesung jaehrlich bei SLP mittels Fernauslesung",
            Self::ReadingMde => "Auslesung mit mobiler Daten Erfassung",
            Self::RemoteReading => "Auslesung mittels Fernauslesung",
            Self::RemoteReadingAdditionalMsb => {
                "Auslesung mittels Fernauslesung zusaetzlich vom MSB"
            }
            Self::RemoteReadingMonthlyAlt => "Auslesung monatlich mittels Fernauslesung",
            Self::RemoteReadingHourly => "Auslesung stuendlich mittels Fernauslesung",
            Self::ManualReadingMonthly => "Ablesung monatlich",
            Self::ManualReadingQuarterly => "Ablesung vierteljaehrlich",
            Self::ManualReadingSemiAnnually => "Ablesung halbjaehrlich",
            Self::ManualReadingYearly => "Ablesung jaehrlich",
            Self::AdditionalReadingMsb => "Ablesung zusaetzlich vom MSB",
            Self::AdditionalReadingCustomer => "Ablesung zusaetzlich vom Kunden",
            Self::TemperatureVolumeConverterReading => "Auslesung Temperaturmengenumwerter",
            Self::StateVolumeConverterReading => "Auslesung Zustandsmengenumwerter",
            Self::SystemVolumeConverterReading => "Auslesung Systemmengenumwerter",
            Self::PerTransactionReading => "Auslesung je Vorgang",
            Self::CompactVolumeConverterReading => "Auslesung Kompaktmengenumwerter",
            Self::Disconnection => "Sperrung",
            Self::Reconnection => "Entsperrung",
            Self::ReminderFees => "Mahnkosten",
            Self::CollectionCosts => "Inkassokosten",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&ServiceType::DataProvisionDaily).unwrap(),
            r#""DATENBEREITSTELLUNG_TAEGLICH""#
        );
        assert_eq!(
            serde_json::to_string(&ServiceType::Disconnection).unwrap(),
            r#""SPERRUNG""#
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<ServiceType>(r#""DATENBEREITSTELLUNG_TAEGLICH""#).unwrap(),
            ServiceType::DataProvisionDaily
        );
        assert_eq!(
            serde_json::from_str::<ServiceType>(r#""INKASSOKOSTEN""#).unwrap(),
            ServiceType::CollectionCosts
        );
    }

    #[test]
    fn test_roundtrip() {
        for service_type in [
            ServiceType::DataProvisionDaily,
            ServiceType::DataProvisionMonthly,
            ServiceType::RemoteReadingDaily,
            ServiceType::ManualReadingMonthly,
            ServiceType::Disconnection,
            ServiceType::Reconnection,
            ServiceType::ReminderFees,
            ServiceType::CollectionCosts,
        ] {
            let json = serde_json::to_string(&service_type).unwrap();
            let parsed: ServiceType = serde_json::from_str(&json).unwrap();
            assert_eq!(service_type, parsed);
        }
    }
}
