mod indicators;
mod market_data;

#[tokio::main]
async fn main() {
    println!("Q2 Week 2 - async engine starting");

    let prices = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 107.0];
    let sma_3 = indicators::simple_moving_average(&prices, 3);
    println!("SMA(3) for {:?}", prices);
    for (i, v) in sma_3.iter().enumerate() {
        match v {
            Some(x) => println!("  bar {i:>2} → {:.2}", x),
            None => println!("  bar {i:>2} → (warming up)"),
        }
    }

    println!("Q2 Week 2 - done");
}
