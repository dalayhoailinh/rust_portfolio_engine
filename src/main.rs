mod indicators;
mod market_data;

use anyhow::Result;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Q2 Week 2 - async engine starting");

    // Sequential baseline
    let t0 = Instant::now();
    let _ = market_data::load_ohlcv_csv("data/aapl_30d.csv").await?;
    let _ = market_data::load_ohlcv_csv("data/msft_30d.csv").await?;
    let _ = market_data::load_ohlcv_csv("data/googl_30d.csv").await?;
    let seq_ms = t0.elapsed().as_micros() as f64 / 1000.0;
    println!("Sequential  : {:>6.2} ms", seq_ms);

    // Concurrent with try_join!
    let t1 = Instant::now();
    let (aapl, msft, googl) = tokio::try_join!(
        market_data::load_ohlcv_csv("data/aapl_30d.csv"),
        market_data::load_ohlcv_csv("data/msft_30d.csv"),
        market_data::load_ohlcv_csv("data/googl_30d.csv"),
    )?;
    let par_ms = t1.elapsed().as_micros() as f64 / 1000.0;
    println!("Concurrent  : {:>6.2} ms", par_ms);
    println!(
        "AAPL bars: {}, MSFT bars: {}. GOOGL bars: {}",
        aapl.len(),
        msft.len(),
        googl.len()
    );

    // Compute SMA(20) on each
    for (sym, bars) in
        [("AAPL", &aapl), ("MSFT", &msft), ("GOOGL", &googl)]
    {
        let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
        let sma = indicators::simple_moving_average(&closes, 20);

        let last_sma = sma.last().and_then(|x| *x).unwrap_or(f64::NAN);
        let last_close = bars.last().map(|b| b.close).unwrap_or(0.0);
        let signal =
            if last_close > last_sma { "BULL ▲" } else { "BEAR ▼" };
        println!(
            "  {sym:<5} close = {:>7.2}  SMA20 = {:>7.2}  {signal}",
            last_close, last_sma
        )
    }

    println!("Q2 Week 2 - done");
    Ok(())
}
