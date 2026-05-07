use crate::position::Position;

#[derive(Debug)]
pub enum TradingError {
    InsufficientCash { needed: f64, have: f64 },
    InsufficientQuantity { requested: f64, owned: f64 },
    SymbolNotFound(String),
}

pub fn buy(
    cash: f64,
    existing: Option<&Position>,
    symbol: &str,
    quantity: f64,
    price: f64,
) -> Result<(Position, f64), TradingError> {
    let cost = quantity * price;
    if cost > cash {
        return Err(TradingError::InsufficientCash {
            needed: cost,
            have: cash,
        });
    }

    // match: null check
    let new_position = match existing {
        Some(p) => {
            let total_qty = p.quantity + quantity;
            let total_cost = p.total_cost() + cost;
            Position::new(symbol, total_qty, total_cost / total_qty)
        }
        None => Position::new(symbol, quantity, price),
    };

    let new_cash = cash - cost;
    Ok((new_position, new_cash))
}

pub fn withdraw(
    position: &Position,
    quantity: f64,
    current_price: f64,
) -> Result<f64, TradingError> {
    if quantity > position.quantity {
        return Err(TradingError::InsufficientQuantity {
            requested: quantity,
            owned: position.quantity,
        });
    }
    let cash_received = quantity * current_price;
    Ok(cash_received)
}
