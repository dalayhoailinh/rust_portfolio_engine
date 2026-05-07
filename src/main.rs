mod position;
mod trading;
use crate::trading::{buy, withdraw};
use position::Position;

fn main() {
    let cash = 10_000.0;

    // match: try-catch
    match buy(cash, None, "AAPL", 10.0, 100.0) {
        Ok((pos, new_cash)) => {
            println!("Bought: {:?}", pos);
            println!("Cash left: {}", new_cash);
        }
        Err(e) => println!("Buy failed: {:?}", e),
    }

    match buy(cash, None, "TLSA", 100.0, 250.0) {
        Ok((pos, new_cash)) => {
            println!("Bought: {:?}", pos);
            println!("Cash left: {}", new_cash);
        }
        Err(e) => println!("Buy failed: {:?}", e),
    }

    let pos = Position::new("AAPL", 10.0, 100.0);
    match withdraw(&pos, 50.0, 120.0) {
        Ok(cash) => println!("Got ${} cash", cash),
        Err(e) => println!("Withdraw failed: {:?}", e),
    }
}
