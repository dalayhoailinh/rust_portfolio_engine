// serde: serialization/deserialization library for Rust
// - Serialize: convert our structs into formats like JSON
// - Deserialize: convert from formats like JSON back into our structs
use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)] auto-generates traits for our struct:
// - Debug: lets us print with {:?}
// - Clone: lets us call .clone() to make a deep copy
// - Serialize: lets us convert our struct into formats like JSON
// - Deserialize: lets us convert from formats like JSON back into our struct
// - PartialEq: lets us use the == operator
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OhlcvBar {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip_preserves_every_field() {
        let original = OhlcvBar {
            date: "2026-04-15".to_string(),
            open: 187.10,
            high: 188.40,
            low: 186.55,
            close: 187.95,
            volume: 42_531_900,
        };

        let json = serde_json::to_string(&original).expect("serialize");

        let parsed: OhlcvBar =
            serde_json::from_str(&json).expect("deserialize");

        assert_eq!(parsed, original);
    }

    #[test]
    fn parses_realistic_eodhd_shape() {
        let raw = r#"{
            "date": "2026-04-15",
            "open": 187.10,
            "high": 188.40,
            "low":  186.55,
            "close": 187.95,
            "volume": 42531900
        }"#;

        let bar: OhlcvBar =
            serde_json::from_str(raw).expect("parse EODHD bar");
        assert_eq!(bar.date, "2026-04-15");
        assert_eq!(bar.volume, 42_531_900);
    }
}
