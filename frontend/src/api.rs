use common::api::{GameView, GuessRequest, GuessResponse};
use common::types::{GameMode, Language};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SeedScores {
    pub scores: Vec<u32>,
}

use crate::mode::mode_str;

#[derive(Serialize)]
pub struct FeedbackPayload {
    pub message:                          String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email:       Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id:     Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round_id:    Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language:    Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_url: Option<String>,
}

const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(url) => url,
    None => "http://localhost:3000",
};

pub(crate) fn seed_scores_url(backend: &str, seed: &str) -> String {
    format!("{backend}/api/seeds/{seed}/scores")
}

pub(crate) fn game_url(backend: &str, mode: &GameMode, seed: Option<&str>) -> String {
    match seed {
        Some(s) => format!("{backend}/api/game?seed={s}&mode={}", mode_str(mode)),
        None    => format!("{backend}/api/game?mode={}", mode_str(mode)),
    }
}

pub async fn fetch_game(mode: &GameMode, seed: Option<&str>) -> Result<GameView, String> {
    let response = gloo_net::http::Request::get(&game_url(BACKEND_URL, mode, seed))
    .send()
    .await
    .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response.json::<GameView>().await.map_err(|e| format!("Parse error: {e}"))
}

pub async fn submit_guess(
    game_id: Uuid,
    round_id: Uuid,
    language: Language,
) -> Result<GuessResponse, String> {
    let body = serde_json::to_string(&GuessRequest { round_id, language })
        .map_err(|e| format!("Serialise error: {e}"))?;

    let response = gloo_net::http::Request::post(
        &format!("{BACKEND_URL}/api/game/{game_id}/guess"),
    )
    .header("Content-Type", "application/json")
    .body(body)
    .map_err(|e| format!("Request error: {e}"))?
    .send()
    .await
    .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response.json::<GuessResponse>().await.map_err(|e| format!("Parse error: {e}"))
}

pub async fn post_seed_score(seed: &str, score: u32) -> Result<(), String> {
    let body = serde_json::to_string(&serde_json::json!({ "score": score }))
        .map_err(|e| format!("Serialise error: {e}"))?;
    let response = gloo_net::http::Request::post(&seed_scores_url(BACKEND_URL, seed))
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| format!("Request error: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;
    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }
    Ok(())
}

pub async fn fetch_seed_scores(seed: &str) -> Result<Vec<u32>, String> {
    let response = gloo_net::http::Request::get(&seed_scores_url(BACKEND_URL, seed))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;
    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }
    response.json::<SeedScores>().await
        .map(|s| s.scores)
        .map_err(|e| format!("Parse error: {e}"))
}

