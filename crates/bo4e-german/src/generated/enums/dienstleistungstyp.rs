#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Dienstleistungstyp {
    #[serde(rename = "DATENBEREITSTELLUNG_TAEGLICH")]
    DatenbereitstellungTaeglich,
    #[serde(rename = "DATENBEREITSTELLUNG_WOECHENTLICH")]
    DatenbereitstellungWoechentlich,
    #[serde(rename = "DATENBEREITSTELLUNG_MONATLICH")]
    DatenbereitstellungMonatlich,
    #[serde(rename = "DATENBEREITSTELLUNG_JAEHRLICH")]
    DatenbereitstellungJaehrlich,
    #[serde(rename = "DATENBEREITSTELLUNG_HISTORISCHE_LG")]
    DataProvisionHistoricalLoadProfiles,
    #[serde(rename = "DATENBEREITSTELLUNG_STUENDLICH")]
    DatenbereitstellungStuendlich,
    #[serde(rename = "DATENBEREITSTELLUNG_VIERTELJAEHRLICH")]
    DatenbereitstellungVierteljaehrlich,
    #[serde(rename = "DATENBEREITSTELLUNG_HALBJAEHRLICH")]
    DatenbereitstellungHalbjaehrlich,
    #[serde(rename = "DATENBEREITSTELLUNG_MONATLICH_ZUSAETZLICH")]
    DataProvisionMonthlyAdditional,
    #[serde(rename = "DATENBEREITSTELLUNG_EINMALIG")]
    DatenbereitstellungEinmalig,
    #[serde(rename = "AUSLESUNG_2X_TAEGLICH_FERNAUSLESUNG")]
    RemoteReading2xDaily,
    #[serde(rename = "AUSLESUNG_TAEGLICH_FERNAUSLESUNG")]
    RemoteReadingDaily,
    #[serde(rename = "AUSLESUNG_MANUELL_MSB")]
    ManualReadingMsb,
    #[serde(rename = "AUSLESUNG_MONATLICH_FERNAUSLESUNG")]
    RemoteReadingMonthly,
    #[serde(rename = "AUSLESUNG_JAEHRLICH_FERNAUSLESUNG")]
    RemoteReadingYearly,
    #[serde(rename = "AUSLESUNG_MDE")]
    MDE,
    #[serde(rename = "AUSLESUNG_FERNAUSLESUNG")]
    RemoteReading,
    #[serde(rename = "AUSLESUNG_FERNAUSLESUNG_ZUSAETZLICH_MSB")]
    RemoteReadingAdditionalMsb,
    #[serde(rename = "AUSLESUNG_MOATLICH_FERNAUSLESUNG")]
    AlternateSpelling,
    #[serde(rename = "AUSLESUNG_STUENDLICH_FERNAUSLESUNG")]
    RemoteReadingHourly,
    #[serde(rename = "ABLESUNG_MONATLICH")]
    ManualReadingMonthly,
    #[serde(rename = "ABLESUNG_VIERTELJAEHRLICH")]
    ManualReadingQuarterly,
    #[serde(rename = "ABLESUNG_HALBJAEHRLICH")]
    ManualReadingSemiAnnually,
    #[serde(rename = "ABLESUNG_JAEHRLICH")]
    ManualReadingYearly,
    #[serde(rename = "ABLESUNG_ZUSAETZLICH_MSB")]
    AdditionalReadingMsb,
    #[serde(rename = "ABLESUNG_ZUSAETZLICH_KUNDE")]
    AdditionalReadingCustomer,
    #[serde(rename = "AUSLESUNG_TEMPERATURMENGENUMWERTER")]
    TemperatureVolumeConverterReading,
    #[serde(rename = "AUSLESUNG_ZUSTANDSMENGENUMWERTER")]
    StateVolumeConverterReading,
    #[serde(rename = "AUSLESUNG_SYSTEMMENGENUMWERTER")]
    SystemVolumeConverterReading,
    #[serde(rename = "AUSLESUNG_VORGANG")]
    PerTransactionReading,
    #[serde(rename = "AUSLESUNG_KOMPAKTMENGENUMWERTER")]
    CompactVolumeConverterReading,
    #[serde(rename = "SPERRUNG")]
    Sperrung,
    #[serde(rename = "ENTSPERRUNG")]
    Entsperrung,
    #[serde(rename = "MAHNKOSTEN")]
    Mahnkosten,
    #[serde(rename = "INKASSOKOSTEN")]
    Inkassokosten,
}
impl From<bo4e_core::enums::ServiceType> for Dienstleistungstyp {
    fn from(v: bo4e_core::enums::ServiceType) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::ServiceType::DataProvisionDaily => {
                Dienstleistungstyp::DatenbereitstellungTaeglich
            }
            bo4e_core::enums::ServiceType::DataProvisionWeekly => {
                Dienstleistungstyp::DatenbereitstellungWoechentlich
            }
            bo4e_core::enums::ServiceType::DataProvisionMonthly => {
                Dienstleistungstyp::DatenbereitstellungMonatlich
            }
            bo4e_core::enums::ServiceType::DataProvisionYearly => {
                Dienstleistungstyp::DatenbereitstellungJaehrlich
            }
            bo4e_core::enums::ServiceType::DataProvisionHistoricalLoadProfiles => {
                Dienstleistungstyp::DataProvisionHistoricalLoadProfiles
            }
            bo4e_core::enums::ServiceType::DataProvisionHourly => {
                Dienstleistungstyp::DatenbereitstellungStuendlich
            }
            bo4e_core::enums::ServiceType::DataProvisionQuarterly => {
                Dienstleistungstyp::DatenbereitstellungVierteljaehrlich
            }
            bo4e_core::enums::ServiceType::DataProvisionSemiAnnually => {
                Dienstleistungstyp::DatenbereitstellungHalbjaehrlich
            }
            bo4e_core::enums::ServiceType::DataProvisionMonthlyAdditional => {
                Dienstleistungstyp::DataProvisionMonthlyAdditional
            }
            bo4e_core::enums::ServiceType::DataProvisionOneTime => {
                Dienstleistungstyp::DatenbereitstellungEinmalig
            }
            bo4e_core::enums::ServiceType::RemoteReading2xDaily => {
                Dienstleistungstyp::RemoteReading2xDaily
            }
            bo4e_core::enums::ServiceType::RemoteReadingDaily => {
                Dienstleistungstyp::RemoteReadingDaily
            }
            bo4e_core::enums::ServiceType::ManualReadingMsb => Dienstleistungstyp::ManualReadingMsb,
            bo4e_core::enums::ServiceType::RemoteReadingMonthly => {
                Dienstleistungstyp::RemoteReadingMonthly
            }
            bo4e_core::enums::ServiceType::RemoteReadingYearly => {
                Dienstleistungstyp::RemoteReadingYearly
            }
            bo4e_core::enums::ServiceType::ReadingMde => Dienstleistungstyp::MDE,
            bo4e_core::enums::ServiceType::RemoteReading => Dienstleistungstyp::RemoteReading,
            bo4e_core::enums::ServiceType::RemoteReadingAdditionalMsb => {
                Dienstleistungstyp::RemoteReadingAdditionalMsb
            }
            bo4e_core::enums::ServiceType::RemoteReadingMonthlyAlt => {
                Dienstleistungstyp::AlternateSpelling
            }
            bo4e_core::enums::ServiceType::RemoteReadingHourly => {
                Dienstleistungstyp::RemoteReadingHourly
            }
            bo4e_core::enums::ServiceType::ManualReadingMonthly => {
                Dienstleistungstyp::ManualReadingMonthly
            }
            bo4e_core::enums::ServiceType::ManualReadingQuarterly => {
                Dienstleistungstyp::ManualReadingQuarterly
            }
            bo4e_core::enums::ServiceType::ManualReadingSemiAnnually => {
                Dienstleistungstyp::ManualReadingSemiAnnually
            }
            bo4e_core::enums::ServiceType::ManualReadingYearly => {
                Dienstleistungstyp::ManualReadingYearly
            }
            bo4e_core::enums::ServiceType::AdditionalReadingMsb => {
                Dienstleistungstyp::AdditionalReadingMsb
            }
            bo4e_core::enums::ServiceType::AdditionalReadingCustomer => {
                Dienstleistungstyp::AdditionalReadingCustomer
            }
            bo4e_core::enums::ServiceType::TemperatureVolumeConverterReading => {
                Dienstleistungstyp::TemperatureVolumeConverterReading
            }
            bo4e_core::enums::ServiceType::StateVolumeConverterReading => {
                Dienstleistungstyp::StateVolumeConverterReading
            }
            bo4e_core::enums::ServiceType::SystemVolumeConverterReading => {
                Dienstleistungstyp::SystemVolumeConverterReading
            }
            bo4e_core::enums::ServiceType::PerTransactionReading => {
                Dienstleistungstyp::PerTransactionReading
            }
            bo4e_core::enums::ServiceType::CompactVolumeConverterReading => {
                Dienstleistungstyp::CompactVolumeConverterReading
            }
            bo4e_core::enums::ServiceType::Disconnection => Dienstleistungstyp::Sperrung,
            bo4e_core::enums::ServiceType::Reconnection => Dienstleistungstyp::Entsperrung,
            bo4e_core::enums::ServiceType::ReminderFees => Dienstleistungstyp::Mahnkosten,
            bo4e_core::enums::ServiceType::CollectionCosts => Dienstleistungstyp::Inkassokosten,
            _ => panic!("Unknown {} variant", stringify!(ServiceType)),
        }
    }
}
impl From<Dienstleistungstyp> for bo4e_core::enums::ServiceType {
    fn from(v: Dienstleistungstyp) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Dienstleistungstyp::DatenbereitstellungTaeglich => {
                bo4e_core::enums::ServiceType::DataProvisionDaily
            }
            Dienstleistungstyp::DatenbereitstellungWoechentlich => {
                bo4e_core::enums::ServiceType::DataProvisionWeekly
            }
            Dienstleistungstyp::DatenbereitstellungMonatlich => {
                bo4e_core::enums::ServiceType::DataProvisionMonthly
            }
            Dienstleistungstyp::DatenbereitstellungJaehrlich => {
                bo4e_core::enums::ServiceType::DataProvisionYearly
            }
            Dienstleistungstyp::DataProvisionHistoricalLoadProfiles => {
                bo4e_core::enums::ServiceType::DataProvisionHistoricalLoadProfiles
            }
            Dienstleistungstyp::DatenbereitstellungStuendlich => {
                bo4e_core::enums::ServiceType::DataProvisionHourly
            }
            Dienstleistungstyp::DatenbereitstellungVierteljaehrlich => {
                bo4e_core::enums::ServiceType::DataProvisionQuarterly
            }
            Dienstleistungstyp::DatenbereitstellungHalbjaehrlich => {
                bo4e_core::enums::ServiceType::DataProvisionSemiAnnually
            }
            Dienstleistungstyp::DataProvisionMonthlyAdditional => {
                bo4e_core::enums::ServiceType::DataProvisionMonthlyAdditional
            }
            Dienstleistungstyp::DatenbereitstellungEinmalig => {
                bo4e_core::enums::ServiceType::DataProvisionOneTime
            }
            Dienstleistungstyp::RemoteReading2xDaily => {
                bo4e_core::enums::ServiceType::RemoteReading2xDaily
            }
            Dienstleistungstyp::RemoteReadingDaily => {
                bo4e_core::enums::ServiceType::RemoteReadingDaily
            }
            Dienstleistungstyp::ManualReadingMsb => bo4e_core::enums::ServiceType::ManualReadingMsb,
            Dienstleistungstyp::RemoteReadingMonthly => {
                bo4e_core::enums::ServiceType::RemoteReadingMonthly
            }
            Dienstleistungstyp::RemoteReadingYearly => {
                bo4e_core::enums::ServiceType::RemoteReadingYearly
            }
            Dienstleistungstyp::MDE => bo4e_core::enums::ServiceType::ReadingMde,
            Dienstleistungstyp::RemoteReading => bo4e_core::enums::ServiceType::RemoteReading,
            Dienstleistungstyp::RemoteReadingAdditionalMsb => {
                bo4e_core::enums::ServiceType::RemoteReadingAdditionalMsb
            }
            Dienstleistungstyp::AlternateSpelling => {
                bo4e_core::enums::ServiceType::RemoteReadingMonthlyAlt
            }
            Dienstleistungstyp::RemoteReadingHourly => {
                bo4e_core::enums::ServiceType::RemoteReadingHourly
            }
            Dienstleistungstyp::ManualReadingMonthly => {
                bo4e_core::enums::ServiceType::ManualReadingMonthly
            }
            Dienstleistungstyp::ManualReadingQuarterly => {
                bo4e_core::enums::ServiceType::ManualReadingQuarterly
            }
            Dienstleistungstyp::ManualReadingSemiAnnually => {
                bo4e_core::enums::ServiceType::ManualReadingSemiAnnually
            }
            Dienstleistungstyp::ManualReadingYearly => {
                bo4e_core::enums::ServiceType::ManualReadingYearly
            }
            Dienstleistungstyp::AdditionalReadingMsb => {
                bo4e_core::enums::ServiceType::AdditionalReadingMsb
            }
            Dienstleistungstyp::AdditionalReadingCustomer => {
                bo4e_core::enums::ServiceType::AdditionalReadingCustomer
            }
            Dienstleistungstyp::TemperatureVolumeConverterReading => {
                bo4e_core::enums::ServiceType::TemperatureVolumeConverterReading
            }
            Dienstleistungstyp::StateVolumeConverterReading => {
                bo4e_core::enums::ServiceType::StateVolumeConverterReading
            }
            Dienstleistungstyp::SystemVolumeConverterReading => {
                bo4e_core::enums::ServiceType::SystemVolumeConverterReading
            }
            Dienstleistungstyp::PerTransactionReading => {
                bo4e_core::enums::ServiceType::PerTransactionReading
            }
            Dienstleistungstyp::CompactVolumeConverterReading => {
                bo4e_core::enums::ServiceType::CompactVolumeConverterReading
            }
            Dienstleistungstyp::Sperrung => bo4e_core::enums::ServiceType::Disconnection,
            Dienstleistungstyp::Entsperrung => bo4e_core::enums::ServiceType::Reconnection,
            Dienstleistungstyp::Mahnkosten => bo4e_core::enums::ServiceType::ReminderFees,
            Dienstleistungstyp::Inkassokosten => bo4e_core::enums::ServiceType::CollectionCosts,
        }
    }
}
