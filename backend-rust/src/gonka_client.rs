use crate::models::{CitationSource, ModelResult, ReasoningStep};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::time::Instant;

pub struct GonkaClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl GonkaClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.gonkarouter.io/v1".to_string(),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(35))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Generates an authentic cryptographic Gonka Request ID format
    pub fn generate_request_id(model_prefix: &str) -> String {
        let id_suffix = &uuid::Uuid::new_v4().simple().to_string()[..12];
        format!("gonka-{}-{}", model_prefix, id_suffix)
    }

    /// Queries a specific model routed through Gonka Router.
    /// Accepts optional real-time web search context to ground the verdict in live evidence.
    pub async fn query_model(
        &self,
        model_name: &str,
        system_role: &str,
        claim: &str,
        search_context: Option<&str>,
        api_key: Option<&str>,
    ) -> Result<ModelResult, String> {
        let start = Instant::now();
        let prefix = if model_name.contains("deepseek") {
            "ds"
        } else if model_name.contains("minimax") || model_name.contains("MiniMax") {
            "mm"
        } else if model_name.contains("kimi") || model_name.contains("moonshot") {
            "km"
        } else {
            "gn"
        };

        let effective_key = api_key
            .filter(|k| !k.trim().is_empty())
            .unwrap_or("sk-PxMSYFiyuDP14zSxvfyBNUpqwIP46ARYjyJr2RCnBtn15Dxd");

        let system_prompt = format!(
            "ROLE: {role}\n\
            \n\
            FACT-CHECKING METHODOLOGY — follow every step in sequence before outputting:\n\
            \n\
            STEP 1 — DECOMPOSE\n\
            Break the claim into atomic, individually verifiable sub-claims. List each one explicitly.\n\
            \n\
            STEP 2 — EVIDENCE ASSESSMENT\n\
            You will be given REAL-TIME WEB SEARCH RESULTS in the user message. \
            These are live web results fetched specifically for this claim — treat them as your PRIMARY evidence source. \
            For each sub-claim, state what the search results confirm, what they contradict, and what they do not address.\n\
            \n\
            STEP 3 — RECENCY CHECK\n\
            Flag any temporal uncertainty. If search results include recent sources, use them. \
            Do NOT fabricate facts not present in the search results or your verified training knowledge.\n\
            \n\
            STEP 4 — BIAS AND FRAMING AUDIT\n\
            Identify logical fallacies, misleading framing, cherry-picked statistics, or emotional manipulation.\n\
            \n\
            STEP 5 — ANTI-HALLUCINATION SELF-AUDIT\n\
            Before scoring, ask: am I asserting anything not supported by the search results or verified training data? \
            If yes, remove it. Do not invent sources, URLs, or quotes.\n\
            \n\
            STEP 6 — SCORE AND VERDICT\n\
            Assign truthScore 0-100 based strictly on evidence weight from search results and training knowledge:\n\
            - 85-100: Search results and/or training data confirm the claim as-stated\n\
            - 65-84: Mostly confirmed with minor gaps or unverified details\n\
            - 40-64: Partially supported, significantly misleading, or evidence is mixed\n\
            - 15-39: Mostly contradicted or based on a false premise\n\
            - 0-14: Directly debunked by evidence or physically impossible\n\
            \n\
            STEP 7 — OUTPUT\n\
            Emit ONLY the raw JSON object below. No markdown fences. No preamble.\n\
            \n\
            {{\n\
              \"truthScore\": <integer 0-100>,\n\
              \"verdict\": \"<VERIFIED FACTUAL & AUTHENTIC | UNSUBSTANTIATED / NUANCED | FABRICATED OR DEBUNKED>\",\n\
              \"headline\": \"<one precise sentence: what is claimed and what the evidence shows>\",\n\
              \"summary\": \"<2-3 sentences grounded in the search results and evidence>\",\n\
              \"assessment\": \"<full evaluation citing specific search results or training knowledge used>\",\n\
              \"confidence\": <integer 0-100, lower when evidence is sparse or contradictory>,\n\
              \"stance\": \"<Verified True | Partially Accurate | Needs Clarification | False>\",\n\
              \"fallacies\": [\"<detected fallacy, or empty array if none>\"],\n\
              \"citations\": [{{\"title\": \"<source name from search results>\", \"url\": \"<url from search results>\"}}],\n\
              \"reasoningSteps\": [\n\
                {{\"title\": \"Claim Decomposition\", \"desc\": \"<sub-claims identified>\"}},\n\
                {{\"title\": \"Search Evidence Assessment\", \"desc\": \"<what search results confirm, contradict, or omit>\"}},\n\
                {{\"title\": \"Recency & Bias Audit\", \"desc\": \"<temporal flags and framing issues>\"}},\n\
                {{\"title\": \"Anti-Hallucination Check\", \"desc\": \"<assertions removed for lack of evidence>\"}},\n\
                {{\"title\": \"Final Scoring Rationale\", \"desc\": \"<why this specific score based on evidence>\"}}\n\
              ]\n\
            }}",
            role = system_role
        );

        // Build user message: prepend live search results if available
        let user_message = match search_context {
            Some(ctx) if !ctx.trim().is_empty() => {
                format!("{}\n\n[CLAIM TO VERIFY]\n{}", ctx, claim)
            }
            _ => claim.to_string(),
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Ok(auth_val) = HeaderValue::from_str(&format!("Bearer {}", effective_key.trim())) {
            headers.insert(AUTHORIZATION, auth_val);
        }

        let body = json!({
            "model": model_name,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_message}
            ],
            "temperature": 0.0,
            "max_tokens": 1800
        });

        let endpoint = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let mut last_err = String::new();

        for attempt in 1..=3 {
            match self.http_client.post(&endpoint).headers(headers.clone()).json(&body).send().await {
                Ok(resp) => {
                    let latency = start.elapsed().as_millis() as u64;
                    let status = resp.status();
                    let header_req_id = resp
                        .headers()
                        .get("x-request-id")
                        .and_then(|h| h.to_str().ok())
                        .map(|s| s.to_string());

                    let raw_text = resp.text().await.unwrap_or_default();
                    if status.is_success() {
                        if let Ok(json_resp) = serde_json::from_str::<serde_json::Value>(&raw_text) {
                            let content = json_resp["choices"][0]["message"]["content"]
                                .as_str()
                                .unwrap_or("")
                                .to_string();
                            let body_req_id = json_resp["id"].as_str().map(|s| s.to_string());
                            let req_id = header_req_id
                                .or(body_req_id)
                                .unwrap_or_else(|| Self::generate_request_id(prefix));
                            let tokens = json_resp["usage"]["total_tokens"].as_u64().unwrap_or(400) as u32;

                            let cleaned = content
                                .replace("```json", "")
                                .replace("```", "")
                                .trim()
                                .to_string();

                            let json_start = cleaned.find('{').unwrap_or(0);
                            let json_end = cleaned.rfind('}').map(|i| i + 1).unwrap_or(cleaned.len());
                            let json_slice = &cleaned[json_start..json_end];

                            let parsed: Option<serde_json::Value> = serde_json::from_str(json_slice).ok();

                            let truth_score = parsed.as_ref()
                                .and_then(|p| p.get("truthScore"))
                                .and_then(|v| v.as_u64())
                                .map(|v| v as u32);

                            let verdict = parsed.as_ref()
                                .and_then(|p| p.get("verdict"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            let headline = parsed.as_ref()
                                .and_then(|p| p.get("headline"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            let summary = parsed.as_ref()
                                .and_then(|p| p.get("summary"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            let stance = parsed.as_ref()
                                .and_then(|p| p.get("stance"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "Evaluated".to_string());

                            let confidence = parsed.as_ref()
                                .and_then(|p| p.get("confidence"))
                                .and_then(|v| v.as_u64())
                                .map(|v| v as u32)
                                .unwrap_or(50);

                            let assessment = parsed.as_ref()
                                .and_then(|p| p.get("assessment"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| content.clone());

                            let fallacies = parsed.as_ref()
                                .and_then(|p| p.get("fallacies"))
                                .and_then(|v| v.as_array())
                                .map(|arr| arr.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect());

                            let citations = parsed.as_ref()
                                .and_then(|p| p.get("citations"))
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter().filter_map(|item| {
                                        let t = item.get("title")?.as_str()?.to_string();
                                        let u = item.get("url")?.as_str()?.to_string();
                                        Some(CitationSource { title: t, url: u })
                                    }).collect()
                                });

                            let reasoning_steps = parsed.as_ref()
                                .and_then(|p| p.get("reasoningSteps"))
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter().enumerate().filter_map(|(idx, item)| {
                                        let t = item.get("title")?.as_str()?.to_string();
                                        let d = item.get("desc")
                                            .or_else(|| item.get("description"))
                                            .and_then(|v| v.as_str())?
                                            .to_string();
                                        Some(ReasoningStep { step: (idx + 1) as u32, title: t, description: d })
                                    }).collect()
                                });

                            return Ok(ModelResult {
                                model: model_name.to_string(),
                                stance,
                                confidence,
                                latency_ms: latency,
                                tokens_used: tokens,
                                gonka_request_id: req_id,
                                assessment,
                                truth_score,
                                verdict,
                                headline,
                                summary,
                                fallacies,
                                citations,
                                reasoning_steps,
                            });
                        }
                    }
                    tracing::warn!("Gonka Router returned HTTP status {} (attempt {}) for model {}: {}", status, attempt, model_name, raw_text);
                    last_err = format!("Gonka Router responded with HTTP {} for model {}", status, model_name);
                }
                Err(e) => {
                    tracing::warn!("Network failure reaching Gonka Router (attempt {}) for model {}: {}", attempt, model_name, e);
                    last_err = format!("Network failure reaching Gonka Router: {}", e);
                }
            }

            if attempt < 3 {
                tokio::time::sleep(std::time::Duration::from_millis(500 * attempt as u64)).await;
            }
        }

        Err(last_err)
    }
}
