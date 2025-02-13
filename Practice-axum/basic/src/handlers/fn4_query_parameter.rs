use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};



#[derive(Debug,Serialize, Deserialize)]
pub struct QueryParam{
    message: String,
    id: i32,
}

pub async fn queryparameter(Query(query): Query<QueryParam>)-> Json<QueryParam>{
    Json(query)

}