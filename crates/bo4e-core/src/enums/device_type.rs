//! Device type (Geraetetyp) enumeration.

use serde::{Deserialize, Serialize};

/// Type of billable device.
///
/// Lists possible billable device types.
///
/// German: Geraetetyp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeviceType {
    /// Multiplexer system
    #[serde(rename = "MULTIPLEXANLAGE")]
    MultiplexSystem,

    /// Flat-rate system
    #[serde(rename = "PAUSCHALANLAGE")]
    FlatRateSystem,

    /// Amplifier system
    #[serde(rename = "VERSTAERKERANLAGE")]
    AmplifierSystem,

    /// Summation device
    #[serde(rename = "SUMMATIONSGERAET")]
    SummationDevice,

    /// Pulse generator
    #[serde(rename = "IMPULSGEBER")]
    PulseGenerator,

    /// Volume converter
    #[serde(rename = "MENGENUMWERTER")]
    VolumeConverter,

    /// Current transformer
    #[serde(rename = "STROMWANDLER")]
    CurrentTransformer,

    /// Voltage transformer
    #[serde(rename = "SPANNUNGSWANDLER")]
    VoltageTransformer,

    /// Combined measuring transformer
    #[serde(rename = "KOMBIMESSWANDLER")]
    CombinedMeasuringTransformer,

    /// Block current transformer
    #[serde(rename = "BLOCKSTROMWANDLER")]
    BlockCurrentTransformer,

    /// Data logger
    #[serde(rename = "DATENLOGGER")]
    DataLogger,

    /// Communication connection
    #[serde(rename = "KOMMUNIKATIONSANSCHLUSS")]
    CommunicationConnection,

    /// Modem
    #[serde(rename = "MODEM")]
    Modem,

    /// Telecommunication equipment provided by MSB
    #[serde(rename = "TELEKOMMUNIKATIONSEINRICHTUNG")]
    TelecommunicationEquipment,

    /// Modern measuring device
    #[serde(rename = "MODERNE_MESSEINRICHTUNG")]
    ModernMeasuringDevice,

    /// Intelligent measuring system
    #[serde(rename = "INTELLIGENTES_MESSYSTEM")]
    IntelligentMeasuringSystem,

    /// Control device
    #[serde(rename = "STEUEREINRICHTUNG")]
    ControlDevice,

    /// Tariff switching device
    #[serde(rename = "TARIFSCHALTGERAET")]
    TariffSwitchingDevice,

    /// Ripple control receiver
    #[serde(rename = "RUNDSTEUEREMPFAENGER")]
    RippleControlReceiver,

    /// Optional additional metering device
    #[serde(rename = "OPTIONALE_ZUS_ZAEHLEINRICHTUNG")]
    OptionalAdditionalMeteringDevice,

    /// Measuring transformer set for iMS and mME, NSP
    #[serde(rename = "MESSWANDLERSATZ_IMS_MME")]
    MeasuringTransformerSetImsMme,

    /// Combined measuring transformer set for iMS and mME
    #[serde(rename = "KOMBIMESSWANDLER_IMS_MME")]
    CombinedTransformerSetImsMme,

    /// Tariff switching for iMS and mME
    #[serde(rename = "TARIFSCHALTGERAET_IMS_MME")]
    TariffSwitchingDeviceImsMme,

    /// Ripple control receiver for iMS and mME
    #[serde(rename = "RUNDSTEUEREMPFAENGER_IMS_MME")]
    RippleControlReceiverImsMme,

    /// Temperature compensation
    #[serde(rename = "TEMPERATUR_KOMPENSATION")]
    TemperatureCompensation,

    /// Maximum demand indicator
    #[serde(rename = "HOECHSTBELASTUNGS_ANZEIGER")]
    MaximumDemandIndicator,

    /// Other device
    #[serde(rename = "SONSTIGES_GERAET")]
    OtherDevice,

    /// EDL21
    #[serde(rename = "EDL_21")]
    Edl21,

    /// EDL 40 meter attachment
    #[serde(rename = "EDL_40_ZAEHLERAUFSATZ")]
    Edl40MeterAttachment,

    /// EDL 40
    #[serde(rename = "EDL_40")]
    Edl40,

    /// Telephone connection
    #[serde(rename = "TELEFONANSCHLUSS")]
    TelephoneConnection,

    /// GSM modem
    #[serde(rename = "MODEM_GSM")]
    ModemGsm,

    /// GPRS modem
    #[serde(rename = "MODEM_GPRS")]
    ModemGprs,

    /// Radio modem
    #[serde(rename = "MODEM_FUNK")]
    ModemRadio,

    /// GSM modem without load profile (provided by MSB)
    #[serde(rename = "MODEM_GSM_O_LG")]
    ModemGsmWithoutLoadProfile,

    /// GSM modem with load profile (provided by MSB)
    #[serde(rename = "MODEM_GSM_M_LG")]
    ModemGsmWithLoadProfile,

    /// Landline modem (provided by MSB)
    #[serde(rename = "MODEM_FESTNETZ")]
    ModemLandline,

    /// GPRS modem with load profile (provided by MSB)
    #[serde(rename = "MODEM_GPRS_M_LG")]
    ModemGprsWithLoadProfile,

    /// PLC communication equipment (Powerline)
    #[serde(rename = "PLC_KOM")]
    PlcCommunication,

    /// Ethernet communication equipment (LAN/WLAN)
    #[serde(rename = "ETHERNET_KOM")]
    EthernetCommunication,

    /// DSL communication equipment
    #[serde(rename = "DSL_KOM")]
    DslCommunication,

    /// LTE communication equipment
    #[serde(rename = "LTE_KOM")]
    LteCommunication,

    /// Compact volume converter
    #[serde(rename = "KOMPAKT_MU")]
    CompactVolumeConverter,

    /// System volume converter
    #[serde(rename = "SYSTEM_MU")]
    SystemVolumeConverter,

    /// Temperature volume converter
    #[serde(rename = "TEMPERATUR_MU")]
    TemperatureVolumeConverter,

    /// State volume converter
    #[serde(rename = "ZUSTANDS_MU")]
    StateVolumeConverter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&DeviceType::ModernMeasuringDevice).unwrap(),
            r#""MODERNE_MESSEINRICHTUNG""#
        );
    }

    #[test]
    fn test_roundtrip() {
        for dtype in [
            DeviceType::MultiplexSystem,
            DeviceType::DataLogger,
            DeviceType::ModernMeasuringDevice,
            DeviceType::IntelligentMeasuringSystem,
            DeviceType::ModemGsm,
        ] {
            let json = serde_json::to_string(&dtype).unwrap();
            let parsed: DeviceType = serde_json::from_str(&json).unwrap();
            assert_eq!(dtype, parsed);
        }
    }
}
