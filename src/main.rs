fn main() {
    // immutable
    let cash: f64 = 10_000.0;
    println!("Starting cash: ${}", cash);

    // mutable
    let mut realized_pnl: f64 = 0.0;
    realized_pnl += 250.5;
    realized_pnl -= 100.00;
    println!("Realized PnL after 2 trades: ${}", realized_pnl);

    // type inference
    let symbol = "AAPL";
    let quantity = 10;
    let avg_price = 150.0;

    // shadowing
    let price_label = avg_price;
    let price_label = format!("${}", price_label);
    println!("{} x {} @ {}", symbol, quantity, price_label);
}
