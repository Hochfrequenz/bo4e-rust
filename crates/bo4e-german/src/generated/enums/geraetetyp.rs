#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Geraetetyp {
    #[serde(rename = "MULTIPLEXANLAGE")]
    MultiplexSystem,
    #[serde(rename = "PAUSCHALANLAGE")]
    FlatRateSystem,
    #[serde(rename = "VERSTAERKERANLAGE")]
    AmplifierSystem,
    #[serde(rename = "SUMMATIONSGERAET")]
    SummationDevice,
    #[serde(rename = "IMPULSGEBER")]
    PulseGenerator,
    #[serde(rename = "MENGENUMWERTER")]
    VolumeConverter,
    #[serde(rename = "STROMWANDLER")]
    CurrentTransformer,
    #[serde(rename = "SPANNUNGSWANDLER")]
    VoltageTransformer,
    #[serde(rename = "KOMBIMESSWANDLER")]
    CombinedMeasuringTransformer,
    #[serde(rename = "BLOCKSTROMWANDLER")]
    BlockCurrentTransformer,
    #[serde(rename = "DATENLOGGER")]
    DataLogger,
    #[serde(rename = "KOMMUNIKATIONSANSCHLUSS")]
    CommunicationConnection,
    #[serde(rename = "MODEM")]
    Modem,
    #[serde(rename = "TELEKOMMUNIKATIONSEINRICHTUNG")]
    TelecommunicationEquipment,
    #[serde(rename = "MODERNE_MESSEINRICHTUNG")]
    ModernMeasuringDevice,
    #[serde(rename = "INTELLIGENTES_MESSYSTEM")]
    IntelligentMeasuringSystem,
    #[serde(rename = "STEUEREINRICHTUNG")]
    ControlDevice,
    #[serde(rename = "TARIFSCHALTGERAET")]
    TariffSwitchingDevice,
    #[serde(rename = "RUNDSTEUEREMPFAENGER")]
    RippleControlReceiver,
    #[serde(rename = "OPTIONALE_ZUS_ZAEHLEINRICHTUNG")]
    OptionalAdditionalMeteringDevice,
    #[serde(rename = "MESSWANDLERSATZ_IMS_MME")]
    MeasuringTransformerSetImsMme,
    #[serde(rename = "KOMBIMESSWANDLER_IMS_MME")]
    CombinedTransformerSetImsMme,
    #[serde(rename = "TARIFSCHALTGERAET_IMS_MME")]
    TariffSwitchingDeviceImsMme,
    #[serde(rename = "RUNDSTEUEREMPFAENGER_IMS_MME")]
    RippleControlReceiverImsMme,
    #[serde(rename = "TEMPERATUR_KOMPENSATION")]
    TemperatureCompensation,
    #[serde(rename = "HOECHSTBELASTUNGS_ANZEIGER")]
    MaximumDemandIndicator,
    #[serde(rename = "SONSTIGES_GERAET")]
    OtherDevice,
    #[serde(rename = "EDL_21")]
    Edl21,
    #[serde(rename = "EDL_40_ZAEHLERAUFSATZ")]
    Edl40MeterAttachment,
    #[serde(rename = "EDL_40")]
    Edl40,
    #[serde(rename = "TELEFONANSCHLUSS")]
    TelephoneConnection,
    #[serde(rename = "MODEM_GSM")]
    ModemGsm,
    #[serde(rename = "MODEM_GPRS")]
    ModemGprs,
    #[serde(rename = "MODEM_FUNK")]
    ModemRadio,
    #[serde(rename = "MODEM_GSM_O_LG")]
    ProvidedByMSB,
    #[serde(rename = "MODEM_GSM_M_LG")]
    ProvidedByMSB,
    #[serde(rename = "MODEM_FESTNETZ")]
    ProvidedByMSB,
    #[serde(rename = "MODEM_GPRS_M_LG")]
    ProvidedByMSB,
    #[serde(rename = "PLC_KOM")]
    Powerline,
    #[serde(rename = "ETHERNET_KOM")]
    EthernetCommunication,
    #[serde(rename = "DSL_KOM")]
    DslCommunication,
    #[serde(rename = "LTE_KOM")]
    LteCommunication,
    #[serde(rename = "KOMPAKT_MU")]
    CompactVolumeConverter,
    #[serde(rename = "SYSTEM_MU")]
    SystemVolumeConverter,
    #[serde(rename = "TEMPERATUR_MU")]
    TemperatureVolumeConverter,
    #[serde(rename = "ZUSTANDS_MU")]
    StateVolumeConverter,
}
impl From<bo4e_core::enums::DeviceType> for Geraetetyp {
    fn from(v: bo4e_core::enums::DeviceType) -> Self {
        match v {
            bo4e_core::enums::DeviceType::MultiplexSystem => Geraetetyp::MultiplexSystem,
            bo4e_core::enums::DeviceType::FlatRateSystem => Geraetetyp::FlatRateSystem,
            bo4e_core::enums::DeviceType::AmplifierSystem => Geraetetyp::AmplifierSystem,
            bo4e_core::enums::DeviceType::SummationDevice => Geraetetyp::SummationDevice,
            bo4e_core::enums::DeviceType::PulseGenerator => Geraetetyp::PulseGenerator,
            bo4e_core::enums::DeviceType::VolumeConverter => Geraetetyp::VolumeConverter,
            bo4e_core::enums::DeviceType::CurrentTransformer => {
                Geraetetyp::CurrentTransformer
            }
            bo4e_core::enums::DeviceType::VoltageTransformer => {
                Geraetetyp::VoltageTransformer
            }
            bo4e_core::enums::DeviceType::CombinedMeasuringTransformer => {
                Geraetetyp::CombinedMeasuringTransformer
            }
            bo4e_core::enums::DeviceType::BlockCurrentTransformer => {
                Geraetetyp::BlockCurrentTransformer
            }
            bo4e_core::enums::DeviceType::DataLogger => Geraetetyp::DataLogger,
            bo4e_core::enums::DeviceType::CommunicationConnection => {
                Geraetetyp::CommunicationConnection
            }
            bo4e_core::enums::DeviceType::Modem => Geraetetyp::Modem,
            bo4e_core::enums::DeviceType::TelecommunicationEquipment => {
                Geraetetyp::TelecommunicationEquipment
            }
            bo4e_core::enums::DeviceType::ModernMeasuringDevice => {
                Geraetetyp::ModernMeasuringDevice
            }
            bo4e_core::enums::DeviceType::IntelligentMeasuringSystem => {
                Geraetetyp::IntelligentMeasuringSystem
            }
            bo4e_core::enums::DeviceType::ControlDevice => Geraetetyp::ControlDevice,
            bo4e_core::enums::DeviceType::TariffSwitchingDevice => {
                Geraetetyp::TariffSwitchingDevice
            }
            bo4e_core::enums::DeviceType::RippleControlReceiver => {
                Geraetetyp::RippleControlReceiver
            }
            bo4e_core::enums::DeviceType::OptionalAdditionalMeteringDevice => {
                Geraetetyp::OptionalAdditionalMeteringDevice
            }
            bo4e_core::enums::DeviceType::MeasuringTransformerSetImsMme => {
                Geraetetyp::MeasuringTransformerSetImsMme
            }
            bo4e_core::enums::DeviceType::CombinedTransformerSetImsMme => {
                Geraetetyp::CombinedTransformerSetImsMme
            }
            bo4e_core::enums::DeviceType::TariffSwitchingDeviceImsMme => {
                Geraetetyp::TariffSwitchingDeviceImsMme
            }
            bo4e_core::enums::DeviceType::RippleControlReceiverImsMme => {
                Geraetetyp::RippleControlReceiverImsMme
            }
            bo4e_core::enums::DeviceType::TemperatureCompensation => {
                Geraetetyp::TemperatureCompensation
            }
            bo4e_core::enums::DeviceType::MaximumDemandIndicator => {
                Geraetetyp::MaximumDemandIndicator
            }
            bo4e_core::enums::DeviceType::OtherDevice => Geraetetyp::OtherDevice,
            bo4e_core::enums::DeviceType::Edl21 => Geraetetyp::Edl21,
            bo4e_core::enums::DeviceType::Edl40MeterAttachment => {
                Geraetetyp::Edl40MeterAttachment
            }
            bo4e_core::enums::DeviceType::Edl40 => Geraetetyp::Edl40,
            bo4e_core::enums::DeviceType::TelephoneConnection => {
                Geraetetyp::TelephoneConnection
            }
            bo4e_core::enums::DeviceType::ModemGsm => Geraetetyp::ModemGsm,
            bo4e_core::enums::DeviceType::ModemGprs => Geraetetyp::ModemGprs,
            bo4e_core::enums::DeviceType::ModemRadio => Geraetetyp::ModemRadio,
            bo4e_core::enums::DeviceType::ModemGsmWithoutLoadProfile => {
                Geraetetyp::ProvidedByMSB
            }
            bo4e_core::enums::DeviceType::ModemGsmWithLoadProfile => {
                Geraetetyp::ProvidedByMSB
            }
            bo4e_core::enums::DeviceType::ModemLandline => Geraetetyp::ProvidedByMSB,
            bo4e_core::enums::DeviceType::ModemGprsWithLoadProfile => {
                Geraetetyp::ProvidedByMSB
            }
            bo4e_core::enums::DeviceType::PlcCommunication => Geraetetyp::Powerline,
            bo4e_core::enums::DeviceType::EthernetCommunication => {
                Geraetetyp::EthernetCommunication
            }
            bo4e_core::enums::DeviceType::DslCommunication => {
                Geraetetyp::DslCommunication
            }
            bo4e_core::enums::DeviceType::LteCommunication => {
                Geraetetyp::LteCommunication
            }
            bo4e_core::enums::DeviceType::CompactVolumeConverter => {
                Geraetetyp::CompactVolumeConverter
            }
            bo4e_core::enums::DeviceType::SystemVolumeConverter => {
                Geraetetyp::SystemVolumeConverter
            }
            bo4e_core::enums::DeviceType::TemperatureVolumeConverter => {
                Geraetetyp::TemperatureVolumeConverter
            }
            bo4e_core::enums::DeviceType::StateVolumeConverter => {
                Geraetetyp::StateVolumeConverter
            }
            _ => panic!("Unknown {} variant", stringify!(DeviceType)),
        }
    }
}
impl From<Geraetetyp> for bo4e_core::enums::DeviceType {
    fn from(v: Geraetetyp) -> Self {
        match v {
            Geraetetyp::MultiplexSystem => bo4e_core::enums::DeviceType::MultiplexSystem,
            Geraetetyp::FlatRateSystem => bo4e_core::enums::DeviceType::FlatRateSystem,
            Geraetetyp::AmplifierSystem => bo4e_core::enums::DeviceType::AmplifierSystem,
            Geraetetyp::SummationDevice => bo4e_core::enums::DeviceType::SummationDevice,
            Geraetetyp::PulseGenerator => bo4e_core::enums::DeviceType::PulseGenerator,
            Geraetetyp::VolumeConverter => bo4e_core::enums::DeviceType::VolumeConverter,
            Geraetetyp::CurrentTransformer => {
                bo4e_core::enums::DeviceType::CurrentTransformer
            }
            Geraetetyp::VoltageTransformer => {
                bo4e_core::enums::DeviceType::VoltageTransformer
            }
            Geraetetyp::CombinedMeasuringTransformer => {
                bo4e_core::enums::DeviceType::CombinedMeasuringTransformer
            }
            Geraetetyp::BlockCurrentTransformer => {
                bo4e_core::enums::DeviceType::BlockCurrentTransformer
            }
            Geraetetyp::DataLogger => bo4e_core::enums::DeviceType::DataLogger,
            Geraetetyp::CommunicationConnection => {
                bo4e_core::enums::DeviceType::CommunicationConnection
            }
            Geraetetyp::Modem => bo4e_core::enums::DeviceType::Modem,
            Geraetetyp::TelecommunicationEquipment => {
                bo4e_core::enums::DeviceType::TelecommunicationEquipment
            }
            Geraetetyp::ModernMeasuringDevice => {
                bo4e_core::enums::DeviceType::ModernMeasuringDevice
            }
            Geraetetyp::IntelligentMeasuringSystem => {
                bo4e_core::enums::DeviceType::IntelligentMeasuringSystem
            }
            Geraetetyp::ControlDevice => bo4e_core::enums::DeviceType::ControlDevice,
            Geraetetyp::TariffSwitchingDevice => {
                bo4e_core::enums::DeviceType::TariffSwitchingDevice
            }
            Geraetetyp::RippleControlReceiver => {
                bo4e_core::enums::DeviceType::RippleControlReceiver
            }
            Geraetetyp::OptionalAdditionalMeteringDevice => {
                bo4e_core::enums::DeviceType::OptionalAdditionalMeteringDevice
            }
            Geraetetyp::MeasuringTransformerSetImsMme => {
                bo4e_core::enums::DeviceType::MeasuringTransformerSetImsMme
            }
            Geraetetyp::CombinedTransformerSetImsMme => {
                bo4e_core::enums::DeviceType::CombinedTransformerSetImsMme
            }
            Geraetetyp::TariffSwitchingDeviceImsMme => {
                bo4e_core::enums::DeviceType::TariffSwitchingDeviceImsMme
            }
            Geraetetyp::RippleControlReceiverImsMme => {
                bo4e_core::enums::DeviceType::RippleControlReceiverImsMme
            }
            Geraetetyp::TemperatureCompensation => {
                bo4e_core::enums::DeviceType::TemperatureCompensation
            }
            Geraetetyp::MaximumDemandIndicator => {
                bo4e_core::enums::DeviceType::MaximumDemandIndicator
            }
            Geraetetyp::OtherDevice => bo4e_core::enums::DeviceType::OtherDevice,
            Geraetetyp::Edl21 => bo4e_core::enums::DeviceType::Edl21,
            Geraetetyp::Edl40MeterAttachment => {
                bo4e_core::enums::DeviceType::Edl40MeterAttachment
            }
            Geraetetyp::Edl40 => bo4e_core::enums::DeviceType::Edl40,
            Geraetetyp::TelephoneConnection => {
                bo4e_core::enums::DeviceType::TelephoneConnection
            }
            Geraetetyp::ModemGsm => bo4e_core::enums::DeviceType::ModemGsm,
            Geraetetyp::ModemGprs => bo4e_core::enums::DeviceType::ModemGprs,
            Geraetetyp::ModemRadio => bo4e_core::enums::DeviceType::ModemRadio,
            Geraetetyp::ProvidedByMSB => {
                bo4e_core::enums::DeviceType::ModemGsmWithoutLoadProfile
            }
            Geraetetyp::ProvidedByMSB => {
                bo4e_core::enums::DeviceType::ModemGsmWithLoadProfile
            }
            Geraetetyp::ProvidedByMSB => bo4e_core::enums::DeviceType::ModemLandline,
            Geraetetyp::ProvidedByMSB => {
                bo4e_core::enums::DeviceType::ModemGprsWithLoadProfile
            }
            Geraetetyp::Powerline => bo4e_core::enums::DeviceType::PlcCommunication,
            Geraetetyp::EthernetCommunication => {
                bo4e_core::enums::DeviceType::EthernetCommunication
            }
            Geraetetyp::DslCommunication => {
                bo4e_core::enums::DeviceType::DslCommunication
            }
            Geraetetyp::LteCommunication => {
                bo4e_core::enums::DeviceType::LteCommunication
            }
            Geraetetyp::CompactVolumeConverter => {
                bo4e_core::enums::DeviceType::CompactVolumeConverter
            }
            Geraetetyp::SystemVolumeConverter => {
                bo4e_core::enums::DeviceType::SystemVolumeConverter
            }
            Geraetetyp::TemperatureVolumeConverter => {
                bo4e_core::enums::DeviceType::TemperatureVolumeConverter
            }
            Geraetetyp::StateVolumeConverter => {
                bo4e_core::enums::DeviceType::StateVolumeConverter
            }
            _ => panic!("Unknown {} variant", stringify!(Geraetetyp)),
        }
    }
}
