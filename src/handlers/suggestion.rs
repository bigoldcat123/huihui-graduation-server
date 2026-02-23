use faithea::{data::{Json, inbound::FromRequest}, get, post};

use crate::{model::{ApiResponse, input::{AddTodoLogInput, CreateSuggestionInput, MoveSuggestionNextInput, ReviewSuggestionInput}}, service::{self, auth::{CurrentRootUserId, CurrentUserId}}};

#[post("/")]
async fn create_suggestion(ipt: Json<CreateSuggestionInput>, user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::suggestion::create(user_id.into_inner().0, ipt.0).await.into();
    res.json()
}

#[get("/my")]
async fn list_my_suggestion(user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::suggestion::list_my(user_id.into_inner().0).await.into();
    res.json()
}

#[get("/{suggestion_id}")]
async fn get_suggestion_by_id(suggestion_id: i32, _user_id: FromRequest<CurrentUserId>) {
    let res: ApiResponse<_> = service::suggestion::get_by_id(suggestion_id).await.into();
    res.json()
}

#[get("/todo_log/{suggestion_id}/{suggestion_status}")]
async fn list_todo_logs_by_suggestion_status(
    suggestion_id: i32,
    suggestion_status: String,
    _user_id: FromRequest<CurrentUserId>,
) {
    let res: ApiResponse<_> =
        service::suggestion::list_todo_logs(suggestion_id, suggestion_status).await.into();
    res.json()
}

#[get("/list")]
async fn list_suggestion_by_page(
    #[search_param] page: Option<i64>,
    #[search_param] page_size: Option<i64>,
    #[search_param] status: Option<String>,
    #[search_param] suggestion_type: Option<String>,
    _root: FromRequest<CurrentRootUserId>,
) {
    let res: ApiResponse<_> = service::suggestion::list_by_page(
        page,
        page_size,
        status,
        suggestion_type,
    )
        .await
        .into();
    res.json()
}

#[get("/list/todos")]
async fn list_todos_by_page(
    #[search_param] page: Option<i64>,
    #[search_param] page_size: Option<i64>,
    _root: FromRequest<CurrentRootUserId>,
) {
    let res: ApiResponse<_> = service::suggestion::list_todos_by_page(page, page_size)
        .await
        .into();
    res.json()
}

#[post("/review")]
async fn review_suggestion(ipt: Json<ReviewSuggestionInput>, root: FromRequest<CurrentRootUserId>) {
    let res: ApiResponse<_> = service::suggestion::review(root.into_inner().0, ipt.0).await.into();
    res.json()
}

#[post("/next_stage")]
async fn move_suggestion_to_next_stage(
    ipt: Json<MoveSuggestionNextInput>,
    _root: FromRequest<CurrentRootUserId>,
) {
    let res: ApiResponse<_> = service::suggestion::move_to_next_stage(ipt.0).await.into();
    res.json()
}

#[post("/todo_log")]
async fn add_todo_log(
    ipt: Json<AddTodoLogInput>,
    _root: FromRequest<CurrentRootUserId>,
) {
    let res: ApiResponse<_> = service::suggestion::add_todo_log_by_current_status(ipt.0)
        .await
        .into();
    res.json()
}
