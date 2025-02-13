use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use rand::Rng;
use tower_cookies::{Cookie, Cookies};
use crate::state::AppState;

fn generate_csrf_token() -> String {
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| rng.gen_range(33..127) as u8 as char)
        .collect()
}

// Route to get a CSRF token
pub async fn get_csrf_token(cookies: Cookies) -> impl IntoResponse {
    let csrf_token = generate_csrf_token();
    cookies.add(
        Cookie::build("csrf_token", csrf_token.clone())
            .same_site(tower_cookies::cookie::SameSite::Strict)
            .http_only(true)
            .finish(),
    );
    (StatusCode::OK, csrf_token)
}

// Middleware to check CSRF token in protected routes
pub async fn protected_csrf_route(
    headers: HeaderMap,
    cookies: Cookies,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    let csrf_cookie = cookies.get("csrf_token").map(|c| c.value().to_string());
    let csrf_header = headers.get("X-CSRF-Token").map(|h| h.to_str().unwrap().to_string());

    if csrf_cookie.is_some() && csrf_cookie == csrf_header {
        (StatusCode::OK, "CSRF validation passed").into_response()
    } else {
        (StatusCode::FORBIDDEN, "CSRF validation failed").into_response()
    }
}

// Routes for CSRF
pub fn csrf_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/csrf-token", get(get_csrf_token))
        .route("/protected", post(protected_csrf_route))
}
