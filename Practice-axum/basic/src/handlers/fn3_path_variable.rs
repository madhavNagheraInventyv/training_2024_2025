use axum::{extract::Path, response::IntoResponse, Json};
use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    name: String,
    id: u32,
}

impl Messages {
    pub fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

pub async fn path_variable(Path(id): Path<u32>) -> impl IntoResponse {
    let user1 = Messages::new(id, "Madhav".to_string());
    Json(Messages::new(id, user1.name.clone()))
}

pub async fn path(Path(id): Path<u32>) -> impl IntoResponse {
    id.to_string()
}
