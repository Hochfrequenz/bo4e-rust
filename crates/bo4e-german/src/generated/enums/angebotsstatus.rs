#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum Angebotsstatus {
    #[serde(rename = "KONZEPTION")]
    Konzeption,
    #[serde(rename = "UNVERBINDLICH")]
    Unverbindlich,
    #[serde(rename = "VERBINDLICH")]
    Verbindlich,
    #[serde(rename = "BEAUFTRAGT")]
    Beauftragt,
    #[serde(rename = "UNGUELTIG")]
    Ungueltig,
    #[serde(rename = "ABGELEHNT")]
    Abgelehnt,
    #[serde(rename = "NACHGEFASST")]
    Nachgefasst,
    #[serde(rename = "AUSSTEHEND")]
    Ausstehend,
    #[serde(rename = "ERLEDIGT")]
    Erledigt,
}
impl From<bo4e_core::enums::OfferStatus> for Angebotsstatus {
    fn from(v: bo4e_core::enums::OfferStatus) -> Self {
        match v {
            bo4e_core::enums::OfferStatus::Concept => Angebotsstatus::Konzeption,
            bo4e_core::enums::OfferStatus::NonBinding => Angebotsstatus::Unverbindlich,
            bo4e_core::enums::OfferStatus::Binding => Angebotsstatus::Verbindlich,
            bo4e_core::enums::OfferStatus::Commissioned => Angebotsstatus::Beauftragt,
            bo4e_core::enums::OfferStatus::Invalid => Angebotsstatus::Ungueltig,
            bo4e_core::enums::OfferStatus::Rejected => Angebotsstatus::Abgelehnt,
            bo4e_core::enums::OfferStatus::FollowedUp => Angebotsstatus::Nachgefasst,
            bo4e_core::enums::OfferStatus::Pending => Angebotsstatus::Ausstehend,
            bo4e_core::enums::OfferStatus::Completed => Angebotsstatus::Erledigt,
            _ => panic!("Unknown {} variant", stringify!(OfferStatus)),
        }
    }
}
impl From<Angebotsstatus> for bo4e_core::enums::OfferStatus {
    fn from(v: Angebotsstatus) -> Self {
        match v {
            Angebotsstatus::Konzeption => bo4e_core::enums::OfferStatus::Concept,
            Angebotsstatus::Unverbindlich => bo4e_core::enums::OfferStatus::NonBinding,
            Angebotsstatus::Verbindlich => bo4e_core::enums::OfferStatus::Binding,
            Angebotsstatus::Beauftragt => bo4e_core::enums::OfferStatus::Commissioned,
            Angebotsstatus::Ungueltig => bo4e_core::enums::OfferStatus::Invalid,
            Angebotsstatus::Abgelehnt => bo4e_core::enums::OfferStatus::Rejected,
            Angebotsstatus::Nachgefasst => bo4e_core::enums::OfferStatus::FollowedUp,
            Angebotsstatus::Ausstehend => bo4e_core::enums::OfferStatus::Pending,
            Angebotsstatus::Erledigt => bo4e_core::enums::OfferStatus::Completed,
            _ => panic!("Unknown {} variant", stringify!(Angebotsstatus)),
        }
    }
}
