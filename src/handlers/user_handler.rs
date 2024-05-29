use axum::http::StatusCode;
use axum::{
    extract::Request,
    response::{IntoResponse, Response},
    Json,
};
use log::info;
use serde_json::json;

pub async fn register_users(req: Request<axum::body::Body>) -> Response {
    // Handle the request body and return a response
    let response_json = json!({ "msg": "user registered" });
    info!("user registered");
    (StatusCode::CREATED, Json(response_json)).into_response()
}
