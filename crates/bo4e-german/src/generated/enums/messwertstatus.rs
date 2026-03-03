#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Messwertstatus {
    #[serde(rename = "ABGELESEN")]
    Abgelesen,
    #[serde(rename = "ERSATZWERT")]
    Ersatzwert,
    #[serde(rename = "ANGABE_FUER_LIEFERSCHEIN")]
    AngabeFRLieferschein,
    #[serde(rename = "VORSCHLAGSWERT")]
    Vorschlagswert,
    #[serde(rename = "NICHT_VERWENDBAR")]
    NichtVerwendbar,
    #[serde(rename = "PROGNOSEWERT")]
    Prognosewert,
    #[serde(rename = "VORLAEUFIGERWERT")]
    VorlUfigerWert,
    #[serde(rename = "ENERGIEMENGESUMMIERT")]
    EnergiemengeSummiert,
    #[serde(rename = "FEHLT")]
    Fehlt,
}
impl From<bo4e_core::enums::MeasuredValueStatus> for Messwertstatus {
    fn from(v: bo4e_core::enums::MeasuredValueStatus) -> Self {
        match v {
            bo4e_core::enums::MeasuredValueStatus::Read => Messwertstatus::Abgelesen,
            bo4e_core::enums::MeasuredValueStatus::Substitute => {
                Messwertstatus::Ersatzwert
            }
            bo4e_core::enums::MeasuredValueStatus::DeliveryNoteInfo => {
                Messwertstatus::AngabeFRLieferschein
            }
            bo4e_core::enums::MeasuredValueStatus::Proposed => {
                Messwertstatus::Vorschlagswert
            }
            bo4e_core::enums::MeasuredValueStatus::NotUsable => {
                Messwertstatus::NichtVerwendbar
            }
            bo4e_core::enums::MeasuredValueStatus::Forecast => {
                Messwertstatus::Prognosewert
            }
            bo4e_core::enums::MeasuredValueStatus::Preliminary => {
                Messwertstatus::VorlUfigerWert
            }
            bo4e_core::enums::MeasuredValueStatus::EnergySummed => {
                Messwertstatus::EnergiemengeSummiert
            }
            bo4e_core::enums::MeasuredValueStatus::Missing => Messwertstatus::Fehlt,
            _ => panic!("Unknown {} variant", stringify!(MeasuredValueStatus)),
        }
    }
}
impl From<Messwertstatus> for bo4e_core::enums::MeasuredValueStatus {
    fn from(v: Messwertstatus) -> Self {
        match v {
            Messwertstatus::Abgelesen => bo4e_core::enums::MeasuredValueStatus::Read,
            Messwertstatus::Ersatzwert => {
                bo4e_core::enums::MeasuredValueStatus::Substitute
            }
            Messwertstatus::AngabeFRLieferschein => {
                bo4e_core::enums::MeasuredValueStatus::DeliveryNoteInfo
            }
            Messwertstatus::Vorschlagswert => {
                bo4e_core::enums::MeasuredValueStatus::Proposed
            }
            Messwertstatus::NichtVerwendbar => {
                bo4e_core::enums::MeasuredValueStatus::NotUsable
            }
            Messwertstatus::Prognosewert => {
                bo4e_core::enums::MeasuredValueStatus::Forecast
            }
            Messwertstatus::VorlUfigerWert => {
                bo4e_core::enums::MeasuredValueStatus::Preliminary
            }
            Messwertstatus::EnergiemengeSummiert => {
                bo4e_core::enums::MeasuredValueStatus::EnergySummed
            }
            Messwertstatus::Fehlt => bo4e_core::enums::MeasuredValueStatus::Missing,
            _ => panic!("Unknown {} variant", stringify!(Messwertstatus)),
        }
    }
}
