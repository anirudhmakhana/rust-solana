# Project Spec: Pulse — A Mini Solana Portfolio Analytics API

> A simplified, in-memory version of what Step Finance or Birdeye does:
> track Solana wallets, store portfolio snapshots, and compute analytics.
> Built entirely in Rust. No database required for the POC.

---

## What You're Building

**Pulse** is a REST API backend that:
- Tracks Solana wallet addresses
- Accepts portfolio snapshots (token balances + USD values)
- Computes PnL between snapshots
- Serves analytics across all tracked wallets

You've used all 30 concepts from this repo. This project forces you to use every single one in a real context.

### Why this project?

| Property | Why it matters |
|---|---|
| Simplified Birdeye / Step Finance | Readers immediately know what this is for |
| No database needed | You can ship a POC in one sitting |
| All Rust concepts used | Every chapter of CONCEPTS.md maps to a real file |
| Real API design | Teaches routing, error handling, shared state |
| Extensible | Phase 2 can hit actual Solana RPC — just swap the data layer |
| Great blog material | "I built a Solana analytics API in pure Rust" is a strong post |

---

## Tech Stack (2026)

```toml
# Cargo.toml
[dependencies]
axum = "0.8"                    # Web framework (Tokio-native, composable)
tokio = { version = "1", features = ["full"] }  # Async runtime
serde = { version = "1", features = ["derive"] } # Serialization
serde_json = "1"                # JSON support
tower-http = { version = "0.6", features = ["cors", "trace"] }  # Middleware
tracing = "0.1"                 # Structured logging
tracing-subscriber = "0.3"     # Log configuration
uuid = { version = "1", features = ["v4"] }  # Snapshot IDs
chrono = { version = "0.4", features = ["serde"] }  # Timestamps
reqwest = { version = "0.12", features = ["json"] }  # (Phase 2: real RPC)
```

**Why Axum?** It's the dominant Rust web framework in 2026: zero-cost abstractions,
tower middleware compatibility, Tokio-native, and the best type-safe extractor system.

---

## Architecture

```
src/
├── main.rs           # Entry point: build router, start server
├── state.rs          # AppState (Arc<RwLock<Store>>) — shared in-memory storage
├── models/
│   ├── mod.rs
│   ├── wallet.rs     # Wallet struct, WalletStatus enum
│   ├── portfolio.rs  # Portfolio, TokenBalance, Token structs
│   └── analytics.rs  # PnL, TopToken, Summary structs
├── routes/
│   ├── mod.rs
│   ├── health.rs     # GET /health
│   ├── wallets.rs    # CRUD for wallets
│   ├── portfolio.rs  # Snapshot submission + retrieval
│   └── analytics.rs  # PnL + cross-wallet analytics
├── errors.rs         # AppError enum, impl IntoResponse
└── traits.rs         # Summarize, Validate traits
```

---

## Domain Model

### Structs you'll write

```rust
// models/wallet.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,          // Solana public key (base58)
    pub label: Option<String>,    // Human-readable nickname
    pub tracked_since: DateTime<Utc>,
    pub status: WalletStatus,
}

// models/portfolio.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub mint: String,             // Token mint address
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub token: Token,
    pub raw_amount: u64,          // On-chain amount (unscaled)
    pub usd_value: Option<f64>,   // May be unknown for obscure tokens
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: String,               // UUID
    pub wallet_address: String,
    pub balances: Vec<TokenBalance>,
    pub total_usd: f64,
    pub snapshot_at: DateTime<Utc>,
}

// models/analytics.rs
#[derive(Debug, Serialize)]
pub struct PnL {
    pub wallet_address: String,
    pub from_snapshot: String,    // snapshot UUID
    pub to_snapshot: String,
    pub from_usd: f64,
    pub to_usd: f64,
    pub delta_usd: f64,
    pub delta_pct: f64,
}

#[derive(Debug, Serialize)]
pub struct TopToken {
    pub token: Token,
    pub wallet_count: usize,      // How many tracked wallets hold this
    pub total_usd: f64,
}
```

