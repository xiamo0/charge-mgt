use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::error::AppError;
use crate::ocpp16::entity::enums::IdentityStatus;
use crate::ocpp16::entity::identity_info::{Column, Entity as IdentityInfos};
use crate::ocpp16::envelope::CloudMessage;
use crate::ocpp16::message_from_cp_handler::Handler;
use crate::state::AppState;
use ocpp_1_6::calls::AuthorizeRequest;
use ocpp_1_6::confs::AuthorizeConfirmation;

impl Handler<AuthorizeConfirmation> for AuthorizeRequest {
    async fn handel_detail(
        state: &AppState,
        msg: &CloudMessage,
    ) -> Result<AuthorizeConfirmation, AppError> {
        let req: AuthorizeRequest = serde_json::from_value(msg.payload.clone())?;

        let db = state.db()?;
        let record = IdentityInfos::find()
            .filter(Column::TagId.eq(req.id_tag.clone()))
            .one(db)
            .await?;

        let conf = match record {
            Some(m) if m.status == IdentityStatus::Accepted => AuthorizeConfirmation::accepted(),
            Some(_) => AuthorizeConfirmation::blocked(),
            None => AuthorizeConfirmation::invalid(),
        };

        Ok(conf)
    }
}
