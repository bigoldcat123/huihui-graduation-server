use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::{CreateFoodInput, RecommendationReactionInput, SuggestionInput, UpdateFoodInput}}, service::{self, auth::{CurrentRootUserId, CurrentUserId}}};

#[get("/recommendation")]
async fn recommendation(token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food::recommendation(token.0).await.into();
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

#[get("/recommendation/reaction/count")]
async fn reaction_count(token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food::count_reactions(token.into_inner().0).await.into();
    res.json()
}

#[get("/topTags")]
async fn top_liked_tags(token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food::top_liked_tags(token.into_inner().0).await.into();
    res.json()
}

#[post("/consecutiveSuggest")]
async fn consecutive_suggest(food_ids: Json<SuggestionInput>, token: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::food::consecutive_suggest(food_ids.0, token.0).await.into();
    res.json()
}

#[get("/list")]
async fn list_foods(
    #[search_param] page: Option<i64>,
    #[search_param] page_size: Option<i64>,
    _root: FromRequest<CurrentRootUserId>,
) {
    let res: ApiResponse<_> = service::food::list_foods_by_page(page, page_size).await.into();
    res.json()
}

#[post("/")]
async fn create_food(ipt: Json<CreateFoodInput>, _root: FromRequest<CurrentRootUserId>) {
    let res: ApiResponse<_> = service::food::create_food(ipt.0).await.into();
    res.json()
}

#[post("/update")]
async fn update_food(ipt: Json<UpdateFoodInput>, _root: FromRequest<CurrentRootUserId>) {
    let res: ApiResponse<_> = service::food::update_food(ipt.0).await.into();
    res.json()
}
