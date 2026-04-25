# rust_portfolio_engine

Pure-Rust portfolio math engine. Built in Q2 Week 1 of the 2026 roadmap as the
foundation that will replace the Dart math layer of [portfolio_tracker](https://github.com/dalayhoailinh/portfolio_tracker)
in Q2 Week 4 via `flutter_rust_bridge`.

## What it does today
- `Position` struct mirroring the Dart entity (symbol, quantity, avg buy price).
- P&L, market value and percentage helpers.
- `buy` / `withdraw` returning `Result<_, TradingError>`.
- Unit tests for every math path.

## Run

```bash
cargo run
cargo test
```

## Why Rust here?
Q1 closed with a Flutter app where all P&L math lives in Dart. Q2 moves that
math into a Rust crate — same logic, same numbers, different runtime path.
The win shows up in Q3 when the engine has to compute RSI / MACD / Bollinger
Bands at thousands of ticks per second without blocking the Flutter UI thread.

## Roadmap link
- ✅ Week 1 — ownership, borrowing, struct, enum, Result, Option (this repo)
- ⏳ Week 2 — async with Tokio + JSON parsing with serde
- ⏳ Week 3 — `flutter_rust_bridge` integration
- ⏳ Week 4 — replace Dart math in `portfolio_tracker` with Rust calls + benchmark