//! Authorize Confirmation (Functional Block C)

use serde::{Deserialize, Serialize};
use crate::common::{AuthorizeCertificateStatusEnumType, IdTokenInfoType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeConfirmation {
    pub id_token_info: IdTokenInfoType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
}

impl AuthorizeConfirmation {
    pub fn new(id_token_info: IdTokenInfoType) -> Self {
        Self {
            id_token_info,
            certificate_status: None,
        }
    }

    pub fn accepted() -> Self {
        Self::new(IdTokenInfoType::accepted())
    }
}