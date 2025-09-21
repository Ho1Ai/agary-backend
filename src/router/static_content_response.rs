use axum::http::StatusCode;
use axum::Json;
use serde::{Serialize, Deserialize};
use crate::router::response_models;

#[derive(Deserialize)]
pub struct HelpOffer {
    username: String,
    token: String,
    github: String,
}

pub async fn defaultResponse()->&'static str {
    "Hello, World!"
}

pub async fn HelpOffer (Json(payload): Json<HelpOffer>) -> (StatusCode, response_models::DefaultResponseModel) {
    let mut response = response_models::DefaultResponseModel{ state: 0, };
    return (StatusCode::CREATED, response);
}
