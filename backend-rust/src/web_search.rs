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
    data: Option<Vec<JinaResult>>,
}

#[derive(Deserialize)]
struct JinaResult {
    title: Option<String>,
    url: Option<String>,
    content: Option<String>,
    description: Option<String>,
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

    /// Searches the web for real-time evidence about a claim.
    /// Returns the top results formatted as a context string for injection into the AI prompt.
    pub async fn search_for_claim(&self, claim: &str) -> String {
        let results = self.fetch_jina(claim).await;

        if results.is_empty() {
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
        // Jina AI Search — free, no API key, returns top web results as JSON
        let encoded_query = query
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("+");

        let url = format!("https://s.jina.ai/?q={}", encoded_query);

        match self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Respond-With", "no-content")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<JinaResponse>().await {
                    Ok(jina) => jina
                        .data
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|r| {
                            let title = r.title.filter(|t| !t.trim().is_empty())?;
                            let url = r.url.filter(|u| !u.trim().is_empty())?;
                            let snippet = r
                                .content
                                .or(r.description)
                                .unwrap_or_default()
                                .chars()
                                .take(300)
                                .collect::<String>();
                            Some(SearchResult { title, url, snippet })
                        })
                        .collect(),
                    Err(e) => {
                        tracing::warn!("Jina AI search JSON parse failed: {}", e);
                        // Fallback: DuckDuckGo Instant Answer
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