### Enums you'll write

```rust
// models/wallet.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalletStatus {
    Active,
    Paused,
    Error(String),  // data-carrying variant
}

// errors.rs
#[derive(Debug)]
pub enum AppError {
    WalletNotFound(String),
    DuplicateWallet(String),
    InsufficientSnapshots { wallet: String, found: usize, need: usize },
    InvalidAddress(String),
    Internal(String),
}
```

### Traits you'll write

```rust
// traits.rs
pub trait Summarize {
    fn summary(&self) -> String;
}

pub trait Validate {
    fn validate(&self) -> Result<(), AppError>;
}

// You implement both on Wallet and Portfolio
impl Summarize for Wallet { ... }
impl Summarize for Portfolio { ... }
impl Validate for Wallet { ... }
```

### Generics you'll write

```rust
// Every endpoint returns this wrapper
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self { ... }
    pub fn err(msg: impl Into<String>) -> ApiResponse<()> { ... }
}
```

### Shared State (Ownership in practice)

```rust
// state.rs
#[derive(Default)]
pub struct Store {
    pub wallets: HashMap<String, Wallet>,
    pub snapshots: HashMap<String, Vec<Portfolio>>,  // keyed by wallet address
}

// Shared across all request handlers via Arc<RwLock<>>
pub type AppState = Arc<RwLock<Store>>;
```

This is where ownership clicks in a real context:
- `Arc` — multiple owners across threads
- `RwLock` — many readers OR one writer at a time
- `.read()` / `.write()` — explicit borrow control at runtime

---

## API Endpoints

### Health
```
GET  /health              → { status: "ok", version: "0.1.0" }
```

### Wallets
```
POST   /wallets                  → Register a wallet
GET    /wallets                  → List all tracked wallets
GET    /wallets/:address         → Get wallet details
PATCH  /wallets/:address         → Update label or status
DELETE /wallets/:address         → Untrack wallet (deletes all snapshots)
```

### Portfolio Snapshots
```
POST  /wallets/:address/snapshots        → Submit a portfolio snapshot
GET   /wallets/:address/snapshots        → List all snapshots for wallet
GET   /wallets/:address/snapshots/latest → Get the most recent snapshot
GET   /wallets/:address/pnl              → Compute PnL (first vs latest snapshot)
```

### Analytics
```
GET  /analytics/top-tokens               → Tokens held across the most wallets
GET  /analytics/summary                  → Total wallets tracked, total USD, avg portfolio
```

### Example Request/Response

**POST /wallets**
```json
// Request
{ "address": "9WzDXwBbmkg8...", "label": "My DeFi Wallet" }

// Response
{
  "success": true,
  "data": {
    "address": "9WzDXwBbmkg8...",
    "label": "My DeFi Wallet",
    "tracked_since": "2026-03-23T10:00:00Z",
    "status": "Active"
  },
  "timestamp": "2026-03-23T10:00:00Z"
}
```

**POST /wallets/:address/snapshots**
```json
// Request — you send this manually in the POC (no real RPC call yet)
{
  "balances": [
    {
      "token": { "mint": "So11111...", "symbol": "SOL", "decimals": 9 },
      "raw_amount": 5000000000,
      "usd_value": 750.00
    },
    {
      "token": { "mint": "EPjFWdd5...", "symbol": "USDC", "decimals": 6 },
      "raw_amount": 250000000,
      "usd_value": 250.00
    }
  ],
  "total_usd": 1000.00
}

// Response
{ "success": true, "data": { "id": "uuid-here", "snapshot_at": "..." } }
```

**GET /wallets/:address/pnl**
```json
{
  "success": true,
  "data": {
    "wallet_address": "9WzDXwBbmkg8...",
    "from_snapshot": "uuid-a",
    "to_snapshot": "uuid-b",
    "from_usd": 1000.00,
    "to_usd": 1340.00,
    "delta_usd": 340.00,
    "delta_pct": 34.0
  }
}
```

