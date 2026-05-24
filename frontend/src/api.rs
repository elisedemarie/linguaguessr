use common::api::{GameView, GuessRequest, GuessResponse};
use common::types::{GameMode, Language};
use serde::Serialize;
use uuid::Uuid;

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

pub async fn fetch_game(mode: &GameMode) -> Result<GameView, String> {
    let response = gloo_net::http::Request::get(
        &format!("{BACKEND_URL}/api/game?mode={}", mode_str(mode))
    )
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
