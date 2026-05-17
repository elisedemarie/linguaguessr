use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use common::api::{GuessRequest, GuessResponse};
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
    let languages: Vec<Language> = languages.into_iter().take(5).collect();

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

pub async fn post_guess(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
    Json(request): Json<GuessRequest>,
) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    let Some(session) = store.get(&game_id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(round) = session.rounds.iter().find(|r| r.round_id == request.round_id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let correct = round.language == request.language;
    Json(GuessResponse {
        correct,
        correct_language: round.language.clone(),
    }).into_response()
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
            .route("/api/game/:game_id/guess", axum::routing::post(post_guess))
            .with_state(state);
        (app, store)
    }

    fn make_session(language: Language, text: &str) -> (GameSession, Uuid, Uuid) {
        let game_id = Uuid::new_v4();
        let round_id = Uuid::new_v4();
        let session = GameSession {
            game_id,
            rounds: vec![Round { round_id, text: text.to_string(), language }],
        };
        (session, game_id, round_id)
    }

    fn post_request(uri: &str, body: serde_json::Value) -> Request<axum::body::Body> {
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(axum::body::Body::from(body.to_string()))
            .unwrap()
    }

    async fn parse_guess_response(response: axum::response::Response) -> common::api::GuessResponse {
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
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

    // --- POST /api/game/:game_id/guess ---

    #[tokio::test]
    async fn correct_guess_returns_200_with_correct_true() {
        let (app, store) = make_app(MockWikipediaClient::failing());
        let (session, game_id, round_id) = make_session(Language::French, "Bonjour.");
        store.lock().unwrap().insert(game_id, session);

        let body = serde_json::json!({ "round_id": round_id, "language": "French" });
        let response = app.oneshot(post_request(&format!("/api/game/{game_id}/guess"), body)).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let result = parse_guess_response(response).await;
        assert!(result.correct);
        assert_eq!(result.correct_language, Language::French);
    }

    #[tokio::test]
    async fn wrong_guess_returns_200_with_correct_false() {
        let (app, store) = make_app(MockWikipediaClient::failing());
        let (session, game_id, round_id) = make_session(Language::French, "Bonjour.");
        store.lock().unwrap().insert(game_id, session);

        let body = serde_json::json!({ "round_id": round_id, "language": "English" });
        let response = app.oneshot(post_request(&format!("/api/game/{game_id}/guess"), body)).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let result = parse_guess_response(response).await;
        assert!(!result.correct);
        assert_eq!(result.correct_language, Language::French);
    }

    #[tokio::test]
    async fn unknown_game_id_returns_404() {
        let (app, _store) = make_app(MockWikipediaClient::failing());
        let body = serde_json::json!({ "round_id": Uuid::new_v4(), "language": "French" });
        let response = app.oneshot(post_request(&format!("/api/game/{}/guess", Uuid::new_v4()), body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn unknown_round_id_returns_404() {
        let (app, store) = make_app(MockWikipediaClient::failing());
        let (session, game_id, _) = make_session(Language::French, "Bonjour.");
        store.lock().unwrap().insert(game_id, session);

        let body = serde_json::json!({ "round_id": Uuid::new_v4(), "language": "French" });
        let response = app.oneshot(post_request(&format!("/api/game/{game_id}/guess"), body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn guessing_same_round_twice_returns_valid_response() {
        let (app, store) = make_app(MockWikipediaClient::failing());
        let (session, game_id, round_id) = make_session(Language::Japanese, "日本語。");
        store.lock().unwrap().insert(game_id, session);

        let body = serde_json::json!({ "round_id": round_id, "language": "Japanese" });
        let uri = format!("/api/game/{game_id}/guess");

        let first = app.clone().oneshot(post_request(&uri, body.clone())).await.unwrap();
        assert_eq!(first.status(), StatusCode::OK);

        let second = app.oneshot(post_request(&uri, body)).await.unwrap();
        assert_eq!(second.status(), StatusCode::OK);
    }
}
