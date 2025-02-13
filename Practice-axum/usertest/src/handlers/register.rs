use axum::{Json, http::{StatusCode}, response::{IntoResponse, Response}, extract::State};
use sqlx::mysql::{MySqlPool, MySqlConnection};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::{ops::DerefMut, sync::Arc};
use tikv_client::RawClient;

// TiKV + MySQL client state
#[derive(Clone)]
pub struct AppState {
    pub tikv_client: Arc<RawClient>,
    pub pool: Arc<MySqlPool>,
}
impl AppState {
    pub fn new(tikv_client: RawClient, pool: MySqlPool) -> Self {
        AppState {
            tikv_client: Arc::new(tikv_client),
            pool: Arc::new(pool),
        }
    }
}

// Define User struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    username: String,
    password_hash: String,
    token: Option<String>,
}

// Register request struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

// Login request struct (changed to include `user` object)
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    user: UserRequest,  
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    username: String,
    password: String,
}

// Claims struct for JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

const SECRET_KEY: &[u8] = b"super_secret_key";

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Response, (StatusCode, String)> {
    // Ensure the table exists
    let create_table_query = r"
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            token TEXT NULL
        )
    ";
    
    let mut conn = state.pool.acquire().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to acquire database connection: {}", e)))?;

    sqlx::query(create_table_query)
        .execute(&mut *conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create table: {}", e)))?;

    // Hash the password
    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to hash password: {}", e)))?;

    // Insert the user
    let query = r"
        INSERT INTO users (username, password_hash, token)
        VALUES (?, ?, NULL)
    ";

    sqlx::query(query)
        .bind(&payload.username)
        .bind(&hashed_password)
        .execute(&mut *conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to execute query: {}", e)))?;

    Ok((StatusCode::CREATED, "User registered successfully").into_response())
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = payload.user;

    let mut conn = state.pool.acquire().await.unwrap();

    let query = r"SELECT id, password_hash FROM users WHERE username = ?";
    let result: Option<(i32, String)> = sqlx::query_as(query)
        .bind(&user.username)
        .fetch_optional(&mut *conn)
        .await
        .unwrap();

    if let Some((id, password_hash)) = result {
        if verify(&user.password, &password_hash).unwrap() {
            let claims = Claims {
                sub: user.username.clone(),
                exp: 10000000000,
            };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
                .unwrap();

            let update_query = r"UPDATE users SET token = ? WHERE id = ?";
            sqlx::query(update_query)
                .bind(&token)
                .bind(id)
                .execute(&mut *conn)
                .await
                .unwrap();

            return Json(serde_json::json!({ "token": token })).into_response();
        }
    }

    (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
}