use thiserror::Error;

#[derive(Error, Debug)]
pub enum HachiError {
    #[error("D-Bus connection failed: {0}")]
    DbusConnection(#[from] zbus::Error),

    #[error("D-Bus method call failed: {0}")]
    DbusCall(String),

    #[error("Hardware actor channel closed")]
    ActorChannelClosed,

    #[error("Invalid power profile: {0}")]
    InvalidPowerProfile(String),

    #[error("Invalid fan curve: {0}")]
    InvalidFanCurve(String),

    #[error("Battery limit out of range: {0}")]
    BatteryLimitOutOfRange(u8),

    #[error("Terminal error: {0}")]
    Terminal(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, HachiError>;
