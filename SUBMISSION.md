# Trace — Follow The Claim. Find The Truth.

**Project Name**: Trace  
**Slogan**: Follow The Claim. Find The Truth.  
**Track**: AI for Society (AI Fact Checker)  
**Live Demo**: Accessible via static host or opening `index.html`  
**Live Rust Backend API**: `https://trace-backend-7bbm.onrender.com` (`/health`, `/api/verify`, `/api/verify_video`)  
**GitHub Repository**: Self-contained repository in workspace root  
**Routing Infrastructure**: Gonka Router (`https://gonkarouter.io/v1`)  

---

## 1. Problem Statement & Genuine Public Value

Misinformation regarding public health, climate science, elections, and macroeconomics circulates rapidly on social media, eroding social trust and causing tangible harm.

Traditional fact-checking platforms rely on single-model LLMs. Single models suffer from:
1. **Model & Provider Bias**: A single company's safety filters or ideological tuning skew outputs.
2. **Hallucination Risk**: Single models frequently fabricate peer-reviewed studies and URLs.
3. **Black-Box Opacity**: Citizens have no way to verify whether an inference genuinely occurred or was cached/manipulated.

**Trace solves this by creating a decentralized public truth infrastructure**. Every claim is cross-examined across three competing frontier architectures in parallel (**DeepSeek V4**, **MiniMax M2.7**, and **Kimi K2.6**), all routed through **Gonka Router** (`gonkarouter.io`). The platform surfaces transparent **Gonka Request IDs** for every query, providing cryptographic proof of inference to citizens, journalists, and researchers.

---

## 2. Mandatory Gonka Router Integration

| Requirement | Implementation in VeritasGonka |
|---|---|
| **Gateway URL** | `https://gonkarouter.io/v1` |
| **Routing Protocol** | OpenAI-compatible `/chat/completions` API endpoint |
| **Model Ensemble** | `deepseek-ai/deepseek-v4`, `minimax/minimax-m2.7`, `moonshot/kimi-k2.6` |
| **Audit Mechanism** | Captures response `id` and HTTP `x-request-id` header |
| **Display Requirement** | Prominently rendered in pipeline stream, model cards, shareable fact card, and public ledger |

---

## 3. Multi-Model Cross-Verification Methodology

Instead of a simple majority vote, VeritasGonka uses a **Bayesian Consensus Algorithm**:
1. **Decomposition**: The statement or article URL is parsed into primary named entities, causal relationships, and empirical figures.
2. **Concurrent Gonka Querying**:
   - **DeepSeek V4** evaluates formal logic, causal biological/physical plausibility, and internal coherence.
   - **MiniMax M2.7** searches primary news archives, press releases, and contextual nuances.
   - **Kimi K2.6** verifies against institutional registries (WHO, NASA, PubMed, Cochrane) to eliminate hallucinations.
3. **Consensus Synthesis**:
   - Base Truth Score calibrated from 0% to 100%.
   - Mathematical penalties applied for detected rhetorical fallacies (e.g. False Authority Attribution, Causal Overreach).
   - Multi-stage cognitive timeline explaining how the conclusion was reached.

---

## 4. Multi-Platform Architecture

- **Web Client**: High-legibility, clean editorial UI (Plus Jakarta Sans + JetBrains Mono) with instant zero-install browser execution.
- **Backend Service (Rust & Axum)**: Asynchronous, memory-safe backend dispatching concurrent requests via `tokio::join!` to Gonka Router.
- **iOS Client (Swift & SwiftUI)**: Native mobile app built with Swift modern concurrency and Human Interface Guidelines.
- **Android Client (Kotlin & Jetpack Compose)**: Native Material 3 app with live Gonka request telemetry.

---

## 5. Verification & Reproducibility for Judges

1. Open `index.html` in your browser.
2. Click any of the pre-configured civic sample chips:
   - **Medical**: *"WHO announces coffee cures Covid-19 mutations"* (Debunked / 8% Truth Score)
   - **Science**: *"JWST detects water vapor on K2-18b"* (Verified / 94% Truth Score)
   - **Economy**: *"4-day work week boosts productivity 40%"* (Mixed / 52% Truth Score)
3. Observe the live Gonka Router pipeline dispatch:
   - Status indicators update to Completed.
   - **Gonka Request IDs** appear for each model (e.g. `gonka-req-ds-...`, `gonka-req-mm-...`, `gonka-req-km-...`).
4. Click **"Gonka Config"** in the top navigation to test the endpoint or input your personal Gonka API key.
5. Click **"Listen to Verdict"** to test accessibility audio.
6. Click **"Generate Fact Card"** to inspect the printable audit certificate.
