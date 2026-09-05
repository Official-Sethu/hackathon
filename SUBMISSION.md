# Trace — Follow The Claim. Find The Truth.

**Project Name**: Trace  
**Slogan**: Follow The Claim. Find The Truth.  
**Track**: AI for Society (AI Fact Checker)  
**Live Demo**: Open `index.html` in browser  
**Live Rust Backend API**: `https://trace-backend-7bbm.onrender.com` (`/health`, `/api/verify`, `/api/verify_video`)  
**GitHub Repository**: Self-contained in workspace root  
**Routing Infrastructure**: Gonka Router (`https://api.gonkarouter.io/v1`)  

---

## 1. Problem Statement & Genuine Public Value

Misinformation regarding public health, climate science, elections, and macroeconomics circulates rapidly on social media, eroding social trust and causing tangible harm.

Traditional fact-checking platforms rely on single-model LLMs. Single models suffer from:
1. **Model & Provider Bias**: A single company's safety filters or ideological tuning skew outputs.
2. **Hallucination Risk**: Single models frequently fabricate peer-reviewed studies and URLs.
3. **Black-Box Opacity**: Citizens have no way to verify whether an inference genuinely occurred or was cached/manipulated.

**Trace solves this by creating a decentralized public truth infrastructure**. Every claim is cross-examined across three competing frontier architectures in parallel (**DeepSeek V4**, **MiniMax M2.7**, and **Kimi K2**), all routed through **Gonka Router** (`api.gonkarouter.io`). The platform surfaces transparent **Gonka Request IDs** for every query, providing cryptographic proof of inference to citizens, journalists, and researchers.

---

## 2. Mandatory Gonka Router Integration

| Requirement | Implementation |
|---|---|
| **Gateway URL** | `https://api.gonkarouter.io/v1` |
| **Routing Protocol** | OpenAI-compatible `/v1/chat/completions` |
| **Model Ensemble** | `deepseek-ai/DeepSeek-V4-Flash-0731`, `MiniMaxAI/MiniMax-M2.7`, `moonshotai/Kimi-K2` |
| **Audit Mechanism** | Captures response `id` and HTTP `x-request-id` header per model call |
| **Display** | Rendered in model cards, pipeline stream, shareable fact card, and public ledger |

---

## 3. Multi-Model Cross-Verification Methodology

Instead of a simple majority vote, Trace uses a **Confidence-Weighted Consensus Algorithm**:

1. **Real-Time Web Search**: Every claim is grounded by live Jina AI web search before any AI sees it. The search query is intelligently extracted from the claim (or video metadata) — not from raw system prompt strings.
2. **Concurrent Gonka Querying** via `tokio::join!`:
   - **DeepSeek V4** — Causal reasoning & formal logic analysis
   - **MiniMax M2.7** — Source attribution & news context verification
   - **Kimi K2** — Temporal cross-referencing & bias detection
3. **Consensus Synthesis**:
   - Confidence-weighted truth score (0–100)
   - Logical fallacy detection with penalties
   - Transparent reasoning trace with 5 reasoning steps per model
   - Dissent flag raised if models diverge by >25 points

---

## 4. Video Fact-Checking (Unique Feature)

Trace can fact-check **viral social media videos** — not just text:

- **Input**: Any TikTok, YouTube Shorts, Instagram Reel, Facebook, or X video URL
- **Extraction**: Multi-tier metadata extraction (oEmbed → noembed.com → OpenGraph HTML)
- **Search**: Smart query built from video title + author + platform sent to Jina AI
- **Output**: Full verdict with truth score, citations, and spoken transcript

**Live test result** (verified at submission time):
```
URL:      https://vt.tiktok.com/ZSq8csDSa/
VERDICT:  VERIFIED FACTUAL & AUTHENTIC
SCORE:    92/100
HEADLINE: Two Airlink passenger jets performed a low-altitude flyover over a packed
          Cape Town stadium, confirmed by multiple news sources.
SOURCES:  BBC, AP, People, CBS News
```

---

## 5. Multi-Platform Architecture

| Platform | Stack |
|---|---|
| **Web Client** | Vanilla HTML/CSS/JS, Plus Jakarta Sans, JetBrains Mono, zero-install |
| **Backend (Rust + Axum)** | Async Rust, tokio::join!, server-side key rotation, sliding window rate limiter |
| **Android Client** | Kotlin + Jetpack Compose, Material 3, live Gonka request telemetry |
| **iOS Client** | Swift + SwiftUI, native HIG-compliant design |

---

## 6. Security

- **Zero client-side key exposure**: All Gonka API keys are server-only environment variables on Render
- **Sliding window rate limiter**: Per-IP enforcement (20 req/60s) in Rust, no external dependency
- **Jina AI web search**: Authenticated via server-side `JINA_API_KEY` environment variable
- **CORS**: Configured for production cross-origin access

---

## 7. Verification & Reproducibility for Judges

### Web Demo
1. Open `index.html` in browser
2. Click any sample chip:
   - **Medical**: *"WHO announces coffee cures Covid-19"* → ~8% (Debunked)
   - **Science**: *"JWST detects water vapor on K2-18b"* → ~94% (Verified)
   - **Economy**: *"4-day work week boosts productivity 40%"* → ~52% (Mixed)
3. Observe live Gonka Router pipeline — status indicators update, **Gonka Request IDs** appear per model
4. Paste any TikTok/YouTube Shorts/Instagram URL to fact-check a viral video

### API (curl)
```bash
# Text claim
curl -X POST https://trace-backend-7bbm.onrender.com/api/verify \
  -H "Content-Type: application/json" \
  -d '{"claim": "The Eiffel Tower is in Berlin"}'

# Video URL
curl -X POST https://trace-backend-7bbm.onrender.com/api/verify_video \
  -H "Content-Type: application/json" \
  -d '{"video_url": "https://vt.tiktok.com/ZSq8csDSa/"}'
```

### Health Check
```bash
curl https://trace-backend-7bbm.onrender.com/health
```
