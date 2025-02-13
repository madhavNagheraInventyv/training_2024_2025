use axum::{http::StatusCode};

pub async fn alwayserror()-> Result<(), StatusCode> {
 Err(StatusCode::IM_A_TEAPOT )
}