mod position;
mod trading;

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Q2 Week 2 - async engine starting");

    let price = fetch_fake_price("AAPL").await;
    println!("Fake price for AAPL = ${}", price);

    println!("Q2 Week 2 - done");
}

async fn fetch_fake_price(symbol: &str) -> f64 {
    sleep(Duration::from_millis(200)).await;
    match symbol {
        "AAPL" => 187.45,
        _ => 0.0,
    }
}
