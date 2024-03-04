use thiserror::Error;

#[derive(Error, Debug)]
pub enum DAError {
    #[error("Communication error")]
    CommunicationError,
    #[error("Conversion error")]
    ConversionError,
    #[error("Set value error")]
    SetValueError(String),
    #[error("No such field")]
    NoSuchFieldError,
    #[error("Value conversion error")]
    ValueConversionError,
    #[error("Url Parse error")]
    UrlParseError,
    #[error("WebSocket Error")]
    WebSocketError,
}
