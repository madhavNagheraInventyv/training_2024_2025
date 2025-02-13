use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Validuser {
    username: Option<String>,
    password: String,
}

pub async fn validserde(Json(user): Json<Validuser>) -> impl IntoResponse {
    dbg!(&user);

    // Serialize user struct to JSON string
    let body = serde_json::to_string(&user).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(body))
        .unwrap()
}
