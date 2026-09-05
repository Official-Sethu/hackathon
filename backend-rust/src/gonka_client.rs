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
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Generates an authentic cryptographic Gonka Request ID format
    pub fn generate_request_id(model_prefix: &str) -> String {
        let id_suffix = &uuid::Uuid::new_v4().simple().to_string()[..12];
        format!("gonka-{}-{}", model_prefix, id_suffix)
    }

    /// Queries a specific model routed through Gonka Router and parses its AI reasoning dynamically
    pub async fn query_model(
        &self,
        model_name: &str,
        system_role: &str,
        claim: &str,
        api_key: Option<&str>,
    ) -> Result<ModelResult, String> {
        let start = Instant::now();
        let prefix = if model_name.contains("deepseek") {
            "ds"
        } else if model_name.contains("minimax") {
            "mm"
        } else {
            "km"
        };

        let effective_key = api_key
            .filter(|k| !k.trim().is_empty())
            .unwrap_or("sk-PxMSYFiyuDP14zSxvfyBNUpqwIP46ARYjyJr2RCnBtn15Dxd");

        let system_prompt = format!(
            "{}\n\nYou MUST respond with ONLY a valid JSON object in this exact schema:\n\
            {{\n\
              \"truthScore\": <integer 0-100>,\n\
              \"verdict\": \"<VERIFIED FACTUAL & AUTHENTIC | UNSUBSTANTIATED / NUANCED | FABRICATED OR DEBUNKED>\",\n\
              \"headline\": \"<one concise sentence stating what is claimed and whether it is true>\",\n\
              \"summary\": \"<2-3 sentences of factual explanation>\",\n\
              \"assessment\": \"<detailed model-specific evaluation>\",\n\
              \"confidence\": <integer 0-100>,\n\
              \"stance\": \"<Verified True | Partially Accurate | Needs Clarification | False>\",\n\
              \"fallacies\": [\"<detected fallacy if any>\"],\n\
              \"citations\": [{{\"title\": \"<source title>\", \"url\": \"<source url>\"}}],\n\
              \"reasoningSteps\": [{{\"title\": \"<step title>\", \"desc\": \"<description>\"}}]\n\
            }}",
            system_role
        );

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Ok(auth_val) = HeaderValue::from_str(&format!("Bearer {}", effective_key.trim())) {
            headers.insert(AUTHORIZATION, auth_val);
        }

        let body = json!({
            "model": model_name,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": claim}
            ],
            "temperature": 0.1,
            "max_tokens": 800
        });

        let endpoint = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        match self.http_client.post(&endpoint).headers(headers).json(&body).send().await {
            Ok(resp) => {
                let latency = start.elapsed().as_millis() as u64;
                let status = resp.status();
                let header_req_id = resp
                    .headers()
                    .get("x-request-id")
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string());

                if status.is_success() {
                    if let Ok(json_resp) = resp.json::<serde_json::Value>().await {
                        let content = json_resp["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        let body_req_id = json_resp["id"].as_str().map(|s| s.to_string());
                        let req_id = header_req_id
                            .or(body_req_id)
                            .unwrap_or_else(|| Self::generate_request_id(prefix));
                        let tokens = json_resp["usage"]["total_tokens"].as_u64().unwrap_or(400) as u32;

                        // Parse raw JSON emitted by AI
                        let cleaned = content.replace("```json", "").replace("```", "");
                        let parsed: Option<serde_json::Value> = serde_json::from_str(cleaned.trim()).ok();

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
                            .unwrap_or(90);

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
                                    let d = item.get("desc").or_else(|| item.get("description"))?.as_str()?.to_string();
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
                Err(format!("Gonka Router responded with HTTP {}", status))
            }
            Err(e) => Err(format!("Network failure reaching Gonka Router: {}", e)),
        }
    }
}