---

## Rust Concepts → Real Code

This is why this project was designed this way. Every concept you learned has a home:

| Concept | Where it appears |
|---|---|
| Structs | `Wallet`, `Portfolio`, `TokenBalance`, `Token`, `PnL`, `ApiResponse<T>` |
| Enums | `WalletStatus`, `AppError` |
| Traits | `Summarize`, `Validate`, `IntoResponse` for AppError |
| Generics | `ApiResponse<T>`, handler type params |
| Option | `label: Option<String>`, `usd_value: Option<f64>`, `.get()` on HashMap |
| Result + ? | Every route handler returns `Result<Json<ApiResponse<T>>, AppError>` |
| HashMap | `Store.wallets`, `Store.snapshots` |
| Vec | `Portfolio.balances`, `Store.snapshots[address]` |
| Iterators | Analytics: `.filter()`, `.map()`, `.fold()`, `.max_by()`, `.collect()` |
| Closures | Passed to iterator adapters in analytics calculations |
| Ownership | `Arc<RwLock<Store>>` shared across handlers |
| Borrowing | `.read()` guard gives `&Store` |
| Mutable borrowing | `.write()` guard gives `&mut Store` |
| Derive macros | `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]` on every model |
| Lifetimes | In trait method signatures returning borrowed data |
| Associated types | Custom iterator if you build `SnapshotIterator` |
| Shadowing | Request body parsing pipelines |
| Match | Error handling, status matching in AppError |
| If as expression | Inline PnL percentage calc, label formatting |
| Mutability | Building response data inside handlers |
| Primitives | `u64` raw amounts, `f64` USD values, `u8` decimals |
| Variable shadowing | Parsing raw request fields into typed values |
| Strings | Address validation, label trimming |

---

## Build Plan

### Phase 0: Project Setup (30 min)
```bash
cargo new pulse --bin
cd pulse
# Add all dependencies to Cargo.toml
```
Goal: `cargo run` starts a server on port 3000.

### Phase 1: Types + State (1–2 hours)
- Write all structs and enums in `models/`
- Write `AppError` and implement `IntoResponse`
- Write `AppState` in `state.rs`
- Write `ApiResponse<T>` generic wrapper
- Write `Summarize` and `Validate` traits

Goal: All types compile. No routes yet.

### Phase 2: Health + Wallet CRUD (2–3 hours)
- Implement `GET /health`
- Implement `POST /wallets` (validate address format, check for duplicates)
- Implement `GET /wallets` (list all)
- Implement `GET /wallets/:address` (single wallet or 404)
- Implement `DELETE /wallets/:address`

Goal: Test with `curl` or Postman. All wallet endpoints work.

### Phase 3: Portfolio Snapshots (2–3 hours)
- Implement `POST /wallets/:address/snapshots`
- Implement `GET /wallets/:address/snapshots`
- Implement `GET /wallets/:address/snapshots/latest` (iterator `.last()`)
- Implement `GET /wallets/:address/pnl`

Goal: Full snapshot lifecycle works. PnL math is correct.

### Phase 4: Analytics (2 hours)
- Implement `GET /analytics/top-tokens`
  (iterator chain: flatten all balances → group by mint → sort by wallet_count)
- Implement `GET /analytics/summary`
  (fold across all wallets: total wallets, total USD, average)

Goal: Analytics endpoints return meaningful data across multiple wallets.

### Phase 5: Polish (1 hour)
- Add `tower-http` CORS middleware
- Add `tracing` request logging
- Write a README with curl examples
- Write the blog post

---

## The Analytics Iterator Chain (Preview)

This is the most satisfying piece of Rust code you'll write in this project.
The `GET /analytics/top-tokens` handler looks like this:

