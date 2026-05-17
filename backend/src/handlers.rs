use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use common::types::Language;
use crate::game::{GameSession, Round, session_to_view};
use crate::wikipedia::{fetch_article, WikipediaClient};

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<HashMap<Uuid, GameSession>>>,
    pub wikipedia: Arc<dyn WikipediaClient>,
}

pub async fn get_game(State(state): State<AppState>) -> impl IntoResponse {
    let mut languages = Language::all().to_vec();
    rand::seq::SliceRandom::shuffle(languages.as_mut_slice(), &mut rand::thread_rng());

    let mut rounds = Vec::new();
    for lang in languages {
        match fetch_article(&lang, state.wikipedia.as_ref()).await {
            Ok(text) => rounds.push(Round {
                round_id: Uuid::new_v4(),
                text,
                language: lang,
            }),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    }

    let game_id = Uuid::new_v4();
    let session = GameSession { game_id, rounds };
    let view = session_to_view(&session);
    state.store.lock().unwrap().insert(game_id, session);

    Json(view).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use axum::body::to_bytes;
    use axum::http::Request;
    use axum::routing::get;
    use axum::Router;
    use common::api::GameView;
    use crate::wikipedia::FetchError;
    use tower::ServiceExt;

    struct MockWikipediaClient {
        text: Result<String, ()>,
    }

    impl MockWikipediaClient {
        fn returning(text: &str) -> Arc<Self> {
            Arc::new(Self { text: Ok(text.to_string()) })
        }
        fn failing() -> Arc<Self> {
            Arc::new(Self { text: Err(()) })
        }
    }

    #[async_trait]
    impl WikipediaClient for MockWikipediaClient {
        async fn fetch_summary(&self, _url: &str) -> Result<String, FetchError> {
            self.text.as_ref()
                .map(|t| t.clone())
                .map_err(|_| FetchError::Http("mock error".into()))
        }
    }

    fn make_app(wikipedia: Arc<dyn WikipediaClient>) -> (Router, Arc<Mutex<HashMap<Uuid, GameSession>>>) {
        let store = Arc::new(Mutex::new(HashMap::new()));
        let state = AppState {
            store: Arc::clone(&store),
            wikipedia,
        };
        let app = Router::new()
            .route("/api/game", get(get_game))
            .with_state(state);
        (app, store)
    }

    fn get_request(uri: &str) -> Request<axum::body::Body> {
        Request::builder().uri(uri).body(axum::body::Body::empty()).unwrap()
    }

    async fn parse_game_view(response: axum::response::Response) -> GameView {
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    // sufficient text to pass the MIN_CHARS check
    fn sample_text() -> &'static str {
        "This is a sufficiently long extract from a Wikipedia article about something interesting and informative for the player to read."
    }

    #[tokio::test]
    async fn returns_200() {
        let (app, _store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn response_has_five_rounds() {
        let (app, _store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        assert_eq!(game.rounds.len(), 5);
    }

    #[tokio::test]
    async fn response_has_game_id() {
        let (app, _store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        assert_ne!(game.game_id, Uuid::nil());
    }

    #[tokio::test]
    async fn all_rounds_have_non_empty_text() {
        let (app, _store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        for round in &game.rounds {
            assert!(!round.text.is_empty());
        }
    }

    #[tokio::test]
    async fn all_rounds_have_unique_ids() {
        let (app, _store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        let ids: std::collections::HashSet<_> = game.rounds.iter().map(|r| r.round_id).collect();
        assert_eq!(ids.len(), 5);
    }

    #[tokio::test]
    async fn session_is_saved_to_store() {
        let (app, store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        let locked = store.lock().unwrap();
        assert!(locked.contains_key(&game.game_id));
    }

    #[tokio::test]
    async fn stored_session_has_five_rounds() {
        let (app, store) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        let locked = store.lock().unwrap();
        assert_eq!(locked[&game.game_id].rounds.len(), 5);
    }

    #[tokio::test]
    async fn wikipedia_failure_returns_503() {
        let (app, _store) = make_app(MockWikipediaClient::failing());
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
