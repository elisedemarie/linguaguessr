use common::api::{GameView, RoundView};
use common::types::Language;
use uuid::Uuid;

pub struct Round {
    pub round_id: Uuid,
    pub text: String,
    pub language: Language,
}

pub struct GameSession {
    pub game_id: Uuid,
    pub rounds: Vec<Round>,
}

pub fn session_to_view(session: &GameSession) -> GameView {
    GameView {
        game_id: session.game_id,
        rounds: session.rounds.iter().map(|r| RoundView {
            round_id: r.round_id,
            text: r.text.clone(),
            options: vec![],
        }).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::Language;
    use uuid::Uuid;

    fn make_session() -> GameSession {
        let game_id = Uuid::new_v4();
        let rounds = vec![
            Round { round_id: Uuid::new_v4(), text: "Bonjour le monde.".into(), language: Language::French },
            Round { round_id: Uuid::new_v4(), text: "Hello world.".into(), language: Language::English },
        ];
        GameSession { game_id, rounds }
    }

    #[test]
    fn view_preserves_game_id() {
        let session = make_session();
        let view = session_to_view(&session);
        assert_eq!(view.game_id, session.game_id);
    }

    #[test]
    fn view_has_same_number_of_rounds() {
        let session = make_session();
        let view = session_to_view(&session);
        assert_eq!(view.rounds.len(), session.rounds.len());
    }

    #[test]
    fn view_preserves_round_ids() {
        let session = make_session();
        let view = session_to_view(&session);
        for (round, view_round) in session.rounds.iter().zip(view.rounds.iter()) {
            assert_eq!(round.round_id, view_round.round_id);
        }
    }

    #[test]
    fn view_preserves_round_text() {
        let session = make_session();
        let view = session_to_view(&session);
        for (round, view_round) in session.rounds.iter().zip(view.rounds.iter()) {
            assert_eq!(round.text, view_round.text);
        }
    }

    #[test]
    fn view_round_has_no_language_field() {
        let session = make_session();
        let view = session_to_view(&session);
        let json = serde_json::to_string(&view.rounds[0]).unwrap();
        assert!(!json.contains("language"));
        assert!(!json.contains("French"));
    }
}
