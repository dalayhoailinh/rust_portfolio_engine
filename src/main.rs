mod position;
use position::Position;

fn main() {
    let aapl = Position::new("AAPL", 10.0, 100.0);

    println!("Position: {:?}", aapl);
    println!("Total cost: ${}", aapl.total_cost());
    println!("Market value @120: ${}", aapl.market_value(120.0));
    println!("PnL @120: ${}", aapl.unrealized_pnl(120.0));
    println!("PnL % @120: {:.2}%", aapl.unrealized_pnl_percent(120.0));
}
