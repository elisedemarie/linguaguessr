use async_trait::async_trait;

#[derive(Debug)]
pub enum GitHubError {
    Http(String),
}

#[async_trait]
pub trait GitHubClient: Send + Sync {
    async fn create_issue(&self, title: &str, body: &str) -> Result<(), GitHubError>;
}

pub struct ReqwestGitHubClient {
    client:   reqwest::Client,
    token:    String,
    base_url: String,
}

impl ReqwestGitHubClient {
    pub fn new(token: String) -> Self {
        Self::with_base_url(token, "https://api.github.com".to_string())
    }

    pub fn with_base_url(token: String, base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("linguaguessr-backend")
            .build()
            .expect("failed to build HTTP client");
        Self { client, token, base_url }
    }
}

#[async_trait]
impl GitHubClient for ReqwestGitHubClient {
    async fn create_issue(&self, title: &str, body: &str) -> Result<(), GitHubError> {
        let url = format!("{}/repos/elisedemarie/linguaguessr/issues", self.base_url);
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({
                "title":  title,
                "body":   body,
                "labels": ["feedback"],
            }))
            .send()
            .await
            .map_err(|e| GitHubError::Http(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(GitHubError::Http(response.status().to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn create_issue_posts_to_github_issues_endpoint() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repos/elisedemarie/linguaguessr/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_string(r#"{"number": 1}"#))
            .expect(1)
            .mount(&server)
            .await;

        let client = ReqwestGitHubClient::with_base_url("token123".into(), server.uri());
        client.create_issue("Test title", "Test body").await.unwrap();
        server.verify().await;
    }

    #[tokio::test]
    async fn create_issue_sends_bearer_auth_header() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(header("authorization", "Bearer mytoken"))
            .respond_with(ResponseTemplate::new(201).set_body_string(r#"{"number": 1}"#))
            .expect(1)
            .mount(&server)
            .await;

        let client = ReqwestGitHubClient::with_base_url("mytoken".into(), server.uri());
        client.create_issue("Title", "Body").await.unwrap();
        server.verify().await;
    }

    #[tokio::test]
    async fn create_issue_sends_correct_json_payload() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_json(serde_json::json!({
                "title":  "My title",
                "body":   "My body",
                "labels": ["feedback"]
            })))
            .respond_with(ResponseTemplate::new(201).set_body_string(r#"{"number": 1}"#))
            .expect(1)
            .mount(&server)
            .await;

        let client = ReqwestGitHubClient::with_base_url("token".into(), server.uri());
        client.create_issue("My title", "My body").await.unwrap();
        server.verify().await;
    }

    #[tokio::test]
    async fn create_issue_returns_ok_on_201() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(201).set_body_string(r#"{"number": 1}"#))
            .mount(&server)
            .await;

        let client = ReqwestGitHubClient::with_base_url("token".into(), server.uri());
        assert!(client.create_issue("Title", "Body").await.is_ok());
    }

    #[tokio::test]
    async fn create_issue_returns_err_on_non_2xx() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = ReqwestGitHubClient::with_base_url("token".into(), server.uri());
        assert!(client.create_issue("Title", "Body").await.is_err());
    }
}
