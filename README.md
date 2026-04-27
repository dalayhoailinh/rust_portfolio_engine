# rust_portfolio_engine

Pure-Rust portfolio math engine. Built in Q2 Week 1 of the 2026 roadmap as the
foundation that will replace the Dart math layer of [portfolio_tracker](https://github.com/dalayhoailinh/portfolio_tracker)
in Q2 Week 4 via `flutter_rust_bridge`.

## What it does today
- `Position` struct mirroring the Dart entity (symbol, quantity, avg buy price).
- P&L, market value and percentage helpers.
- `buy` / `withdraw` returning `Result<_, TradingError>`.
- **Async I/O via Tokio**: `load_ohlcv_csv` reads OHLCV bars off disk without blocking.
- **`OhlcvBar` + serde**: zero-boilerplate JSON / CSV parsing — same shape EODHD returns in Q2 Week 5.
- **Indicators**: O(n) Simple Moving Average, ready to draw on top of the Q1 candlestick chart.
- **Concurrent multi-symbol loader** with `tokio::try_join!` — measurable speedup over sequential.
- 11 unit tests covering math, JSON roundtrip, parity with the Dart side.

## Run
```bash
cargo run                # demo: load 3 CSVs, compute SMA(20), print signals
cargo run --release      # release build for honest timing comparison
cargo test               # all 11 tests must be green
```

## Roadmap link
- ✅ Week 1 — ownership, borrowing, struct, enum, Result, Option
- ✅ Week 2 — async with Tokio + JSON parsing with serde + CSV + SMA (this commit)
- ⏳ Week 3 — `flutter_rust_bridge` integration: call this engine from Dart
- ⏳ Week 4 — replace Dart math in `portfolio_tracker` with Rust calls + benchmark