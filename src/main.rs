//mod router;

use axum::{
    routing::{get, post,},
    http::{ HeaderValue, StatusCode, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method,},
    Router, Json,
};
//use axum::http::Method;
use tower_http::cors::CorsLayer;
use serde::{ Serialize, Deserialize};

mod router;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION,ACCEPT,CONTENT_TYPE]);

    let app: Router<> = Router::new()
        .route("/", get(router::static_content_response::defaultResponse))
        .route("/api/test", get(||async {"test"})).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Hello, world!");
}
