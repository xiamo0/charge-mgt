use thiserror::Error;

#[derive(Debug, Error)]
pub enum OcppError {
    #[error("WebSocket connect failed: {0}")]
    Connect(String),

    #[error("WebSocket send failed: {0}")]
    Send(String),

    #[error("connection closed")]
    ConnectionClosed,

    #[error("no pending server call with uniqueId `{0}`")]
    UnknownServerCall(String),

    #[error("raw envelope parse error: {0}")]
    Parse(String),
}
