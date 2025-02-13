use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response}, Extension,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tikv_client::RawClient;
use sqlx::mysql::{MySqlPool, MySqlConnection};
use std::{sync::Arc, ops::DerefMut};






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

// Define the Car struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    id: String,
    make: String,
    model: String,
    year: u16,
}

// Car request struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CarRequest {
    make: String,
    model: String,
    year: u16,
}


async fn create_cars_table(conn: &mut MySqlConnection) -> Result<(), sqlx::Error> {
    let query = r"
        CREATE TABLE IF NOT EXISTS cars (
            id VARCHAR(255) PRIMARY KEY,
            make VARCHAR(255) NOT NULL,
            model VARCHAR(255) NOT NULL,
            year INT NOT NULL
        )
    ";
    sqlx::query(query).execute(conn).await?;
    Ok(())
}


pub async fn add_car(State(state): State<AppState>, Json(payload): Json<CarRequest>) -> impl IntoResponse {
    let car_id = Uuid::new_v4().to_string();

   
    let car = Car {
        id: car_id.clone(),
        make: payload.make,
        model: payload.model,
        year: payload.year,
    };

    // let car_data = serde_json::to_vec(&car).unwrap();
    // let key = car_id.as_bytes().to_owned();
    // if let Err(e) = state.tikv_client.put(key.clone(), car_data).await {
    //     return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to insert into TiKV: {}", e)).into_response();
    // }

    let mut conn = state.pool.acquire().await.unwrap();
    let mut conn = conn.deref_mut(); // Dereference to `MySqlConnection`
    if let Err(e) = create_cars_table(&mut conn).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create table in MySQL: {}", e)).into_response();
    }
    
    let query = r"
        INSERT INTO cars (id, make, model, year)
        VALUES (?, ?, ?, ?)
    ";
    if let Err(e) = sqlx::query(query)
        .bind(&car.id)
        .bind(&car.make)
        .bind(&car.model)
        .bind(car.year)
        .execute(conn)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to insert into MySQL: {}", e)).into_response();
    }

    // Return the newly added car
    (StatusCode::CREATED, Json(car)).into_response()
}


pub async fn get_car(
    State(state): State<AppState>,
    Extension(token): Extension<String>, // Get the token from extensions
    Path(car_id): Path<String>,
) -> impl IntoResponse {

    println!("Received Token: {}", token);

    let key = car_id.as_bytes().to_owned();

    // Fetch car from MySQL
    let mut conn = state.pool.acquire().await.unwrap();
    let mut conn = conn.deref_mut(); // Dereference to `MySqlConnection`
    if let Err(e) = create_cars_table(&mut conn).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create table in MySQL: {}", e)).into_response();
    }

    let query = r"SELECT id, make, model, year FROM cars WHERE id = ?";
    if let Ok(Some((id, make, model, year))) = sqlx::query_as::<_, (String, String, String, i32)>(query)
        .bind(&car_id)
        .fetch_optional(conn)
        .await
    {
        let car = Car {
            id,
            make,
            model,
            year: year as u16,
        };
        return Json(car).into_response();
    }

    // If the car is not found, return 404
    (StatusCode::NOT_FOUND, "Car not found").into_response()
}

pub async fn delete_car(
    State(state): State<AppState>,
    Extension(token): Extension<String>, // Get the token from extensions
    Path(car_id): Path<String>,
) -> impl IntoResponse {

    println!("Received Token: {}", token);

    let key = car_id.as_bytes().to_owned();

    // Delete the car from MySQL
    let mut conn = state.pool.acquire().await.unwrap();
    let mut conn = conn.deref_mut(); // Dereference to `MySqlConnection`
    if let Err(e) = create_cars_table(&mut conn).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create table in MySQL: {}", e)).into_response();
    }

    let query = r"DELETE FROM cars WHERE id = ?";
    let result = sqlx::query(query).bind(&car_id).execute(conn).await.unwrap();

    if result.rows_affected() > 0 {
        return (StatusCode::OK, "Car deleted").into_response();
    }

    // If the car is not found, return 404
    (StatusCode::NOT_FOUND, "Car not found").into_response()
}