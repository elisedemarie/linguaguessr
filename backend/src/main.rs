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

fn build_router(state: AppState, frontend_url: Option<&str>) -> Router {
    let cors = match frontend_url {
        Some(url) => CorsLayer::new()
            .allow_origin(url.parse::<HeaderValue>().expect("invalid FRONTEND_URL"))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE]),
        None => CorsLayer::permissive(),
    };

    Router::new()
        .route("/api/game", get(get_game))
        .route("/api/game/:game_id/guess", post(post_guess))
        .layer(cors)
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        store: Arc::new(Mutex::new(HashMap::new())),
        wikipedia: Arc::new(ReqwestWikipediaClient::new()),
    };

    let frontend_url = std::env::var("FRONTEND_URL").ok();
    let app = build_router(state, frontend_url.as_deref());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    println!("Backend listening on http://0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use axum::body::Body;
    use axum::http::Request;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;
    use wikipedia::{FetchError, WikipediaClient};

    struct AlwaysFailingClient;

    #[async_trait]
    impl WikipediaClient for AlwaysFailingClient {
        async fn fetch_summary(&self, _url: &str) -> Result<String, FetchError> {
            Err(FetchError::Http("mock".into()))
        }
    }

    fn make_state() -> AppState {
        AppState {
            store: Arc::new(Mutex::new(HashMap::new())),
            wikipedia: Arc::new(AlwaysFailingClient),
        }
    }

    fn get_with_origin(origin: &str) -> Request<Body> {
        Request::builder()
            .uri("/api/game")
            .header("Origin", origin)
            .body(Body::empty())
            .unwrap()
    }

    #[tokio::test]
    async fn permissive_cors_allows_any_origin() {
        let app = build_router(make_state(), None);
        let response = app.oneshot(get_with_origin("http://any.example.com")).await.unwrap();
        assert!(response.headers().contains_key("access-control-allow-origin"));
    }

    #[tokio::test]
    async fn locked_cors_echoes_configured_origin() {
        let app = build_router(make_state(), Some("http://allowed.example.com"));
        let response = app.oneshot(get_with_origin("http://allowed.example.com")).await.unwrap();
        assert_eq!(
            response.headers().get("access-control-allow-origin").unwrap(),
            "http://allowed.example.com",
        );
    }

    #[tokio::test]
    async fn locked_cors_does_not_echo_non_allowed_origin() {
        let app = build_router(make_state(), Some("http://allowed.example.com"));
        let response = app.oneshot(get_with_origin("http://other.example.com")).await.unwrap();
        // tower-http returns the configured allowed origin, not the request origin.
        // The browser blocks the response because ACAO ≠ request Origin — CORS is enforced.
        let header = response.headers()
            .get("access-control-allow-origin")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_ne!(header, "http://other.example.com");
    }
}
