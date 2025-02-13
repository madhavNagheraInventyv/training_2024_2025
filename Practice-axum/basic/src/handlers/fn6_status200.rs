use axum::{http::StatusCode, response::{IntoResponse, Response}};



pub async fn statuscode()->Response{

    (StatusCode::CREATED,()).into_response()

}