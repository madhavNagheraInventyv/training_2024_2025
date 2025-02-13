
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
    Router,
};
use hyper::header::AUTHORIZATION;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MiddlewareState {
    pub token: Arc<Mutex<Option<String>>>,
}

impl MiddlewareState {
    pub fn new() -> Self {
        MiddlewareState {
            token: Arc::new(Mutex::new(None)),
        }
    }
}

pub async fn attach_token(
    State(state): State<Arc<MiddlewareState>>, // Accept Arc<MiddlewareState>
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    if let Some(token) = req.headers().get(AUTHORIZATION) {
        if let Ok(token_str) = token.to_str() {
            let mut stored_token = state.token.lock().unwrap();
            *stored_token = Some(token_str.to_string());

            println!("🔹 Token received in request: {}", token_str);
        }
    } 

    let mut response = next.run(req).await;

    let stored_token = state.token.lock().unwrap();
    if let Some(token) = stored_token.as_ref() {
        response.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_str(token).unwrap_or_else(|_| HeaderValue::from_static("")),
        );

       
    } 
    Ok(response)
}