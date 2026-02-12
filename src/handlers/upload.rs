use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use faithea::{
    data::inbound::multipart::{MultiPartFile, Multipart},
    post, MultipartData,
};

use crate::model::ApiResponse;

#[derive(MultipartData, Debug)]
struct UploadInput {
    files: Vec<MultiPartFile>,
}

#[post("/upload")]
async fn upload(files: Multipart<UploadInput>) {
    let input = files.into_inner();
    let upload_dir = "static/uploads";
    if let Err(e) = fs::create_dir_all(upload_dir) {
        return ApiResponse::<Vec<String>>::err(format!("create dir failed: {e}")).json();
    }

    let mut urls = Vec::with_capacity(input.files.len());
    for (idx, file) in input.files.into_iter().enumerate() {
        let ext = file
            .file_name
            .as_ref()
            .and_then(|name| Path::new(name).extension())
            .and_then(|ext| ext.to_str())
            .map(|ext| format!(".{ext}"))
            .unwrap_or_default();
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default();
        let file_name = format!("{stamp}_{idx}{ext}");
        let target = format!("{upload_dir}/{file_name}");
        if let Err(e) = fs::copy(&file.temp_path, &target) {
            return ApiResponse::<Vec<String>>::err(format!("save file failed: {e}")).json();
        }
        urls.push(format!("/static/uploads/{file_name}"));
    }

    ApiResponse::ok().data(urls).json()
}