pub async fn submit_feedback(payload: &FeedbackPayload) -> Result<(), String> {
    let body = serde_json::to_string(payload)
        .map_err(|e| format!("Serialise error: {e}"))?;

    let response = gloo_net::http::Request::post(
        &format!("{BACKEND_URL}/api/feedback"),
    )
    .header("Content-Type", "application/json")
    .body(body)
    .map_err(|e| format!("Request error: {e}"))?
    .send()
    .await
    .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::GameMode;

    // --- seed_scores_url ---

    #[test]
    fn seed_scores_url_format() {
        let url = seed_scores_url("http://localhost:3000", "ABC123");
        assert_eq!(url, "http://localhost:3000/api/seeds/ABC123/scores");
    }

    #[test]
    fn seed_scores_url_different_seed() {
        let url = seed_scores_url("https://api.linguaguessr.io", "XYZ789");
        assert_eq!(url, "https://api.linguaguessr.io/api/seeds/XYZ789/scores");
    }

    // --- game_url ---

    #[test]
    fn game_url_without_seed_includes_mode() {
        let url = game_url("http://localhost:3000", &GameMode::Medium, None);
        assert_eq!(url, "http://localhost:3000/api/game?mode=medium");
    }

    #[test]
    fn game_url_without_seed_easy() {
        let url = game_url("http://localhost:3000", &GameMode::Easy, None);
        assert_eq!(url, "http://localhost:3000/api/game?mode=easy");
    }

    #[test]
    fn game_url_with_seed_includes_both() {
        let url = game_url("http://localhost:3000", &GameMode::Hard, Some("ABC123"));
        assert_eq!(url, "http://localhost:3000/api/game?seed=ABC123&mode=hard");
    }

    #[test]
    fn game_url_with_seed_medium() {
        let url = game_url("http://localhost:3000", &GameMode::Medium, Some("XYZ789"));
        assert_eq!(url, "http://localhost:3000/api/game?seed=XYZ789&mode=medium");
    }

    fn only_message() -> FeedbackPayload {
        FeedbackPayload {
            message:     "great game".into(),
            email:       None,
            game_id:     None,
            round_id:    None,
            language:    None,
            article_url: None,
        }
    }

    #[test]
    fn message_is_serialised() {
        let json = serde_json::to_value(only_message()).unwrap();
        assert_eq!(json["message"], "great game");
    }

    #[test]
    fn email_omitted_when_none() {
        let json = serde_json::to_value(only_message()).unwrap();
        assert!(!json.as_object().unwrap().contains_key("email"));
    }

    #[test]
    fn game_id_omitted_when_none() {
        let json = serde_json::to_value(only_message()).unwrap();
        assert!(!json.as_object().unwrap().contains_key("game_id"));
    }

    #[test]
    fn round_id_omitted_when_none() {
        let json = serde_json::to_value(only_message()).unwrap();
        assert!(!json.as_object().unwrap().contains_key("round_id"));
    }

    #[test]
    fn language_omitted_when_none() {
        let json = serde_json::to_value(only_message()).unwrap();
        assert!(!json.as_object().unwrap().contains_key("language"));
    }

    #[test]
    fn article_url_omitted_when_none() {
        let json = serde_json::to_value(only_message()).unwrap();
        assert!(!json.as_object().unwrap().contains_key("article_url"));
    }

    #[test]
    fn email_included_when_some() {
        let payload = FeedbackPayload { email: Some("a@b.com".into()), ..only_message() };
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["email"], "a@b.com");
    }

    #[test]
    fn game_id_included_when_some() {
        let id = Uuid::new_v4();
        let payload = FeedbackPayload { game_id: Some(id), ..only_message() };
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["game_id"], id.to_string());
    }

    #[test]
    fn round_id_included_when_some() {
        let id = Uuid::new_v4();
        let payload = FeedbackPayload { round_id: Some(id), ..only_message() };
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["round_id"], id.to_string());
    }

    #[test]
    fn language_included_when_some() {
        let payload = FeedbackPayload { language: Some("French".into()), ..only_message() };
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["language"], "French");
    }

    #[test]
    fn article_url_included_when_some() {
        let payload = FeedbackPayload { article_url: Some("https://fr.wikipedia.org/wiki/Foo".into()), ..only_message() };
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["article_url"], "https://fr.wikipedia.org/wiki/Foo");
    }

    #[test]
    fn all_fields_serialised_when_all_provided() {
        let game_id  = Uuid::new_v4();
        let round_id = Uuid::new_v4();
        let payload = FeedbackPayload {
            message:     "full".into(),
            email:       Some("x@y.com".into()),
            game_id:     Some(game_id),
            round_id:    Some(round_id),
            language:    Some("Japanese".into()),
            article_url: Some("https://ja.wikipedia.org/wiki/Bar".into()),
        };
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["message"],     "full");
        assert_eq!(json["email"],       "x@y.com");
        assert_eq!(json["game_id"],     game_id.to_string());
        assert_eq!(json["round_id"],    round_id.to_string());
        assert_eq!(json["language"],    "Japanese");
        assert_eq!(json["article_url"], "https://ja.wikipedia.org/wiki/Bar");
    }
}
