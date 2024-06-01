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

pub async fn get_users(req: Request<axum::body::Body>) -> Response {
    let response_json = json!({ "msg": "get users" });
    info!("get users");
    (StatusCode::OK, Json(response_json)).into_response()
}
