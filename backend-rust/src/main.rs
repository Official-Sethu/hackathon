mod consensus;
mod gonka_client;
mod models;
mod rate_limiter;
mod video_parser;

use axum::{
    extract::{ConnectInfo, Json, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use consensus::ConsensusEngine;
use gonka_client::GonkaClient;
use models::{HealthResponse, VerifyRequest, VerifyVideoRequest};
use video_parser::VideoParser;
use rate_limiter::RateLimiter;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

struct AppState {
    gonka_client: GonkaClient,
    rate_limiter: RateLimiter,
    key_pool: Vec<String>,
}

#[tokio::main]
async fn main() {
    // 1. Initialize tracing
    tracing_subscriber::fmt::init();

    // 2. Load environment variables securely from .env
    dotenvy::dotenv().ok();

    let max_req = std::env::var("RATE_LIMIT_MAX_REQUESTS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10);
    let window_secs = std::env::var("RATE_LIMIT_WINDOW_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60);

    // Load API Keys securely from server environment variables
    let mut key_pool = Vec::new();
    if let Ok(k1) = std::env::var("GONKA_API_KEY_1") {
        if !k1.trim().is_empty() { key_pool.push(k1.trim().to_string()); }
    }
    if let Ok(k2) = std::env::var("GONKA_API_KEY_2") {
        if !k2.trim().is_empty() { key_pool.push(k2.trim().to_string()); }
    }
    if let Ok(k3) = std::env::var("GONKA_API_KEY_3") {
        if !k3.trim().is_empty() { key_pool.push(k3.trim().to_string()); }
    }

    info!("Initialized secure key pool with {} Gonka API keys", key_pool.len());
    info!("Rate limiting active: max {} requests per {}s per client", max_req, window_secs);

    let state = Arc::new(AppState {
        gonka_client: GonkaClient::new(),
        rate_limiter: RateLimiter::new(max_req, window_secs),
        key_pool,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/verify", post(verify_claim_handler))
        .route("/api/verify_video", post(verify_video_handler))
        .route("/api/models", get(models_handler))
        .route("/404", get(error_404_handler))
        .route("/429", get(error_429_handler))
        .route("/500", get(error_500_handler))
        .fallback(error_404_handler)
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    info!("Trace Axum Backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn health_handler() -> impl IntoResponse {
    Json(HealthResponse {
        status: "operational".to_string(),
        service: "Trace Axum Multi-Model Truth Engine".to_string(),
        gonka_router_target: "https://gonkarouter.io/v1".to_string(),
        version: "1.1.0 (Rate-Limited & Secure)".to_string(),
    })
}

async fn models_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "models": [
            { "id": "deepseek-ai/DeepSeek-V4-Flash-0731", "role": "Causal & Formal Logic Reasoning", "routed_via": "gonkarouter.io" },
            { "id": "MiniMaxAI/MiniMax-M2.7", "role": "Contextual & Source Evidence Examination", "routed_via": "gonkarouter.io" },
            { "id": "deepseek-ai/DeepSeek-V4-Flash-0731", "role": "Anti-Hallucination & Factual Registry Cross-Check", "routed_via": "gonkarouter.io" }
        ],
        "router": "https://gonkarouter.io/v1",
        "security": "Server-side key rotation & rate limiting enabled"
    }))
}

async fn error_404_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("../404.html"),
    )
}

async fn error_429_handler() -> impl IntoResponse {
    (
        StatusCode::TOO_MANY_REQUESTS,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("../429.html"),
    )
}

async fn error_500_handler() -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        include_str!("../500.html"),
    )
}

async fn verify_claim_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyRequest>,
) -> Result<impl IntoResponse, (StatusCode, HeaderMap, Json<serde_json::Value>)> {
    // 1. Identify client IP for rate limiting
    let client_ip = headers
        .get("x-forwarded-for")
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    // 2. Enforce server-side sliding window rate limit
    if let Err((retry_after, err_msg)) = state.rate_limiter.check(&client_ip) {
        warn!("Rate limit tripped for client {}: {}", client_ip, err_msg);
        let mut resp_headers = HeaderMap::new();
        resp_headers.insert(
            header::RETRY_AFTER,
            HeaderValue::from_str(&retry_after.to_string()).unwrap_or(HeaderValue::from_static("10")),
        );
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            resp_headers,
            Json(serde_json::json!({
                "error": "Too Many Requests",
                "message": err_msg,
                "retry_after_seconds": retry_after
            })),
        ));
    }

    if payload.claim.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            Json(serde_json::json!({ "error": "Claim cannot be empty" })),
        ));
    }

    let claim = payload.claim.trim();

    // 3. Resolve API keys from secure server pool or per-request override
    let key1 = payload.api_key.as_deref().filter(|k| !k.is_empty())
        .or_else(|| state.key_pool.get(0).map(|s| s.as_str()));
    let key2 = payload.api_key.as_deref().filter(|k| !k.is_empty())
        .or_else(|| state.key_pool.get(1).map(|s| s.as_str()))
        .or(key1);
    let key3 = payload.api_key.as_deref().filter(|k| !k.is_empty())
        .or_else(|| state.key_pool.get(2).map(|s| s.as_str()))
        .or(key1);

    // 4. Concurrent dispatch to the 3 Gonka-routed models using tokio::join!
    let (res1, res2, res3) = tokio::join!(
        state.gonka_client.query_model(
            "deepseek-ai/DeepSeek-V4-Flash-0731",
            "Causal and logical fallacy analysis engine.",
            claim,
            key1
        ),
        state.gonka_client.query_model(
            "MiniMaxAI/MiniMax-M2.7",
            "Source attribution and news context verification engine.",
            claim,
            key2
        ),
        state.gonka_client.query_model(
            "deepseek-ai/DeepSeek-V4-Flash-0731",
            "Factual database and anti-hallucination verification engine.",
            claim,
            key3
        )
    );

    let model_results = vec![res1, res2, res3];
    let successful_models: Vec<models::ModelResult> = model_results.into_iter().filter_map(|r| r.ok()).collect();

    if successful_models.is_empty() {
        return Err((
            StatusCode::BAD_GATEWAY,
            HeaderMap::new(),
            Json(serde_json::json!({
                "error": "Gonka Router Unreachable",
                "message": "Failed to receive valid response from Gonka Router AI models. Please try again."
            })),
        ));
    }

    let consensus = ConsensusEngine::calculate_consensus(claim, successful_models);
    Ok(Json(consensus))
}

async fn verify_video_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyVideoRequest>,
) -> Result<impl IntoResponse, (StatusCode, HeaderMap, Json<serde_json::Value>)> {
    let meta = VideoParser::extract_metadata(&payload.video_url).await;
    let spoken_text = payload.spoken_transcript.unwrap_or(meta.spoken_transcript);
    let claim_prompt = format!("[Viral {} Reel Audio & Spoken Captions]: \"{}\"", meta.platform, spoken_text);

    let verify_req = VerifyRequest {
        claim: claim_prompt,
        is_url: Some(true),
        api_key: payload.api_key,
    };

    verify_claim_handler(State(state), headers, ConnectInfo(addr), Json(verify_req)).await
}

