mod game;
mod handlers;
mod wikipedia;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::http::{HeaderValue, Method};
use axum::routing::{get, post};
use axum::Router;
use handlers::{AppState, get_game, post_guess};
use tower_http::cors::CorsLayer;
use wikipedia::ReqwestWikipediaClient;

#[tokio::main]
async fn main() {
    let state = AppState {
        store: Arc::new(Mutex::new(HashMap::new())),
        wikipedia: Arc::new(ReqwestWikipediaClient::new()),
    };

    let cors = match std::env::var("FRONTEND_URL") {
        Ok(url) => CorsLayer::new()
            .allow_origin(url.parse::<HeaderValue>().expect("invalid FRONTEND_URL"))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE]),
        Err(_) => CorsLayer::permissive(),
    };

    let app = Router::new()
        .route("/api/game", get(get_game))
        .route("/api/game/:game_id/guess", post(post_guess))
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("Backend listening on http://0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}
