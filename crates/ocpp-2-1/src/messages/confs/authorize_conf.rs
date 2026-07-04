//! Authorize Confirmation (Block C)
use crate::common::{AuthorizeCertificateStatusEnumType, IdTokenInfoType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeConfirmation {
    pub id_token_info: IdTokenInfoType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
}

impl AuthorizeConfirmation {
    pub fn accepted() -> Self {
        Self {
            id_token_info: IdTokenInfoType::accepted(),
            certificate_status: None,
        }
    }
}
