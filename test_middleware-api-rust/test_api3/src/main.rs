use axum::{
    extract::{Json, Path, State},
    routing::{get, post},
    response::IntoResponse,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
// use axum_extra::extract::WithRejection;
// use axum_extra::debug_handler;

#[derive(Debug, Clone)]
struct AppState {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateUserPayload {
    username: String,
    email: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
}


// #[axum::debug_handler]
async fn create_user(
    State(state): State<Arc<AppState>>, 
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    println!("State: {:?}", state.name);
    Json(payload) 
}

// #[axum::debug_handler]
async fn get_user(
    State(state): State<Arc<AppState>>, 
    Path(user_id): Path<u32>,
) -> impl IntoResponse {
    println!("Fetching user with ID: {}", user_id);
    Json(User {
        id: user_id,
        username: "JohnDoe".to_string(),
        email: "john@example.com".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        name: "App State".to_string(),
    });

    // Attach state to the entire router
    let app = Router::new()
        .route("/users", post(create_user)) 
        .route("/users/{id}", get(get_user)) 
        .with_state(shared_state.clone()); // Attach shared state

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
