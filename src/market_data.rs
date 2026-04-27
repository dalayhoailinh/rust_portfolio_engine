use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OhlcvBar {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub async fn load_ohlcv_csv(path: &str) -> Result<Vec<OhlcvBar>> {
    let body = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("failed to read CSV at {path}"))?;

    let mut reader = csv::Reader::from_reader(body.as_bytes());

    let bars: Vec<OhlcvBar> = reader
        .deserialize()
        .collect::<Result<_, _>>()
        .with_context(|| format!("failed to parse CSV at {path}"))?;

    Ok(bars)
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
            serde_json::from_str(raw).expect("parse EOHDH bar");
        assert_eq!(bar.date, "2026-04-15");
        assert_eq!(bar.volume, 42_531_900);
    }
}
