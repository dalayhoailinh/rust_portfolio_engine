pub fn simple_moving_average(
    prices: &[f64],
    window: usize,
) -> Vec<Option<f64>> {
    if window == 0 {
        return vec![None; prices.len()];
    }

    let mut out: Vec<Option<f64>> = Vec::with_capacity(prices.len());

    let mut sum = 0.0_f64;

    for (i, &price) in prices.iter().enumerate() {
        sum += price;

        if i + 1 < window {
            out.push(None);
        } else {
            if i + 1 > window {
                sum -= prices[i - window];
            }
            out.push(Some(sum / window as f64));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "expect {b}, got {a}");
    }

    #[test]
    fn sma_window_zero_returns_all_none() {
        let out = simple_moving_average(&[1.0, 2.0, 3.0], 0);
        assert_eq!(out, vec![None, None, None]);
    }

    #[test]
    fn sma_window_3_skips_first_two_bars() {
        let prices = [1.0, 2.0, 3.0, 4.0, 5.0];
        let out = simple_moving_average(&prices, 3);

        assert_eq!(out[0], None);
        assert_eq!(out[1], None);

        assert_close(out[2].unwrap(), 2.0);
        assert_close(out[3].unwrap(), 3.0);
        assert_close(out[4].unwrap(), 4.0);
    }

    #[test]
    fn sma_window_equal_to_len_has_one_value() {
        let prices = [10.0, 20.0, 30.0];
        let out = simple_moving_average(&prices, 3);
        assert_eq!(out[0], None);
        assert_eq!(out[1], None);
        assert_close(out[2].unwrap(), 20.0); // mean(10,20,30)
    }

    #[test]
    fn sma_window_larger_than_len_is_all_none() {
        let prices = [1.0, 2.0];
        let out = simple_moving_average(&prices, 5);
        assert_eq!(out, vec![None, None]);
    }

    #[test]
    fn sma_parity_reference_values() {
        let closes = [
            170.80, 171.95, 172.50, 173.00, 173.60, // first 5
            174.80, 175.40, 176.10, 176.85, 177.55, // next 5
        ];
        let out = simple_moving_average(&closes, 5);

        for v in &out[..4] {
            assert_eq!(*v, None);
        }
        // bar 4 → mean(170.80..=173.60)
        assert_close(
            out[4].unwrap(),
            (170.80 + 171.95 + 172.50 + 173.00 + 173.60) / 5.0,
        );
        // bar 9 → mean(174.80..=177.55)
        assert_close(
            out[9].unwrap(),
            (174.80 + 175.40 + 176.10 + 176.85 + 177.55) / 5.0,
        );
    }
}
