use axum::{Router, routing::get, response::IntoResponse};
use std::net::SocketAddr;
use axum::extract::{Path, Json};
use serde::Serialize;
use tokio::net::TcpListener; // ✅ Use TcpListener instead of Server::bind()

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/user/{id}", get(fetch_user_by_id));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct User<'a> {
    id: u32,
    username: &'a str,
}

pub async fn fetch_user_by_id(Path(user_id): Path<u32>) -> impl IntoResponse {
    Json(User {
        username: "madhav",
        id: user_id,
    })
}

async fn root() -> impl IntoResponse {
    "Hello from Axum server"
}
