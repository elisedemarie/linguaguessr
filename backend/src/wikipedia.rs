use async_trait::async_trait;
use common::types::Language;

const MAX_ATTEMPTS: usize = 3;
const MIN_CHARS: usize = 100;
const MAX_CHARS: usize = 600;

#[derive(Debug)]
pub enum FetchError {
    Http(String),
    Parse,
    TooShort,
}

#[async_trait]
pub trait WikipediaClient: Send + Sync {
    async fn fetch_summary(&self, url: &str) -> Result<String, FetchError>;
}

pub struct ReqwestWikipediaClient {
    client: reqwest::Client,
}

impl ReqwestWikipediaClient {
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
    }
}

#[async_trait]
impl WikipediaClient for ReqwestWikipediaClient {
    async fn fetch_summary(&self, url: &str) -> Result<String, FetchError> {
        let response = self.client.get(url).send().await
            .map_err(|e| FetchError::Http(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FetchError::Http(response.status().to_string()));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|_| FetchError::Parse)?;

        json["extract"].as_str()
            .map(|s| s.to_string())
            .ok_or(FetchError::Parse)
    }
}

pub fn wiki_url(lang: &Language) -> String {
    let code = match lang {
        Language::English => "en",
        Language::French => "fr",
        Language::Japanese => "ja",
        Language::Arabic => "ar",
        Language::Russian => "ru",
    };
    format!("https://{}.wikipedia.org/api/rest_v1/page/random/summary", code)
}

pub fn truncate_extract(text: &str) -> String {
    if text.len() <= MAX_CHARS {
        return text.to_string();
    }
    let cut = text[..MAX_CHARS].rfind(' ').unwrap_or(MAX_CHARS);
    text[..cut].to_string()
}

pub async fn fetch_article_from(
    lang: &Language,
    client: &dyn WikipediaClient,
    base_url: &str,
) -> Result<String, FetchError> {
    let url = format!("{}/api/rest_v1/page/random/summary", base_url.trim_end_matches('/'));

    for _ in 0..MAX_ATTEMPTS {
        let extract = client.fetch_summary(&url).await?;
        let truncated = truncate_extract(&extract);
        if truncated.len() >= MIN_CHARS {
            return Ok(truncated);
        }
    }
    Err(FetchError::TooShort)
}

