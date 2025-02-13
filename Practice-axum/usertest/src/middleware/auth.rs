use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use sqlx::MySqlPool;
use std::{sync::Arc};

#[derive(Clone)]
pub struct AuthMiddleware {
    db_pool: Arc<MySqlPool>,
}

impl AuthMiddleware {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = MySqlPool::connect(database_url).await?;
        Ok(Self {
            db_pool: Arc::new(db_pool),
        })
    }

    pub async fn check_auth(
        State(state): State<Arc<AuthMiddleware>>, 
        mut req: Request<axum::body::Body>,
        next: Next,
    ) -> Response {
        // Extract Authorization header from the request
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        println!("🔍 Received Authorization Header: {:?}", auth_header);

        // If the Authorization header is missing, return Unauthorized
        if auth_header.is_none() {
            println!("❌ No Authorization header found! Returning 401.");
            return StatusCode::UNAUTHORIZED.into_response();
        }

        // Extract the token from the Authorization header
        let token = auth_header.unwrap().to_string();
        let token = token.strip_prefix("Bearer ").unwrap_or(&token).to_string();

        println!("🔑 Extracted Token: {}", token);

        // Check if the token exists in the database
        let result: Result<i64, sqlx::Error> = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE token = ?")
            .bind(&token)
            .fetch_one(&*state.db_pool)
            .await;

        match result {
            // If the token is found in the database, proceed to the next handler
            Ok(count) if count > 0 => {
                println!("✅ Token found in database: {}", count);
                next.run(req).await
            }
            // If the token is not found in the database, return Unauthorized
            Ok(_) => {
                println!("❌ Token NOT found in database");
                StatusCode::UNAUTHORIZED.into_response()
            }
            // Handle any database errors
            Err(e) => {
                println!("❌ Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
