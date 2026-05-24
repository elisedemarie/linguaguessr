use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FeedbackRequest {
    pub message:  String,
    pub email:    Option<String>,
    pub game_id:  Option<Uuid>,
    pub round_id: Option<Uuid>,
}

pub fn format_title(message: &str) -> String {
    let truncated = message.chars().take(60).collect::<String>();
    format!("[Feedback] {truncated}")
}

pub fn format_body(req: &FeedbackRequest) -> String {
    let mut body = req.message.clone();
    if let Some(email) = &req.email {
        body.push_str(&format!("\n\nEmail: {email}"));
    }
    if let Some(game_id) = &req.game_id {
        body.push_str(&format!("\n\nGame ID: {game_id}"));
    }
    if let Some(round_id) = &req.round_id {
        body.push_str(&format!("\n\nRound ID: {round_id}"));
    }
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- format_title ---

    #[test]
    fn title_has_feedback_prefix() {
        assert!(format_title("Hello").starts_with("[Feedback] "));
    }

    #[test]
    fn title_includes_short_message_in_full() {
        assert_eq!(format_title("Hello"), "[Feedback] Hello");
    }

    #[test]
    fn title_includes_message_of_exactly_60_chars_in_full() {
        let msg = "A".repeat(60);
        assert_eq!(format_title(&msg), format!("[Feedback] {msg}"));
    }

    #[test]
    fn title_truncates_message_longer_than_60_chars() {
        let msg = "A".repeat(61);
        let expected = format!("[Feedback] {}", "A".repeat(60));
        assert_eq!(format_title(&msg), expected);
    }

    #[test]
    fn title_truncates_long_message_to_60_chars() {
        let msg = "B".repeat(100);
        let title = format_title(&msg);
        assert_eq!(title, format!("[Feedback] {}", "B".repeat(60)));
    }

    // --- format_body ---

    #[test]
    fn body_always_contains_message() {
        let req = FeedbackRequest { message: "great game".into(), email: None, game_id: None, round_id: None };
        assert!(format_body(&req).contains("great game"));
    }

    #[test]
    fn body_includes_email_when_provided() {
        let req = FeedbackRequest { message: "hi".into(), email: Some("user@example.com".into()), game_id: None, round_id: None };
        assert!(format_body(&req).contains("user@example.com"));
    }

    #[test]
    fn body_omits_email_when_absent() {
        let req = FeedbackRequest { message: "hi".into(), email: None, game_id: None, round_id: None };
        assert!(!format_body(&req).contains("email") && !format_body(&req).contains('@'));
    }

    #[test]
    fn body_includes_game_id_when_provided() {
        let id = Uuid::new_v4();
        let req = FeedbackRequest { message: "hi".into(), email: None, game_id: Some(id), round_id: None };
        assert!(format_body(&req).contains(&id.to_string()));
    }

    #[test]
    fn body_omits_game_id_when_absent() {
        let req = FeedbackRequest { message: "hi".into(), email: None, game_id: None, round_id: None };
        assert!(!format_body(&req).contains("game"));
    }

    #[test]
    fn body_includes_round_id_when_provided() {
        let id = Uuid::new_v4();
        let req = FeedbackRequest { message: "hi".into(), email: None, game_id: None, round_id: Some(id) };
        assert!(format_body(&req).contains(&id.to_string()));
    }

    #[test]
    fn body_omits_round_id_when_absent() {
        let req = FeedbackRequest { message: "hi".into(), email: None, game_id: None, round_id: None };
        assert!(!format_body(&req).contains("round"));
    }

    #[test]
    fn body_includes_all_fields_when_all_provided() {
        let game_id  = Uuid::new_v4();
        let round_id = Uuid::new_v4();
        let req = FeedbackRequest {
            message:  "full feedback".into(),
            email:    Some("user@example.com".into()),
            game_id:  Some(game_id),
            round_id: Some(round_id),
        };
        let body = format_body(&req);
        assert!(body.contains("full feedback"));
        assert!(body.contains("user@example.com"));
        assert!(body.contains(&game_id.to_string()));
        assert!(body.contains(&round_id.to_string()));
    }
}
