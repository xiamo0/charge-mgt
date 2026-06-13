use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("FormationViolation: {0}")]
    FormationViolation(String),

    #[error("PropertyConstraintViolation: {0}")]
    PropertyConstraintViolation(String),

    #[error("InternalError: {0}")]
    InternalError(String),

    #[error("NotSupported: {0}")]
    NotSupported(String),
}

impl HandlerError {
    pub fn to_ocpp_error(&self) -> (&'static str, String) {
        let code = match self {
            Self::FormationViolation(_) => "FormationViolation",
            Self::PropertyConstraintViolation(_) => "PropertyConstraintViolation",
            Self::InternalError(_) => "InternalError",
            Self::NotSupported(_) => "NotSupported",
        };
        (code, self.to_string())
    }
}

impl From<sqlx::Error> for HandlerError {
    fn from(e: sqlx::Error) -> Self {
        Self::InternalError(e.to_string())
    }
}

impl From<serde_json::Error> for HandlerError {
    fn from(e: serde_json::Error) -> Self {
        Self::FormationViolation(e.to_string())
    }
}
