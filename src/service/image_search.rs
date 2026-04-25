use crate::service::error::ServiceError;
use serde::Deserialize;
use std::fs;

const EXTERNAL_IMAGE_SERVICE_URL: &str = "http://127.0.0.1:8080/image";
const INSERT_ENDPOINT: &str = "http://127.0.0.1:8080/image/insert";

#[derive(Deserialize)]
struct ExternalResponse {
    _code: i32,
    _message: String,
    data: Option<i64>,
}

pub async fn insert_image(image_path: &str, cal: i32) -> Result<(), ServiceError> {
    let client = reqwest::Client::new();

    let file_bytes = fs::read(image_path)
        .map_err(|e| ServiceError::InvalidInput(format!("failed to read file: {}", e)))?;

    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name("image".to_string());

    let form = reqwest::multipart::Form::new()
        .part("image", part)
        .text("cal", cal.to_string());

    let resp = client
        .post(INSERT_ENDPOINT)
        .multipart(form)
        .send()
        .await
        .map_err(|e| ServiceError::InvalidInput(format!("request failed: {}", e)))?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(ServiceError::InvalidInput(format!(
            "external API error: {}",
            resp.status()
        )))
    }
}

pub async fn search_image(image_path: &str, file_name: Option<String>) -> Result<i64, ServiceError> {
    let client = reqwest::Client::new();

    let file_bytes = fs::read(image_path)
        .map_err(|e| ServiceError::InvalidInput(format!("failed to read file: {}", e)))?;

    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name(file_name.unwrap_or_else(|| "image".to_string()));

    let form = reqwest::multipart::Form::new().part("image", part);

    let resp = client
        .post(EXTERNAL_IMAGE_SERVICE_URL)
        .multipart(form)
        .send()
        .await
        .map_err(|e| ServiceError::InvalidInput(format!("request failed: {}", e)))?;

    if !resp.status().is_success() {
        return Err(ServiceError::InvalidInput(format!(
            "external API error: {}",
            resp.status()
        )));
    }

    let bytes = resp.bytes().await
        .map_err(|e| ServiceError::InvalidInput(format!("read response failed: {}", e)))?;

    let external: ExternalResponse = serde_json::from_slice(&bytes)
        .map_err(|e| ServiceError::JsonError(e))?;

    external.data
        .ok_or_else(|| ServiceError::InvalidInput("no calorie data returned".to_string()))
}