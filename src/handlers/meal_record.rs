use faithea::get;
use faithea::data::inbound::FromRequest;

use crate::{
    model::ApiResponse,
    service::{self, auth::CurrentUserId},
};

#[get("/")]
async fn get_today(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> =
        service::meal_record::get_today_records(user_id.into_inner().0).await.into();
    res.json()
}
