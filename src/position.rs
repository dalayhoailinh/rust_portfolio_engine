// #[derive(Debug, Clone)]: auto-generates two traits:
// - Debug: lets us print with {:?}
// - Clone: lets us call .clone() to make a deep copy
#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_buy_price: f64,
}

// 'impl' block = methods on the struct
impl Position {
    // associated function. called as Possition::new()
    pub fn new(symbol: &str, quantity: f64, avg_buy_price: f64) -> Self {
        Self { symbol: symbol.to_string(), quantity, avg_buy_price }
    }

    pub fn total_cost(&self) -> f64 {
        self.quantity * self.avg_buy_price
    }

    pub fn market_value(&self, current_price: f64) -> f64 {
        self.quantity * current_price
    }

    pub fn unrealized_pnl(&self, current_price: f64) -> f64 {
        (current_price - self.avg_buy_price) * self.quantity
    }

    pub fn unrealized_pnl_percent(&self, current_price: f64) -> f64 {
        if self.avg_buy_price == 0.0 {
            0.0
        } else {
            ((current_price - self.avg_buy_price) / self.avg_buy_price)
                * 100.0
        }
    }
}
