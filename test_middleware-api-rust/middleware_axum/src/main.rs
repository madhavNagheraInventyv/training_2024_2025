use axum::{
    extract::Extension,
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};


async fn request_counter_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    if let Some(counter) = req.extensions().get::<Arc<AtomicUsize>>() {
        let current_count = counter.fetch_add(1, Ordering::SeqCst) + 1;
        println!("Request number: {}", current_count);
    } else {
        eprintln!("Counter extension not found!");
    }

    next.run(req).await
}

async fn hello_handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let app = Router::new()
        .route("/", get(hello_handler))
      
        .layer(middleware::from_fn(request_counter_middleware))
        
        .layer(Extension(counter));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // server run.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
