use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct Messagebody{
    message: String
}
#[derive(Serialize, Debug)]
pub struct ResponseBody {
    response: String,
}
// #[derive(Serialize,Deserialize,Debug)]
// pub struct Testbody{
//     message: String,
//     from_server: Option<String>,
// }
#[warn(unused_must_use)]
pub async fn jsonbody(Json(body): Json<Messagebody>)-> impl IntoResponse {
    dbg!(&body); 
    let response = ResponseBody {
        response: format!("Received: {}", body.message),
    };

    let _ = Json(response);

}
// pub async fn jsonbody(Json(body): Json<Testbody>) ->Json<Testbody>{
//     Json(Testbody{message:body.message, from_server: Some("hello from code side".to_owned()),})
// }