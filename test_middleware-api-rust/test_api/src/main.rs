use axum::{Router, routing::get, response::IntoResponse};
use hyper::{Server, StatusCode};
use std::net::SocketAddr;

async fn test() -> impl IntoResponse {
    println!("Test endpoint reached!");
    (StatusCode::ACCEPTED,"hey there")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/test", get(test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Run our application using hyper::Server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
