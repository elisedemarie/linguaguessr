use common::api::{GameView, GuessRequest, GuessResponse};
use common::types::{GameMode, Language};
use uuid::Uuid;

use crate::mode::mode_str;

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
