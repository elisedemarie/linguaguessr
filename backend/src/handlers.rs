use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use common::api::{GuessRequest, GuessResponse};
use common::api::RoundView;
use common::scoring::{partial_score, score_labels};
use common::types::{GameMode, Language};
use crate::game::{GameSession, Round, session_to_view};
use crate::wikipedia::{fetch_article, WikipediaClient};
use futures::future::join_all;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<HashMap<Uuid, GameSession>>>,
    pub wikipedia: Arc<dyn WikipediaClient>,
}

#[derive(Deserialize)]
pub struct GameParams {
    pub mode: Option<GameMode>,
}

pub(crate) fn language_pool(mode: &GameMode) -> &'static [Language] {
    match mode {
        GameMode::Easy   => Language::easy_pool(),
        GameMode::Medium => Language::medium_pool(),
        GameMode::Hard   => Language::all(),
    }
}

pub async fn get_game(
    State(state): State<AppState>,
    Query(params): Query<GameParams>,
) -> impl IntoResponse {
    let mode = params.mode.unwrap_or_default();
    let mut languages = language_pool(&mode).to_vec();
    rand::seq::SliceRandom::shuffle(languages.as_mut_slice(), &mut rand::thread_rng());
    let languages: Vec<Language> = languages.into_iter().take(5).collect();

    let results = join_all(
        languages.iter().map(|lang| fetch_article(lang, state.wikipedia.as_ref()))
    ).await;

    let mut rounds = Vec::new();
    for (lang, result) in languages.into_iter().zip(results) {
        match result {
            Ok(text) => rounds.push(Round { round_id: Uuid::new_v4(), text, language: lang }),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    }

    let game_id = Uuid::new_v4();
    let session = GameSession { game_id, rounds };
    let mut view = session_to_view(&session);

    if mode == GameMode::Easy {
        let pool = Language::easy_pool();
        for (round_view, round) in view.rounds.iter_mut().zip(session.rounds.iter()) {
            round_view.options = make_options(&round.language, pool);
        }
    }

    state.store.lock().unwrap().insert(game_id, session);

    Json(view).into_response()
}

pub(crate) fn make_options(correct: &Language, pool: &[Language]) -> Vec<Language> {
    let mut rng = rand::thread_rng();
    let mut distractors: Vec<Language> = pool.iter()
        .filter(|l| *l != correct)
        .cloned()
        .collect();
    rand::seq::SliceRandom::shuffle(distractors.as_mut_slice(), &mut rng);
    let mut options: Vec<Language> = distractors.into_iter().take(3).collect();
    options.push(correct.clone());
    rand::seq::SliceRandom::shuffle(options.as_mut_slice(), &mut rng);
    options
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
    let score = partial_score(&request.language, &round.language);
    let labels = score_labels(&request.language, &round.language);
    Json(GuessResponse {
        correct,
        correct_language: round.language.clone(),
        score,
        labels,
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

    #[tokio::test]
    async fn single_fetch_failure_among_five_returns_503() {
        // First fetch fails, rest succeed — whole game should still be 503
        struct OneFailsClient {
            failed: Arc<std::sync::atomic::AtomicBool>,
        }

        #[async_trait]
        impl WikipediaClient for OneFailsClient {
            async fn fetch_summary(&self, _url: &str) -> Result<String, FetchError> {
                if !self.failed.swap(true, std::sync::atomic::Ordering::SeqCst) {
                    Err(FetchError::Http("simulated failure".into()))
                } else {
                    Ok(sample_text().to_string())
                }
            }
        }

        let store = Arc::new(Mutex::new(HashMap::new()));
        let state = AppState {
            store: Arc::clone(&store),
            wikipedia: Arc::new(OneFailsClient {
                failed: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            }),
        };
        let app = Router::new()
            .route("/api/game", get(get_game))
            .with_state(state);

        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn fetches_complete_in_parallel() {
        // Each fetch sleeps 50ms. Sequential would take 250ms+, parallel takes ~50ms.
        struct SlowClient;

        #[async_trait]
        impl WikipediaClient for SlowClient {
            async fn fetch_summary(&self, _url: &str) -> Result<String, FetchError> {
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                Ok(sample_text().to_string())
            }
        }

        let store = Arc::new(Mutex::new(HashMap::new()));
        let state = AppState {
            store: Arc::clone(&store),
            wikipedia: Arc::new(SlowClient),
        };
        let app = Router::new()
            .route("/api/game", get(get_game))
            .with_state(state);

        let start = tokio::time::Instant::now();
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(elapsed.as_millis() < 200,
            "Expected parallel fetch < 200ms, took {}ms", elapsed.as_millis());
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
        assert_eq!(result.score.total, 1000);
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
        // English→French: Latin Jaccard 26/41 → 317 + IE-only family 1/5 → 90 = 407
        assert_eq!(result.score.total, 407);
    }

    #[tokio::test]
    async fn unrelated_guess_scores_zero() {
        let (app, store) = make_app(MockWikipediaClient::failing());
        let (session, game_id, round_id) = make_session(Language::Japanese, "日本語。");
        store.lock().unwrap().insert(game_id, session);

        let body = serde_json::json!({ "round_id": round_id, "language": "English" });
        let response = app.oneshot(post_request(&format!("/api/game/{game_id}/guess"), body)).await.unwrap();

        let result = parse_guess_response(response).await;
        // English (Latin, IE) vs Japanese (Japanese script, Japonic) = 0
        assert_eq!(result.score.total, 0);
    }

    #[tokio::test]
    async fn guess_response_includes_labels() {
        let (app, store) = make_app(MockWikipediaClient::failing());
        let (session, game_id, round_id) = make_session(Language::French, "Bonjour.");
        store.lock().unwrap().insert(game_id, session);

        let body = serde_json::json!({ "round_id": round_id, "language": "Spanish" });
        let response = app.oneshot(post_request(&format!("/api/game/{game_id}/guess"), body)).await.unwrap();

        let result = parse_guess_response(response).await;
        assert_eq!(result.labels.script, "Both Latin script");
        assert_eq!(result.labels.family, "Both Romance languages");
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

    // --- mode-aware game creation ---

    #[test]
    fn language_pool_medium_returns_30() {
        assert_eq!(language_pool(&GameMode::Medium).len(), 30);
    }

    #[test]
    fn language_pool_hard_returns_75() {
        assert_eq!(language_pool(&GameMode::Hard).len(), 75);
    }

    #[test]
    fn language_pool_easy_returns_10() {
        assert_eq!(language_pool(&GameMode::Easy).len(), 10);
    }

    #[test]
    fn language_pool_hard_contains_languages_outside_medium_pool() {
        let hard = language_pool(&GameMode::Hard);
        assert!(hard.contains(&Language::Swedish));
        assert!(hard.contains(&Language::Georgian));
    }

    #[test]
    fn language_pool_medium_does_not_contain_hard_only_languages() {
        let medium = language_pool(&GameMode::Medium);
        assert!(!medium.contains(&Language::Swedish));
        assert!(!medium.contains(&Language::Georgian));
    }

    #[tokio::test]
    async fn get_game_with_mode_medium_returns_200() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game?mode=medium")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_game_with_mode_hard_returns_200_and_five_rounds() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game?mode=hard")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let game = parse_game_view(response).await;
        assert_eq!(game.rounds.len(), 5);
    }

    #[tokio::test]
    async fn get_game_with_mode_easy_returns_200_and_five_rounds() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game?mode=easy")).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let game = parse_game_view(response).await;
        assert_eq!(game.rounds.len(), 5);
    }

    #[tokio::test]
    async fn get_game_with_no_mode_defaults_to_medium_pool_size() {
        assert_eq!(language_pool(&GameMode::default()).len(), 30);
    }

    // --- make_options ---

    #[test]
    fn make_options_returns_exactly_4() {
        let opts = make_options(&Language::French, Language::easy_pool());
        assert_eq!(opts.len(), 4);
    }

    #[test]
    fn make_options_always_includes_correct_language() {
        let opts = make_options(&Language::French, Language::easy_pool());
        assert!(opts.contains(&Language::French));
    }

    #[test]
    fn make_options_has_no_duplicates() {
        let opts = make_options(&Language::English, Language::easy_pool());
        let unique: std::collections::HashSet<_> = opts.iter().collect();
        assert_eq!(unique.len(), 4);
    }

    #[test]
    fn make_options_all_from_pool() {
        let pool = Language::easy_pool();
        let opts = make_options(&Language::Arabic, pool);
        for opt in &opts {
            assert!(pool.contains(opt));
        }
    }

    #[test]
    fn make_options_correct_appears_exactly_once() {
        let opts = make_options(&Language::Hindi, Language::easy_pool());
        assert_eq!(opts.iter().filter(|l| **l == Language::Hindi).count(), 1);
    }

    // --- easy mode HTTP ---

    #[tokio::test]
    async fn easy_mode_rounds_have_exactly_4_options() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game?mode=easy")).await.unwrap();
        let game = parse_game_view(response).await;
        for round in &game.rounds {
            assert_eq!(round.options.len(), 4);
        }
    }

    #[tokio::test]
    async fn medium_mode_rounds_have_no_options() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game?mode=medium")).await.unwrap();
        let game = parse_game_view(response).await;
        for round in &game.rounds {
            assert!(round.options.is_empty());
        }
    }

    #[tokio::test]
    async fn hard_mode_rounds_have_no_options() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game?mode=hard")).await.unwrap();
        let game = parse_game_view(response).await;
        for round in &game.rounds {
            assert!(round.options.is_empty());
        }
    }

    #[tokio::test]
    async fn default_mode_rounds_have_no_options() {
        let (app, _) = make_app(MockWikipediaClient::returning(sample_text()));
        let response = app.oneshot(get_request("/api/game")).await.unwrap();
        let game = parse_game_view(response).await;
        for round in &game.rounds {
            assert!(round.options.is_empty());
        }
    }
}
