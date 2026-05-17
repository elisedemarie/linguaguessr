mod game;
mod handlers;
mod wikipedia;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::routing::{get, post};
use axum::Router;
use handlers::{AppState, get_game, post_guess};
use wikipedia::ReqwestWikipediaClient;

#[tokio::main]
async fn main() {
    let state = AppState {
        store: Arc::new(Mutex::new(HashMap::new())),
        wikipedia: Arc::new(ReqwestWikipediaClient::new()),
    };

    let app = Router::new()
        .route("/api/game", get(get_game))
        .route("/api/game/:game_id/guess", post(post_guess))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Backend listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
