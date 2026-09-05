# VeritasGonka — High-Performance Rust Axum Backend

The core asynchronous backend engine for VeritasGonka, providing high-throughput multi-model cross-examination routed exclusively through **Gonka Router** (`https://gonkarouter.io/v1`).

---

## Key Capabilities
- **Axum & Tokio Async Runtime**: Dispatches concurrent HTTP completions across DeepSeek V4, MiniMax M2.7, and Kimi K2.6 with zero blocking.
- **Cryptographic Gonka Request ID Tracking**: Captures and validates `x-request-id` headers directly from the Gonka network.
- **Bayesian Consensus Engine**: In-memory algorithmic weighting of model confidence, stance alignment, and logical fallacy deductions.
- **Cross-Platform CORS Support**: Exposes clean JSON endpoints for Web (`index.html`), iOS (SwiftUI), and Android (Jetpack Compose).

---

## API Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/health` | Healthcheck & Gonka gateway connectivity |
| `GET` | `/api/models` | List of models routed via `gonkarouter.io` |
| `POST` | `/api/verify` | Multi-model fact-checking and consensus generation |

### Verification Request Body
```json
{
  "claim": "WHO announces coffee cures Covid-19 mutations",
  "is_url": false,
  "api_key": "sk-gonka-optional-key"
}
```

---

## Live Production Deployment

The Rust backend is deployed live on Render:
- **Live Base URL**: `https://trace-backend-7bbm.onrender.com`
- **Health Check**: `https://trace-backend-7bbm.onrender.com/health`
- **Verify Claim Endpoint**: `POST https://trace-backend-7bbm.onrender.com/api/verify`
- **Verify Short Video Reel Endpoint**: `POST https://trace-backend-7bbm.onrender.com/api/verify_video`

---

## Quickstart (Local)

```bash
# Navigate to backend directory
cd backend-rust

# Run in development mode
cargo run

# Build optimized production binary
cargo build --release
```
Server runs on `http://localhost:8080` (or `https://trace-backend-7bbm.onrender.com` in production).