```rust
async fn top_tokens(State(state): State<AppState>) -> impl IntoResponse {
    let store = state.read().await;

    let mut token_stats: HashMap<String, (Token, HashSet<String>, f64)> = HashMap::new();

    // Iterator pipeline:
    // - iterate all wallets
    // - flatten all their snapshots
    // - flatten all token balances in each snapshot
    // - group by token mint
    store.snapshots
        .iter()
        .flat_map(|(wallet_addr, snapshots)| {
            snapshots.iter().flat_map(move |p| {
                p.balances.iter().map(move |b| (wallet_addr.clone(), b.clone()))
            })
        })
        .for_each(|(wallet_addr, balance)| {
            let entry = token_stats
                .entry(balance.token.mint.clone())
                .or_insert_with(|| (balance.token.clone(), HashSet::new(), 0.0));
            entry.1.insert(wallet_addr);
            entry.2 += balance.usd_value.unwrap_or(0.0);
        });

    let mut top: Vec<TopToken> = token_stats
        .into_values()
        .map(|(token, wallets, total_usd)| TopToken {
            token,
            wallet_count: wallets.len(),
            total_usd,
        })
        .collect();

    top.sort_by(|a, b| b.wallet_count.cmp(&a.wallet_count));
    top.truncate(10);

    Json(ApiResponse::ok(top))
}
```

This single function uses: iterators, closures, flat_map, HashMap entry API,
HashSet, collect, sort_by, ownership, borrowing, and generics. All at once.

---

## Blog Post Outline

Write this in parallel as you build each phase.

```
Title: "I built a Solana Portfolio API in Rust from scratch"
Subtitle: "What I learned about ownership, iterators, and error handling by building a real backend"

1. Why Rust for a backend? (not "it's fast" — explain the safety story)
2. The ownership model as a web server superpower (Arc<RwLock<>> explained)
3. Error handling without exceptions (AppError, ?, IntoResponse)
4. Shared state without data races (compile-time guarantee vs runtime lock)
5. Trait-based polymorphism (Summarize, Validate, IntoResponse)
6. Iterators: the most powerful feature nobody talks about
   → Walk through the top-tokens pipeline line by line
7. What I'd do differently (what the POC is missing: persistence, auth, real RPC)
8. What comes next (connecting to real Solana RPC with reqwest)
```

---

## What the POC is Intentionally Missing (Phase 2 extensions)

| Feature | Why excluded from POC | How to add it |
|---|---|---|
| Real Solana RPC | Adds reqwest + async complexity | `reqwest::Client` + Helius/Alchemy API |
| Database | Adds SQLx/Diesel complexity | Replace `HashMap` with `sqlx::PgPool` |
| Authentication | Adds JWT/session complexity | `axum-extra` JWT extractor |
| Token price feed | Adds external API dependency | Jupiter price API via reqwest |
| WebSockets | Adds streaming complexity | `axum::extract::ws` |

The POC is complete without any of these. They are natural next steps.

---

## Quick Start Commands

```bash
# Create the project
cargo new pulse --bin && cd pulse

# Run with auto-reload (install with: cargo install cargo-watch)
cargo watch -x run

# Test the API
curl http://localhost:3000/health

curl -X POST http://localhost:3000/wallets \
  -H "Content-Type: application/json" \
  -d '{"address":"9WzDXwBbmkg8ZZdBmSJqBUqdFqkL9fGnPwKTkRbFhCrX","label":"Test Wallet"}'

curl http://localhost:3000/wallets
```

---

## Where to Start Right Now

1. `cargo new pulse --bin`
2. Copy the `[dependencies]` block above into `Cargo.toml`
3. Run `cargo build` to download and compile all deps (takes a few minutes first time)
4. Write `models/wallet.rs` first — it's just structs and an enum, no async yet
5. Get one endpoint working (`GET /health`) before touching state

The hardest part is Phase 1 — once the types compile, the rest flows naturally.
