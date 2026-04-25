mod position;
mod trading;

use position::Position;
use trading::{TradingError, buy, withdraw};

fn main() {
    let cash = 10_000.0;

    match buy(cash, None, "AAPL", 10.0, 100.0) {
        Ok((pos, new_cash)) => {
            println!("Bought: {:?}", pos);
            println!("Cash left: ${}", new_cash);
        }
        Err(e) => println!("Buy failed: {:?}", e),
    }

    match buy(cash, None, "TSLA", 100.0, 250.0) {
        Ok((pos, new_cash)) => {
            println!("Bought: {:?}, cash {}", pos, new_cash)
        }
        Err(TradingError::InsufficientCash { needed, have }) => {
            println!("Need ${}, have ${}", needed, have);
        }
        Err(e) => println!("Other error: {:?}", e),
    }

    let pos = Position::new("AAPL", 10.0, 100.00);
    match withdraw(&pos, 50.0, 120.0) {
        Ok(cash) => println!("Got ${} cash", cash),
        Err(e) => println!("Withdraw failed: {:?}", e),
    }
}
