#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_buy_price: f64,
}

impl Position {
    pub fn new(symbol: &str, quantity: f64, avg_buy_price: f64) -> Self {
        Position { symbol: symbol.to_string(), quantity, avg_buy_price }
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
