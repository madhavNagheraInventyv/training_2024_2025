use axum::{extract::FromRequest, http::StatusCode};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct validuser{
    username: String,
    password: String
}

#[async_trait]
impl <B> FromRequest<B> for validuser
where   
    B: Send,
{   
    type Rejection= (StatusCode, String);
    async  fn from_request(req: &mut RequestParts<B>) -> Result<Self,Self::Rejection> {
        let _ = req;
        
    }
    
} 

pub async fn extract(){

}