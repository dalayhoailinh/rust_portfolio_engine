mod indicators;
mod market_data;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Q2 Week 2 - async engine starting");

    let aapl = market_data::load_ohlcv_csv("data/aapl_30d.csv").await?;
    println!("Loaded {} AAPL bars", aapl.len());

    let closes: Vec<f64> = aapl.iter().map(|b| b.close).collect();

    let sma_5 = indicators::simple_moving_average(&closes, 5);
    let sma_20 = indicators::simple_moving_average(&closes, 20);

    println!("\n  date         close   sma5    sma20");
    println!("  ----------   -----   -----   -----");
    for i in 0..aapl.len() {
        let s5 = sma_5[i]
            .map(|x| format!("{:>6.2}", x))
            .unwrap_or("  --  ".into());
        let s20 = sma_20[i]
            .map(|x| format!("{:>6.2}", x))
            .unwrap_or("  --  ".into());
        println!(
            "  {}   {:>6.2}  {}  {}",
            aapl[i].date, aapl[i].close, s5, s20
        );
    }

    println!("Q2 Week 2 - done");
    Ok(())
}
