mod indicators;
mod market_data;
mod position;
mod trading;

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Q2 Week 2 - async engine starting");

    let price = fetch_fake_price("AAPL").await;
    println!("Fetched price for AAPL = ${}", price);

    let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 107.0];
    let msa_3 = indicators::simple_moving_average(&prices, 3);
    println!("SMA(3) for {:?}", prices);
    for (i, v) in msa_3.iter().enumerate() {
        match v {
            Some(x) => println!("  bar {i:>2} -> {:.2}", x),
            None => println!("  bar {i:>2} -> (warming up)"),
        }
    }
    println!("Q2 Week 2 - done");
}

async fn fetch_fake_price(symbol: &str) -> f64 {
    sleep(Duration::from_millis(200)).await;
    match symbol {
        "AAPL" => 187.45,
        _ => 0.0,
    }
}
