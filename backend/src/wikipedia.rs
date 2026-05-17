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
        let client = reqwest::Client::builder()
            .user_agent("linguaguessr/0.1 (https://github.com/elisedemarie/linguaguessr)")
            .build()
            .expect("failed to build HTTP client");
        Self { client }
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

pub fn wiki_base_url(lang: &Language) -> String {
    let code = match lang {
        Language::English    => "en",
        Language::French     => "fr",
        Language::Japanese   => "ja",
        Language::Arabic     => "ar",
        Language::Russian    => "ru",
        Language::Spanish    => "es",
        Language::Chinese    => "zh",
        Language::Hindi      => "hi",
        Language::Bengali    => "bn",
        Language::Portuguese => "pt",
        Language::Indonesian => "id",
        Language::Urdu       => "ur",
        Language::German     => "de",
        Language::Korean     => "ko",
        Language::Vietnamese => "vi",
        Language::Telugu     => "te",
        Language::Marathi    => "mr",
        Language::Tamil      => "ta",
        Language::Turkish    => "tr",
        Language::Persian    => "fa",
        Language::Italian    => "it",
        Language::Thai       => "th",
        Language::Swahili    => "sw",
        Language::Polish     => "pl",
        Language::Ukrainian  => "uk",
        Language::Dutch      => "nl",
        Language::Greek      => "el",
        Language::Romanian   => "ro",
        Language::Czech      => "cs",
        Language::Hungarian  => "hu",
    };
    format!("https://{}.wikipedia.org", code)
}

