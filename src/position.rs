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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_cost_is_qty_times_avg() {
        let p = Position::new("AAP:", 10.0, 100.0);
        assert_eq!(p.total_cost(), 1000.0);
    }

    #[test]
    fn pnl_is_positive_when_price_above_avg() {
        let p = Position::new("AAP:", 10.0, 100.0);
        assert_eq!(p.unrealized_pnl(110.0), 100.0);
    }

    #[test]
    fn pnl_is_negative_when_price_below_avg() {
        let p = Position::new("AAPL", 10.0, 100.0);
        assert_eq!(p.unrealized_pnl(95.0), -50.0);
    }

    #[test]
    fn pnl_percent_is_zero_when_avg_price_is_zero() {
        let p = Position::new("X", 5.0, 0.0);
        assert_eq!(p.unrealized_pnl_percent(100.0), 0.0);
    }
}
