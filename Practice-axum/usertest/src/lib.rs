use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use sqlx::mysql::MySqlPool;
use tikv_client::RawClient;
// use crate::middleware::auth::AuthMiddleware;  // Import AuthMiddleware

mod handlers;
mod middleware;

use handlers::{register::{register, login}, functionality::{get_car, add_car, delete_car}};
use handlers::register::AppState as RegisterAppState;
use handlers::functionality::AppState as FunctionalityAppState;
use middleware::{auth::AuthMiddleware, idheader::attach_token};
use middleware::log::log_request;
use middleware::idheader::MiddlewareState;


pub async fn run() {
    // Initialize database and TiKV clients
    let tikv_client = RawClient::new(vec!["127.0.0.1:2379"])
        .await
        .expect("Failed to connect to TiKV");

    let pool = MySqlPool::connect("mysql://root:@127.0.0.1:4000/db")
        .await
        .expect("Failed to connect to TiDB");

    let register_state = RegisterAppState::new(tikv_client.clone(), pool.clone());
    let functionality_state = FunctionalityAppState::new(tikv_client.clone(), pool.clone());
    
    let middleware_state = Arc::new(MiddlewareState::new());

    let auth_middleware = AuthMiddleware::new("mysql://root:@127.0.0.1:4000/db")
        .await
        .expect("Failed to initialize AuthMiddleware");

    let shared_auth_middleware = Arc::new(auth_middleware); // Wrap it in Arc

    // Public routes
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(register_state);

    // Protected routes
    let protected_routes = Router::new()
        .route("/get/{id}", get(get_car))
        .route("/add", post(add_car))
        .route("/delete/{id}", delete(delete_car))
        .layer(from_fn_with_state(middleware_state.clone(), attach_token))
        .layer(from_fn_with_state(shared_auth_middleware.clone(), AuthMiddleware::check_auth))
        .with_state(functionality_state);

    // Combine public and protected routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(from_fn(log_request)); 

    
    let listener = TcpListener::bind("0.0.0.0:3002")
        .await
        .expect("Failed to bind TCP listener");

    println!("Server running on http://127.0.0.1:3002");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