pub async fn fetch_article(lang: &Language, client: &dyn WikipediaClient) -> Result<String, FetchError> {
    fetch_article_from(lang, client, &wiki_url(lang)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use common::types::Language;

    fn summary_json(extract: &str) -> String {
        format!(r#"{{"extract": "{}"}}"#, extract)
    }

    fn long_extract() -> String {
        // 650 chars — longer than MAX_CHARS (600)
        let sentence = "This is a long sentence that takes up space in the extract. ";
        sentence.repeat(11)
    }

    fn adequate_extract() -> &'static str {
        // > MIN_CHARS (100), < MAX_CHARS (600)
        "This is an adequate extract that is long enough to be returned without truncation. It has multiple sentences and is clearly over one hundred characters total."
    }

    fn short_extract() -> &'static str {
        // < MIN_CHARS (100) — triggers retry
        "Too short."
    }

    // --- URL construction ---

    #[test]
    fn english_uses_en_subdomain() {
        assert!(wiki_url(&Language::English).contains("en.wikipedia.org"));
    }

    #[test]
    fn french_uses_fr_subdomain() {
        assert!(wiki_url(&Language::French).contains("fr.wikipedia.org"));
    }

    #[test]
    fn japanese_uses_ja_subdomain() {
        assert!(wiki_url(&Language::Japanese).contains("ja.wikipedia.org"));
    }

    #[test]
    fn arabic_uses_ar_subdomain() {
        assert!(wiki_url(&Language::Arabic).contains("ar.wikipedia.org"));
    }

    #[test]
    fn russian_uses_ru_subdomain() {
        assert!(wiki_url(&Language::Russian).contains("ru.wikipedia.org"));
    }

    // --- truncate_extract ---

    #[test]
    fn short_text_returned_unchanged() {
        assert_eq!(truncate_extract(adequate_extract()), adequate_extract());
    }

    #[test]
    fn long_text_truncated_at_or_below_max_chars() {
        let result = truncate_extract(&long_extract());
        assert!(result.len() <= MAX_CHARS);
    }

    #[test]
    fn long_text_not_cut_mid_word() {
        let result = truncate_extract(&long_extract());
        assert!(!result.ends_with(|c: char| c.is_alphanumeric()));
    }

    #[test]
    fn text_exactly_max_chars_returned_unchanged() {
        let exact = "a".repeat(MAX_CHARS);
        assert_eq!(truncate_extract(&exact).len(), MAX_CHARS);
    }

    #[test]
    fn long_text_preserves_content_up_to_cutoff() {
        let result = truncate_extract(&long_extract());
        assert!(long_extract().starts_with(&result));
    }

    // --- fetch_article_from happy path ---

    #[tokio::test]
    async fn returns_extract_unchanged_when_within_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(adequate_extract())))
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let result = fetch_article_from(&Language::English, &client, &server.uri()).await;
        assert_eq!(result.unwrap(), adequate_extract());
    }

    #[tokio::test]
    async fn returns_truncated_text_when_extract_exceeds_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(&long_extract())))
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let result = fetch_article_from(&Language::English, &client, &server.uri()).await;
        let text = result.unwrap();
        assert!(text.len() <= MAX_CHARS);
    }

    // --- retry on short extract ---

    #[tokio::test]
    async fn retries_on_short_extract_and_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(short_extract())))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(adequate_extract())))
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let result = fetch_article_from(&Language::English, &client, &server.uri()).await;
        assert_eq!(result.unwrap(), adequate_extract());
    }

    #[tokio::test]
    async fn returns_too_short_error_after_three_short_extracts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(short_extract())))
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let result = fetch_article_from(&Language::English, &client, &server.uri()).await;
        assert!(matches!(result, Err(FetchError::TooShort)));
    }

    #[tokio::test]
    async fn makes_exactly_three_attempts_on_repeated_short_extracts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(short_extract())))
            .expect(3)
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let _ = fetch_article_from(&Language::English, &client, &server.uri()).await;
        server.verify().await;
    }

    // --- HTTP errors ---

    #[tokio::test]
    async fn returns_http_error_on_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let result = fetch_article_from(&Language::English, &client, &server.uri()).await;
        assert!(matches!(result, Err(FetchError::Http(_))));
    }

    // --- malformed JSON ---

    #[tokio::test]
    async fn returns_parse_error_on_missing_extract_field() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(r#"{"title": "Something", "no_extract": true}"#))
            .mount(&server)
            .await;

        let client = ReqwestWikipediaClient::new();
        let result = fetch_article_from(&Language::English, &client, &server.uri()).await;
        assert!(matches!(result, Err(FetchError::Parse)));
    }

    // --- fetch_article delegates correctly ---

    #[tokio::test]
    async fn fetch_article_returns_text_from_wiki_url() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string(summary_json(adequate_extract())))
            .mount(&server)
            .await;

        struct RedirectingClient {
            base: String,
            inner: ReqwestWikipediaClient,
        }

        #[async_trait::async_trait]
        impl WikipediaClient for RedirectingClient {
            async fn fetch_summary(&self, _url: &str) -> Result<String, FetchError> {
                self.inner.fetch_summary(&format!(
                    "{}/api/rest_v1/page/random/summary",
                    self.base.trim_end_matches('/')
                )).await
            }
        }

        let client = RedirectingClient {
            base: server.uri(),
            inner: ReqwestWikipediaClient::new(),
        };
        let result = fetch_article(&Language::French, &client).await;
        assert_eq!(result.unwrap(), adequate_extract());
    }
}
