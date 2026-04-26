use crate::service::error::ServiceError;
use serde::{Deserialize, Serialize};
use std::fs;

const EXTERNAL_IMAGE_SERVICE_URL: &str = "http://127.0.0.1:8080/image";
const INSERT_ENDPOINT: &str = "http://127.0.0.1:8080/image/insert";

#[derive(Deserialize, Serialize,Debug)]
pub struct ExternalSearchResponse {
    description: String,
    cal: i64,
    image_url: String,
    food_name: String,
}

#[derive(Deserialize, Debug)]
struct ExternalApiResponse {
    code: i32,
    message: String,
    data: ExternalSearchResponse,
}

pub async fn insert_image(image_path: &str, cal: i32, food_name: &str, description: &str) -> Result<(), ServiceError> {
    let client = reqwest::Client::new();

    let file_bytes = fs::read(image_path)
        .map_err(|e| ServiceError::InvalidInput(format!("failed to read file: {}", e)))?;

    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name("image".to_string());

    let form = reqwest::multipart::Form::new()
        .part("image", part)
        .text("cal", cal.to_string())
        .text("food_name", food_name.to_string())
        .text("description", description.to_string());

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

pub async fn search_image(image_path: &str, file_name: Option<String>) -> Result<ExternalSearchResponse, ServiceError> {
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

    let external: ExternalApiResponse = serde_json::from_slice(&bytes)
        .map_err(|e| ServiceError::JsonError(e))?;
    println!("{external:?}");
    Ok(external.data)
}
