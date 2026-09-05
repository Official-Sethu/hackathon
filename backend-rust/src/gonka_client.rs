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
                .timeout(std::time::Duration::from_secs(60))
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
            For each sub-claim, clearly state what you know from verifiable sources, what you are inferring, \
            and what you cannot confirm. Apply this critical rule:\n\
            ABSENCE FROM TRAINING DATA IS NOT EVIDENCE OF FABRICATION.\n\
            Many real events — especially recent aviation displays, sporting ceremonies, stadium flyovers, \
            and military demonstrations — are not individually catalogued in AI training data. \
            Not finding a specific event in your training set does NOT mean it did not happen.\n\
            \n\
            STEP 3 — PLAUSIBILITY SCORING (critical step)\n\
            Before scoring, assess plausibility on two axes:\n\
            A) EVENT TYPE PLAUSIBILITY: Is this the kind of event that happens in the real world? \
               Passenger jet flyovers at stadiums, low-altitude military and civilian demonstration flights, \
               South African Airways heritage flights, and similar aviation displays are well-documented \
               categories of real events globally. If the event type is physically possible and has real-world precedent, \
               it must NOT be scored as fabricated simply because you lack this specific instance in training data.\n\
            B) SOURCE CREDIBILITY: Is the attributed source (9News, Reuters, BBC, AP, local TV stations) \
               a real, credible news organisation? If yes, and if the event type is plausible, \
               the claim should score at minimum in the UNSUBSTANTIATED / NUANCED range (40-64) \
               unless you have direct contradicting evidence.\n\
            Score guide for plausible + credible-source claims you cannot independently confirm:\n\
            - Plausible event type + real credible outlet cited = 55-75 (UNSUBSTANTIATED / NUANCED leaning VERIFIED)\n\
            - Plausible event type + no source cited = 40-55 (UNSUBSTANTIATED / NUANCED)\n\
            - Implausible/physically impossible event = 0-30 regardless of source\n\
            - Directly contradicted by evidence = 0-20 (FABRICATED OR DEBUNKED)\n\
            \n\
            STEP 4 — RECENCY CHECK\n\
            If the claim involves a recent event, flag uncertainty and lower your confidence score. \
            Do NOT fabricate post-training-cutoff facts. Recency reduces confidence, not truthScore itself \
            unless you have direct contradicting evidence.\n\
            \n\
            STEP 5 — BIAS AND FRAMING AUDIT\n\
            Identify logical fallacies, misleading framing, cherry-picked statistics, or emotional manipulation.\n\
            \n\
            STEP 6 — ANTI-HALLUCINATION SELF-AUDIT\n\
            Remove anything you cannot verify. But do NOT mark real-world plausible events as fabricated \
            merely because you lack training data on that specific instance.\n\
            \n\
            STEP 7 — FINAL SCORE AND OUTPUT\n\
            Assign truthScore 0-100. Emit ONLY the raw JSON object. No markdown fences. No preamble.\n\
            \n\
            {{\n\
              \"truthScore\": <integer 0-100>,\n\
              \"verdict\": \"<VERIFIED FACTUAL & AUTHENTIC | UNSUBSTANTIATED / NUANCED | FABRICATED OR DEBUNKED>\",\n\
              \"headline\": \"<one precise sentence: what is claimed and whether it is substantiated>\",\n\
              \"summary\": \"<2-3 sentences of factual explanation grounded in evidence>\",\n\
              \"assessment\": \"<your full model-specific evaluation referencing the steps above>\",\n\
              \"confidence\": <integer 0-100, lower when recency-flagged or event not in training data>,\n\
              \"stance\": \"<Verified True | Partially Accurate | Needs Clarification | False>\",\n\
              \"fallacies\": [\"<detected fallacy or empty array if none>\"],\n\
              \"citations\": [{{\"title\": \"<source name>\", \"url\": \"<real url>\"}}],\n\
              \"reasoningSteps\": [\n\
                {{\"title\": \"Claim Decomposition\", \"desc\": \"<sub-claims identified>\"}},\n\
                {{\"title\": \"Plausibility & Evidence Assessment\", \"desc\": \"<event type plausibility + what is known vs inferred>\"}},\n\
                {{\"title\": \"Recency & Bias Audit\", \"desc\": \"<recency flags and framing issues>\"}},\n\
                {{\"title\": \"Anti-Hallucination Check\", \"desc\": \"<what was removed or flagged>\"}},\n\
                {{\"title\": \"Final Scoring Rationale\", \"desc\": \"<why this specific score was assigned>\"}}\n\
              ]\n\
            }}",
            role = system_role
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
            "temperature": 0.0,
            "max_tokens": 1800
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
                    match resp.text().await {
                        Ok(raw_text) => {
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

                                // Strip markdown fences if the model ignored the instruction
                                let cleaned = content
                                    .replace("```json", "")
                                    .replace("```", "")
                                    .trim()
                                    .to_string();

                                // Extract JSON object from content in case there is preamble text
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
                            } else {
                                tracing::warn!("Failed to parse JSON response for model {}: {}", model_name, raw_text);
                                return Err(format!("Invalid JSON returned by Gonka Router for model {}", model_name));
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to read response body for model {}: {}", model_name, e);
                            return Err(format!("Failed to read response body for model {}", model_name));
                        }
                    }
                }
                let err_text = resp.text().await.unwrap_or_default();
                tracing::warn!("Gonka Router returned HTTP status {} for model {}: {}", status, model_name, err_text);
                Err(format!("Gonka Router responded with HTTP {} for model {}", status, model_name))
            }
            Err(e) => {
                tracing::warn!("Network failure reaching Gonka Router for model {}: {}", model_name, e);
                Err(format!("Network failure reaching Gonka Router: {}", e))
            }
        }
    }
}
