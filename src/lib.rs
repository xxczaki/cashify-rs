use std::collections::HashMap;

mod utils;

#[allow(clippy::implicit_hasher)]
pub fn convert(amount: f64, from: &str, to: &str, base: &str, rates: HashMap<&str, f64>) -> f64 {
    (amount * 100.0) * utils::get_rate(base, rates, from, to) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::Result;

    #[derive(Serialize, Deserialize)]
    struct Rates<'a> {
        base: &'a str,
        rates: HashMap<&'a str, f64>,
    }

    #[test]
    fn basic_conversion() -> Result<()> {
        let data = r#"{
            "base": "EUR",
            "rates": {
                "GBP": 0.92,
                "USD": 1.12
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(
            convert(12.0, "USD", "GBP", r.base, r.rates),
            9.857142857142856
        );
        Ok(())
    }

    #[test]
    fn from_eq_base() -> Result<()> {
        let data = r#"{
            "base": "EUR",
            "rates": {
                "GBP": 0.92,
                "EUR": 1
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(convert(10.0, "EUR", "GBP", r.base, r.rates), 9.2);
        Ok(())
    }

    #[test]
    fn to_eq_base() -> Result<()> {
        let data = r#"{
            "base": "EUR",
            "rates": {
                "GBP": 0.92,
                "EUR": 1,
                "USD": 1.12
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(
            convert(10.0, "GBP", "EUR", r.base, r.rates),
            10.869565217391303
        );
        Ok(())
    }

    #[test]
    fn from_eq_to() -> Result<()> {
        let data = r#"{
            "base": "EUR",
            "rates": {
                "USD": 1.12
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(convert(10.0, "USD", "USD", r.base, r.rates), 10.0);
        Ok(())
    }

    #[test]
    fn from_eq_to_diff_base() -> Result<()> {
        let data = r#"{
            "base": "USD",
            "rates": {
                "EUR": 1
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(convert(10.0, "EUR", "EUR", r.base, r.rates), 10.0);
        Ok(())
    }

    #[test]
    fn amount_zero() -> Result<()> {
        let data = r#"{
            "base": "USD",
            "rates": {
                "GBP": 0.92,
                "EUR": 1,
                "USD": 1.12
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(convert(0.0, "USD", "GBP", r.base, r.rates), 0.0);
        Ok(())
    }

    #[test]
    fn no_base() -> Result<()> {
        let data = r#"{
            "base": "EUR",
            "rates": {
                "GBP": 0.92,
                "USD": 1.12
            }
        }"#;

        let r: Rates = serde_json::from_str(data)?;

        assert_eq!(convert(10.0, "EUR", "GBP", r.base, r.rates), 9.2);
        Ok(())
    }
}
