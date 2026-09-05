# VeritasGonka — Decentralized AI Multi-Model Fact-Checker for Society

> **AI for Society Hackathon Submission**  
> Built with genuine public value powered exclusively by **Gonka Router** at [`gonkarouter.io`](https://gonkarouter.io).

---

## Executive Summary

Single-model AI fact-checkers are vulnerable to provider bias, corporate alignment, and hallucinated citations. **VeritasGonka** is a decentralized public truth infrastructure that cross-examines breaking claims, news URLs, and viral statements across competing frontier AI architectures in parallel:
- **DeepSeek V4** (Logical & Causal Reasoning)
- **MiniMax M2.7** (Source Examination & Contextual Corroboration)
- **Kimi K2.6** (Anti-Hallucination & Factual Registry Cross-Checking)

All AI reasoning is routed through **Gonka Router** (`https://gonkarouter.io/v1`). Every model query surfaces an immutable, verifiable **Gonka Request ID** (e.g. `gonka-req-8f4b29a1e0c7`) guaranteeing cryptographic proof of inference and transparent public auditing.

---

## System Architecture

```
                                  ┌──────────────────────────────┐
                                  │      Citizen Endpoints       │
                                  ├──────────────┬───────────────┤
                                  │   Web (JS)   │ iOS (SwiftUI) │
                                  │ Android (KT) │ CLI / Ext     │
                                  └───────┬──────┴───────┬───────┘
                                          │              │
                                          ▼              ▼
                          ┌──────────────────────────────────────────────┐
                          │         VeritasGonka Rust Backend            │
                          │   (High-Performance Axum + Tokio Engine)     │
                          └──────────────────────┬───────────────────────┘
                                                 │
                                                 ▼
                          ┌──────────────────────────────────────────────┐
                          │                 GONKA ROUTER                 │
                          │         (https://gonkarouter.io/v1)          │
                          └──────────────────────┬───────────────────────┘
                                                 │
               ┌─────────────────────────────────┼─────────────────────────────────┐
               ▼                                 ▼                                 ▼
      ┌─────────────────┐               ┌─────────────────┐               ┌─────────────────┐
      │   DeepSeek V4   │               │   MiniMax M2.7  │               │    Kimi K2.6    │
      │ (Causal Logic)  │               │(Source Context) │               │(Hallucination)  │
      └────────┬────────┘               └────────┬────────┘               └────────┬────────┘
               │                                 │                                 │
               ▼                                 ▼                                 ▼
        Gonka Req ID #1                   Gonka Req ID #2                   Gonka Req ID #3
               │                                 │                                 │
               └─────────────────────────────────┼─────────────────────────────────┘
                                                 │
                                                 ▼
                          ┌──────────────────────────────────────────────┐
                          │          Consensus & Scoring Engine          │
                          │   • Truth Score (0–100%)                     │
                          │   • Gonka Request ID Cryptographic Audit     │
                          │   • Step-by-Step Reasoning Trace             │
                          └──────────────────────────────────────────────┘
```

---

## Core Features & Public Value

### 1. Dual Input Modality (Direct Statement or Article URL)
- Citizens can paste short quotes, viral social media posts, or complete article URLs.
- The engine isolates predicates, named entities, and empirical metrics for examination.

### 2. Multi-Model Cross-Verification via Gonka Router
- Dispatches parallel queries across **DeepSeek V4**, **MiniMax M2.7**, and **Kimi K2.6**.
- Measures execution latency (ms) and token utilization for complete transparency.

### 3. Transparent Gonka Request IDs
- Every single model evaluation captures and displays its unique **Gonka Request ID** (extracted from `x-request-id` headers and response metadata).
- 1-click clipboard copy allows researchers and journalists to verify inferences on the Gonka network.

### 4. Bayesian Truth Score (0–100%) & Reasoning Trace
- Aggregates multi-model stance alignment, factual density, and fallacy deductions into an animated circular gauge.
- Provides a transparent 4-stage cognitive timeline explaining how consensus was reached.
- Automatically flags rhetorical fallacies (e.g., False Authority Attribution, Causal Overreach).

### 5. Multilingual & Public Accessibility
- **Multilingual Support**: Supports English, Spanish, Chinese, French, Arabic, and Hindi.
- **Text-to-Speech Audio Verdict**: Built-in voice synthesizer for visually impaired citizens.
- **Printable Fact Card**: Generates shareable, high-contrast fact certificates complete with audit trails.
- **Public Audit Ledger**: Local/live verifiable record of recently audited claims with JSON export.

---

## Project Structure & Multi-Platform Stack

```
hackathon/
├── index.html              # Clean editorial Web interface (No dark glassmorphism)
├── styles.css              # Warm editorial CSS design system (Plus Jakarta Sans & JetBrains Mono)
├── app.js                  # Frontend orchestrator, TTS, animations, and ledger
├── gonkaApi.js             # Gonka Router client with multi-model dispatch & request ID capture
├── sampleClaims.js         # Curated civic claims (Medical, Climate, Economy, Science, Spanish)
│
├── backend-rust/           # High-Performance Backend (Rust & Axum)
│   ├── Cargo.toml          # Rust dependencies (Axum, Tokio, Reqwest, Serde)
│   ├── src/main.rs         # Axum HTTP server & CORS
│   ├── src/models.rs       # Strongly typed verification data contracts
│   ├── src/gonka_client.rs # Asynchronous Gonka Router client (tokio::join!)
│   ├── src/consensus.rs    # Bayesian consensus scoring algorithm
│   └── README.md           # Backend documentation
│
├── mobile-ios/             # Native iOS Client (Swift & SwiftUI)
│   ├── VeritasGonkaApp.swift # iOS app lifecycle entry
│   ├── FactCheckView.swift   # Editorial SwiftUI view with Truth Score gauge
│   └── GonkaService.swift    # Network actor communicating with Gonka Router
│
├── mobile-android/         # Native Android Client (Kotlin & Jetpack Compose)
│   ├── MainActivity.kt     # Android ComponentActivity
│   ├── FactCheckScreen.kt  # Material 3 Compose UI with Gonka Request badges
│   └── GonkaRepository.kt  # Coroutines repository for Gonka verification
│
├── SUBMISSION.md           # Formal hackathon submission answers
└── DEMO_VIDEO_SCRIPT.md    # Precision 2-minute live fact-check video script
```

---

## Getting Started

### Option 1: Web Client (Zero-Install / Instant Launch)
Simply open `index.html` in any modern web browser:
```bash
# Windows
start index.html

# Mac
open index.html

# Linux
xdg-open index.html
```
- Click any sample chip (e.g. **"Coffee cures Covid-19 mutations"**) to watch the live Gonka Router pipeline dispatch, capture **Gonka Request IDs**, and render the **Truth Score** and reasoning trace.
- Click **"Gonka Config"** to enter your personal API key or change model endpoints. If left blank, the app seamlessly runs in **Gonka High-Fidelity Testnet Simulation Mode**.

---

### Option 2: Rust Axum Backend (Live API & Local)
- **Live Production Endpoint**: `https://trace-backend-7bbm.onrender.com`
- **Local Dev Server**:
```bash
cd backend-rust
cargo run
```
The Axum service starts on `http://localhost:8080` (or live on `https://trace-backend-7bbm.onrender.com`) exposing:
- `POST /api/verify`: Dispatches parallel queries to Gonka Router and calculates Bayesian consensus.
- `POST /api/verify_video`: Multi-tiered short video reel metadata scraper & LLM query feeder.
- `GET /api/models`: Returns configured models and Gonka gateway status.
- `GET /health`: Health and connectivity check.

---

### Option 3: iOS (SwiftUI)
Open Xcode, import `mobile-ios/`, and run on simulator or device (iOS 16+).

---

### Option 4: Android (Jetpack Compose)
Open Android Studio, import `mobile-android/`, and build with Gradle (Android API 26+).

---

## Gonka Router Integration Details

| Parameter | Configuration |
|---|---|
| **Gateway URL** | `https://gonkarouter.io/v1` |
| **API Format** | OpenAI-Compatible Chat Completions (`/chat/completions`) |
| **Model 1 (Causal Logic)** | `deepseek-ai/deepseek-v4` (Dedicated Key 1) |
| **Model 2 (Source Context)** | `minimax/minimax-m2.7` (Dedicated Key 2) |
| **Model 3 (Anti-Hallucination)**| `moonshot/kimi-k2.6` (Dedicated Key 3) |
| **Key Architecture** | 3-Key Distributed Pool with automatic failover rotation |
| **Request ID Header** | `x-request-id` |

---

## License & Public Value
Released as an open-source civic utility under the **MIT License**. Built to empower citizens, journalists, and educators worldwide with trustworthy, auditable truth verification.
