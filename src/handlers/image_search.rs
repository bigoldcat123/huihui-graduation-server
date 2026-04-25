use faithea::{
    data::inbound::multipart::{MultiPartFile, Multipart},
    get, post, MultipartData,
};

use crate::model::ApiResponse;
use crate::service::image_search;

#[derive(MultipartData, Debug)]
struct InsertInput {
    image: MultiPartFile,
    cal: i32,
}

#[derive(MultipartData, Debug)]
struct SearchInput {
    image: MultiPartFile,
}

#[get("/image")]
async fn image_health() {
    ApiResponse::ok().data("get image").json()
}

#[post("/image/insert")]
async fn image_insert(
    files: Multipart<InsertInput>,
) -> faithea::data::Json<ApiResponse<()>> {
    let input = files.into_inner();
    let image_path = input.image.temp_path.clone();
    let cal = input.cal;

    match image_search::insert_image(&image_path, cal).await {
        Ok(()) => ApiResponse::ok().json(),
        Err(e) => ApiResponse::<()>::err(format!("{:?}", e)).json(),
    }
}

#[post("/")]
async fn image_search(
    files: Multipart<SearchInput>,
) -> faithea::data::Json<ApiResponse<i64>> {
    let input = files.into_inner();
    let image_path = input.image.temp_path.clone();
    let file_name = input.image.file_name.clone();

    match image_search::search_image(&image_path, file_name).await {
        Ok(cal) => ApiResponse::ok().data(cal).json(),
        Err(e) => ApiResponse::<i64>::err(format!("{:?}", e)).json(),
    }
}
