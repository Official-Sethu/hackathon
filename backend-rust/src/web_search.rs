use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[derive(Deserialize)]
struct JinaResponse {
    data: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct JinaResult {
    title: Option<String>,
    url: Option<String>,
    content: Option<String>,
    description: Option<String>,
    snippet: Option<String>,
}

pub struct WebSearcher {
    client: Client,
}

impl WebSearcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(12))
                .user_agent("TraceFactChecker/1.2")
                .build()
                .unwrap_or_default(),
        }
    }

    /// Cleans and extracts a high-precision search query from a raw claim or video title.
    /// Dynamically strips platform noise, system brackets, and generic clickbait filler
    /// without hardcoding specific video titles.
    pub fn clean_query(raw_claim: &str) -> String {
        let mut text = raw_claim.trim().to_string();

        // 1. Strip bracketed system headers like [Viral TikTok Reel Audio...]:
        if let Some(idx) = text.find("]:") {
            text = text[idx + 2..].trim().to_string();
        } else if let Some(idx) = text.find(']') {
            if text.starts_with('[') {
                text = text[idx + 1..].trim().to_string();
            }
        }

        // 2. Remove leading/trailing quotes
        text = text.trim_matches(|c| c == '"' || c == '\'' || c == '`').to_string();

        // 3. Strip social media platform tags dynamically
        let noise_words = [
            "tiktok", "instagram", "facebook", "youtube", "shorts", "reel", "reels",
            "stunning", "shocking", "unbelievable", "viral", "must watch", "leaves spectators", "spectators shocked",
            "watch this", "trending", "omg", "breaking news"
        ];

        let mut cleaned_words: Vec<String> = Vec::new();
        for word in text.split_whitespace() {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            let lower_word = clean_word.to_lowercase();
            if !noise_words.contains(&lower_word.as_str()) && !clean_word.is_empty() {
                cleaned_words.push(clean_word.to_string());
            }
        }

        let cleaned_query = cleaned_words.join(" ");

        if cleaned_query.trim().len() >= 3 {
            cleaned_query
        } else {
            text
        }
    }

    /// Searches the web for real-time evidence about a claim.
    /// Executes a dual-pass search: primary extracted key query, then fallback to broader query if needed.
    pub async fn search_for_claim(&self, claim: &str) -> String {
        let primary_query = Self::clean_query(claim);
        tracing::info!("[WebSearch] Pass 1 — Executing Jina search for primary query: \"{}\"", primary_query);
        let mut results = self.fetch_jina(&primary_query).await;

        // Dual-pass search: if primary query returned no results, retry with the raw stripped claim
        if results.is_empty() {
            let fallback_query = claim.trim_matches(|c| c == '"' || c == '\'' || c == '`').to_string();
            if fallback_query != primary_query && fallback_query.len() >= 3 {
                tracing::info!("[WebSearch] Pass 2 — Fallback search for broader query: \"{}\"", fallback_query);
                results = self.fetch_jina(&fallback_query).await;
            }
        }

        if results.is_empty() {
            tracing::warn!("[WebSearch] Jina AI returned no results for query: \"{}\"", primary_query);
            return String::from("[Web search returned no results. Proceed with training knowledge only and flag uncertainty accordingly.]");
        }

        let mut context = String::from("[REAL-TIME WEB SEARCH RESULTS — use these as primary evidence]\n\n");
        for (i, r) in results.iter().take(5).enumerate() {
            context.push_str(&format!(
                "Result {}:\nTitle: {}\nURL: {}\nSummary: {}\n\n",
                i + 1,
                r.title,
                r.url,
                r.snippet
            ));
        }
        context.push_str("[END OF SEARCH RESULTS — base your verdict on the above evidence first, then your training knowledge]");
        context
    }

    async fn fetch_jina(&self, query: &str) -> Vec<SearchResult> {
        // Jina AI Search — authenticated via API key
        let api_key = std::env::var("JINA_API_KEY")
            .unwrap_or_else(|_| "jina_84df555388c9499ebed7089382a7f7a6o5d1xn5K4fd4AiKIHSNPdRFxXV-7".to_string());

        let encoded_query = query
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("%20");

        let url = format!("https://s.jina.ai/{}", encoded_query);

        let mut req = self
            .client
            .get(&url)
            .header("Accept", "application/json");

        if !api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        match req.send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let mut results = Vec::new();
                        let items = val.get("data")
                            .and_then(|d| d.as_array())
                            .cloned()
                            .unwrap_or_default();

                        for item in items {
                            let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                            let url = item.get("url").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                            let description = item.get("description")
                                .or_else(|| item.get("content"))
                                .or_else(|| item.get("snippet"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .trim()
                                .chars()
                                .take(400)
                                .collect::<String>();

                            if !title.is_empty() && !url.is_empty() {
                                results.push(SearchResult {
                                    title,
                                    url,
                                    snippet: description,
                                });
                            }
                        }

                        if results.is_empty() {
                            tracing::warn!("Jina AI search JSON contained no items, attempting DDG fallback");
                            self.fetch_ddg(query).await
                        } else {
                            results
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Jina AI search JSON parse failed: {}", e);
                        self.fetch_ddg(query).await
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Jina AI search request failed: {}", e);
                self.fetch_ddg(query).await
            }
            Ok(resp) => {
                tracing::warn!("Jina AI search returned HTTP {}", resp.status());
                self.fetch_ddg(query).await
            }
        }
    }

    async fn fetch_ddg(&self, query: &str) -> Vec<SearchResult> {
        // DuckDuckGo Instant Answer API — fallback, no key required
        let encoded = query
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("+");
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
            encoded
        );

        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let mut results = Vec::new();

                        // Abstract (top-level answer)
                        let abstract_text = json["Abstract"].as_str().unwrap_or("").to_string();
                        let abstract_url = json["AbstractURL"].as_str().unwrap_or("").to_string();
                        let abstract_source = json["AbstractSource"].as_str().unwrap_or("DuckDuckGo").to_string();
                        if !abstract_text.is_empty() {
                            results.push(SearchResult {
                                title: abstract_source,
                                url: abstract_url,
                                snippet: abstract_text.chars().take(300).collect(),
                            });
                        }

                        // Related topics
                        if let Some(topics) = json["RelatedTopics"].as_array() {
                            for topic in topics.iter().take(4) {
                                let text = topic["Text"].as_str().unwrap_or("").to_string();
                                let first_url = topic["FirstURL"].as_str().unwrap_or("").to_string();
                                if !text.is_empty() && !first_url.is_empty() {
                                    results.push(SearchResult {
                                        title: format!("Related: {}", &text.chars().take(60).collect::<String>()),
                                        url: first_url,
                                        snippet: text.chars().take(300).collect(),
                                    });
                                }
                            }
                        }
                        results
                    }
                    Err(e) => {
                        tracing::warn!("DuckDuckGo JSON parse failed: {}", e);
                        Vec::new()
                    }
                }
            }
            _ => Vec::new(),
        }
    }
}