pub fn truncate_extract(text: &str) -> String {
    if text.chars().count() <= MAX_CHARS {
        return text.to_string();
    }
    // Find the byte position of the MAX_CHARS-th character — always a valid boundary
    let byte_limit = text.char_indices()
        .nth(MAX_CHARS)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    // Try to cut at the last space to avoid splitting mid-word in Latin scripts
    let cut = text[..byte_limit].rfind(' ').unwrap_or(byte_limit);
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
    fetch_article_from(lang, client, &wiki_base_url(lang)).await
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

    #[test] fn english_url()    { assert_eq!(wiki_base_url(&Language::English),    "https://en.wikipedia.org"); }
    #[test] fn french_url()     { assert_eq!(wiki_base_url(&Language::French),     "https://fr.wikipedia.org"); }
    #[test] fn japanese_url()   { assert_eq!(wiki_base_url(&Language::Japanese),   "https://ja.wikipedia.org"); }
    #[test] fn arabic_url()     { assert_eq!(wiki_base_url(&Language::Arabic),     "https://ar.wikipedia.org"); }
    #[test] fn russian_url()    { assert_eq!(wiki_base_url(&Language::Russian),    "https://ru.wikipedia.org"); }
    #[test] fn spanish_url()    { assert_eq!(wiki_base_url(&Language::Spanish),    "https://es.wikipedia.org"); }
    #[test] fn chinese_url()    { assert_eq!(wiki_base_url(&Language::Chinese),    "https://zh.wikipedia.org"); }
    #[test] fn hindi_url()      { assert_eq!(wiki_base_url(&Language::Hindi),      "https://hi.wikipedia.org"); }
    #[test] fn bengali_url()    { assert_eq!(wiki_base_url(&Language::Bengali),    "https://bn.wikipedia.org"); }
    #[test] fn portuguese_url() { assert_eq!(wiki_base_url(&Language::Portuguese), "https://pt.wikipedia.org"); }
    #[test] fn indonesian_url() { assert_eq!(wiki_base_url(&Language::Indonesian), "https://id.wikipedia.org"); }
    #[test] fn urdu_url()       { assert_eq!(wiki_base_url(&Language::Urdu),       "https://ur.wikipedia.org"); }
    #[test] fn german_url()     { assert_eq!(wiki_base_url(&Language::German),     "https://de.wikipedia.org"); }
    #[test] fn korean_url()     { assert_eq!(wiki_base_url(&Language::Korean),     "https://ko.wikipedia.org"); }
    #[test] fn vietnamese_url() { assert_eq!(wiki_base_url(&Language::Vietnamese), "https://vi.wikipedia.org"); }
    #[test] fn telugu_url()     { assert_eq!(wiki_base_url(&Language::Telugu),     "https://te.wikipedia.org"); }
    #[test] fn marathi_url()    { assert_eq!(wiki_base_url(&Language::Marathi),    "https://mr.wikipedia.org"); }
    #[test] fn tamil_url()      { assert_eq!(wiki_base_url(&Language::Tamil),      "https://ta.wikipedia.org"); }
    #[test] fn turkish_url()    { assert_eq!(wiki_base_url(&Language::Turkish),    "https://tr.wikipedia.org"); }
    #[test] fn persian_url()    { assert_eq!(wiki_base_url(&Language::Persian),    "https://fa.wikipedia.org"); }
    #[test] fn italian_url()    { assert_eq!(wiki_base_url(&Language::Italian),    "https://it.wikipedia.org"); }
    #[test] fn thai_url()       { assert_eq!(wiki_base_url(&Language::Thai),       "https://th.wikipedia.org"); }
    #[test] fn swahili_url()    { assert_eq!(wiki_base_url(&Language::Swahili),    "https://sw.wikipedia.org"); }
    #[test] fn polish_url()     { assert_eq!(wiki_base_url(&Language::Polish),     "https://pl.wikipedia.org"); }
    #[test] fn ukrainian_url()  { assert_eq!(wiki_base_url(&Language::Ukrainian),  "https://uk.wikipedia.org"); }
    #[test] fn dutch_url()      { assert_eq!(wiki_base_url(&Language::Dutch),      "https://nl.wikipedia.org"); }
    #[test] fn greek_url()      { assert_eq!(wiki_base_url(&Language::Greek),      "https://el.wikipedia.org"); }
    #[test] fn romanian_url()   { assert_eq!(wiki_base_url(&Language::Romanian),   "https://ro.wikipedia.org"); }
    #[test] fn czech_url()      { assert_eq!(wiki_base_url(&Language::Czech),      "https://cs.wikipedia.org"); }
    #[test] fn hungarian_url()  { assert_eq!(wiki_base_url(&Language::Hungarian),  "https://hu.wikipedia.org"); }

    fn long_multibyte_extract() -> String {
        // 599 ASCII bytes then Thai chars — byte 600 lands inside a 3-byte Thai char, triggering the panic
        "a".repeat(599) + &"กขคง".repeat(50)
    }

    fn long_arabic_extract() -> String {
        // 599 ASCII bytes then Arabic chars — byte 600 lands inside a 2-byte Arabic char
        "a".repeat(599) + &"مرحبا بالعالم ".repeat(20)
    }

    // --- truncate_extract ---

    #[test]
    fn short_text_returned_unchanged() {
        assert_eq!(truncate_extract(adequate_extract()), adequate_extract());
    }

    #[test]
    fn long_ascii_text_truncated_within_char_limit() {
        let result = truncate_extract(&long_extract());
        assert!(result.chars().count() <= MAX_CHARS);
    }

    #[test]
    fn long_text_not_cut_mid_word() {
        let result = truncate_extract(&long_extract());
        assert!(!result.ends_with(|c: char| c.is_alphanumeric()));
    }

    #[test]
    fn multibyte_thai_text_does_not_panic() {
        let result = truncate_extract(&long_multibyte_extract());
        assert!(result.chars().count() <= MAX_CHARS);
    }

    #[test]
    fn multibyte_thai_text_result_is_valid_utf8() {
        let result = truncate_extract(&long_multibyte_extract());
        assert!(std::str::from_utf8(result.as_bytes()).is_ok());
    }

    #[test]
    fn multibyte_arabic_text_does_not_panic() {
        let result = truncate_extract(&long_arabic_extract());
        assert!(result.chars().count() <= MAX_CHARS);
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
