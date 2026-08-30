use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::api::get_api_router;

mod frontend;
mod api;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .fallback(frontend::static_handler)
        .nest("/api", get_api_router())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    println!("Listening on http://0.0.0.0:4000");
    axum::serve(listener, app).await.unwrap();
}
