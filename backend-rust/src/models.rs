use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyRequest {
    pub claim: String,
    pub is_url: Option<bool>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyVideoRequest {
    pub video_url: String,
    pub spoken_transcript: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelResult {
    pub model: String,
    pub stance: String,
    pub confidence: u32,
    pub latency_ms: u64,
    pub tokens_used: u32,
    pub gonka_request_id: String,
    pub assessment: String,
    pub truth_score: Option<u32>,
    pub verdict: Option<String>,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub fallacies: Option<Vec<String>>,
    pub citations: Option<Vec<CitationSource>>,
    pub reasoning_steps: Option<Vec<ReasoningStep>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReasoningStep {
    pub step: u32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResponse {
    pub claim: String,
    pub truth_score: u32,
    pub verdict_label: String,
    pub headline: String,
    pub summary: String,
    pub factuality_index: u32,
    pub consensus_alignment: u32,
    pub fallacy_risk: String,
    /// True when model truth scores diverge by more than 25 points — signals genuine ambiguity
    pub dissent_flag: Option<bool>,
    pub is_video: Option<bool>,
    pub video_platform: Option<String>,
    pub spoken_transcript: Option<String>,
    pub models: Vec<ModelResult>,
    pub reasoning_trace: Vec<ReasoningStep>,
    pub fallacies: Vec<String>,
    pub citations: Vec<CitationSource>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub gonka_router_target: String,
    pub version: String,
}
