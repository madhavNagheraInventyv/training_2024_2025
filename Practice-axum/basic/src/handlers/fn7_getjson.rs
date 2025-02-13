use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message{
    mess: String,
    count: i32,
    username: String,
}



pub async fn getjson()-> Json<Message>{
    let sent=Message{
        mess: "i am madhav".to_owned(), 
        count: 21, 
        username:"mdhavn".to_owned(),
    };
    Json(sent)


}