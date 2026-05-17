use crate::types::Language;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundView {
    pub round_id: Uuid,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameView {
    pub game_id: Uuid,
    pub rounds: Vec<RoundView>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuessRequest {
    pub round_id: Uuid,
    pub language: Language,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuessResponse {
    pub correct: bool,
    pub correct_language: Language,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Language;
    use uuid::Uuid;

    fn fixed_uuid() -> Uuid {
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
    }

    // --- RoundView ---

    #[test]
    fn round_view_serialises_to_expected_json_shape() {
        let round = RoundView {
            round_id: fixed_uuid(),
            text: "Bonjour le monde.".to_string(),
        };
        let json = serde_json::to_string(&round).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["round_id"], "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(parsed["text"], "Bonjour le monde.");
    }

    #[test]
    fn round_view_round_trips() {
        let original = RoundView {
            round_id: fixed_uuid(),
            text: "Hello world.".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: RoundView = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    // --- GameView ---

    #[test]
    fn game_view_serialises_to_expected_json_shape() {
        let game = GameView {
            game_id: fixed_uuid(),
            rounds: vec![RoundView {
                round_id: fixed_uuid(),
                text: "Some text.".to_string(),
            }],
        };
        let json = serde_json::to_string(&game).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["game_id"], "550e8400-e29b-41d4-a716-446655440000");
        assert!(parsed["rounds"].is_array());
        assert_eq!(parsed["rounds"][0]["text"], "Some text.");
    }

    #[test]
    fn game_view_round_trips() {
        let original = GameView {
            game_id: fixed_uuid(),
            rounds: vec![
                RoundView { round_id: fixed_uuid(), text: "Round one.".to_string() },
                RoundView { round_id: fixed_uuid(), text: "Round two.".to_string() },
            ],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GameView = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    // --- GuessRequest ---

    #[test]
    fn guess_request_deserialises_from_json() {
        let json = r#"{"round_id":"550e8400-e29b-41d4-a716-446655440000","language":"French"}"#;
        let request: GuessRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.round_id, fixed_uuid());
        assert_eq!(request.language, Language::French);
    }

    #[test]
    fn guess_request_round_trips() {
        let original = GuessRequest {
            round_id: fixed_uuid(),
            language: Language::Japanese,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GuessRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    // --- GuessResponse ---

    #[test]
    fn guess_response_correct_serialises_to_expected_json() {
        let response = GuessResponse {
            correct: true,
            correct_language: Language::Arabic,
        };
        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["correct"], true);
        assert_eq!(parsed["correct_language"], "Arabic");
    }

    #[test]
    fn guess_response_incorrect_serialises_to_expected_json() {
        let response = GuessResponse {
            correct: false,
            correct_language: Language::Russian,
        };
        let json = serde_json::to_string(&response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["correct"], false);
        assert_eq!(parsed["correct_language"], "Russian");
    }

    #[test]
    fn guess_response_round_trips() {
        let original = GuessResponse {
            correct: false,
            correct_language: Language::English,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GuessResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    // --- Language serialisation ---

    #[test]
    fn language_serialises_as_plain_string_not_object() {
        let json = serde_json::to_string(&Language::French).unwrap();
        assert_eq!(json, r#""French""#);
    }

    #[test]
    fn language_deserialises_from_plain_string() {
        let lang: Language = serde_json::from_str(r#""Japanese""#).unwrap();
        assert_eq!(lang, Language::Japanese);
    }
}
