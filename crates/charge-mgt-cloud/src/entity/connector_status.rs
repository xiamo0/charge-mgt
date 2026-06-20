use sea_orm::{DbErr, DeriveActiveEnum, EnumIter};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    serde::Serialize,
    serde::Deserialize,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "charge_mgt_connector_status"
)]
pub enum ConnectorStatus {
    #[sea_orm(string_value = "Available")]
    Available,
    #[sea_orm(string_value = "Preparing")]
    Preparing,
    #[sea_orm(string_value = "Charging")]
    Charging,
    #[sea_orm(string_value = "SuspendedEVSE")]
    SuspendedEvse,
    #[sea_orm(string_value = "SuspendedEV")]
    SuspendedEv,
    #[sea_orm(string_value = "Finishing")]
    Finishing,
    #[sea_orm(string_value = "Reserved")]
    Reserved,
    #[sea_orm(string_value = "Unavailable")]
    Unavailable,
    #[sea_orm(string_value = "Faulted")]
    Faulted,
}

impl ConnectorStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Preparing => "Preparing",
            Self::Charging => "Charging",
            Self::SuspendedEvse => "SuspendedEVSE",
            Self::SuspendedEv => "SuspendedEV",
            Self::Finishing => "Finishing",
            Self::Reserved => "Reserved",
            Self::Unavailable => "Unavailable",
            Self::Faulted => "Faulted",
        }
    }
}

pub fn from_str_status(s: &str) -> Result<ConnectorStatus, DbErr> {
    match s {
        "Available" => Ok(ConnectorStatus::Available),
        "Preparing" => Ok(ConnectorStatus::Preparing),
        "Charging" => Ok(ConnectorStatus::Charging),
        "SuspendedEVSE" => Ok(ConnectorStatus::SuspendedEvse),
        "SuspendedEV" => Ok(ConnectorStatus::SuspendedEv),
        "Finishing" => Ok(ConnectorStatus::Finishing),
        "Reserved" => Ok(ConnectorStatus::Reserved),
        "Unavailable" => Ok(ConnectorStatus::Unavailable),
        "Faulted" => Ok(ConnectorStatus::Faulted),
        other => Err(DbErr::Type(format!("unknown connector status: {other}"))),
    }
}
