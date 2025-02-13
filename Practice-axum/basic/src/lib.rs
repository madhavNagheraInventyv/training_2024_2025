use axum::{routing::{get, post}, Router};
mod handlers;
use handlers::hello_world::hello;
use handlers::fn1::mirror_body;
use handlers::fn2_json_body::jsonbody;
use handlers::fn3_path_variable::path_variable;
use handlers::fn3_path_variable::path;
use handlers::fn4_query_parameter::queryparameter;
use handlers::fn5_always_error::alwayserror;
use handlers::fn6_status200::statuscode;
use handlers::fn7_getjson::getjson;
use handlers::fn8_validateserde::validserde;
use handlers::fn9_extractjson::extract;

 pub async fn run(){
let app=Router::new().route("/", get(hello))
                            .route("/gettext", post(mirror_body))
                            .route("/getjson", post(jsonbody))
                            .route("/pathv/{id}", get(path_variable))
                            .route("/path/{id}", get(path))
                            .route("/query", get(queryparameter))
                            .route("/error", get(alwayserror))
                            .route("/code201", get(statuscode))
                            .route("/returnjson",get(getjson))
                            .route("/serde", post(validserde))
                            .route("/extract", post(extract));
    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}