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
    food_name: String,
    description: String,
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
    let food_name = input.food_name;
    let description = input.description;

    match image_search::insert_image(&image_path, cal, &food_name, &description).await {
        Ok(()) => ApiResponse::ok().json(),
        Err(e) => ApiResponse::<()>::err(format!("{:?}", e)).json(),
    }
}

#[post("/")]
async fn image_search(
    files: Multipart<SearchInput>,
) -> faithea::data::Json<ApiResponse<image_search::ExternalSearchResponse>> {
    let input = files.into_inner();
    let image_path = input.image.temp_path.clone();
    let file_name = input.image.file_name.clone();

    match image_search::search_image(&image_path, file_name).await {
        Ok(result) => {
            log::info!("{result:?}");
            ApiResponse::ok().data(result).json()
        },
        Err(e) => {
            log::info!("{e:?}");
            ApiResponse::<image_search::ExternalSearchResponse>::err(format!("{:?}", e)).json()
        },
    }
}
