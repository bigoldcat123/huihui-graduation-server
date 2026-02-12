use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::{RecommendationReactionInput, SuggestionInput}}, service::{self, auth::CurrentUserId}};

#[get("/recommendation")]
async fn recommendation(token: FromRequest<CurrentUserId>) {
    println!("{:?}",token.into_inner().0);
    let res: ApiResponse<_> = service::food::recommendation(1).await.into();
    res.json()
}

#[post("/recommendation/reaction")]
async fn recommendation_reaction(
    ipt: Json<RecommendationReactionInput>,
    token: FromRequest<CurrentUserId>,
) {
    println!("{:?}",ipt);
    let res: ApiResponse<_> = service::food::save_reaction(token.into_inner().0, ipt.0).await.into();
    res.json()
}

#[post("/consecutiveSuggest")]
async fn consecutive_suggest(food_ids: Json<SuggestionInput>, token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food::consecutive_suggest(food_ids.0, token.into_inner().0).await.into();
    res.json()
}
