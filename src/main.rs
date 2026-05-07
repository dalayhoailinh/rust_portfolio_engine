fn main() {
    let cash: f64 = 10_000.0;
    println!("Starting cash: ${}", cash);

    // cash = 5_000.0; // FAIL

    // Using mut
    let mut realized_pnl: f64 = 0.0;
    realized_pnl += 250.0;
    realized_pnl -= 100.0;
    println!("Realized PnL after 2 trades: ${}", realized_pnl);

    // type inference
    let symbol = "AAPL";
    let quantity = 10;
    let avg_price = 187.45;

    // shadowing
    let price_label = avg_price;
    let price_label = format!("${}", price_label);
    // same name, new type, OK

    println!("{} x {} @ {}", symbol, quantity, price_label);
}
