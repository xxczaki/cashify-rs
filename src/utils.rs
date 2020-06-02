use std::collections::HashMap;

pub fn get_rate(base: &str, rates: HashMap<&str, f64>, from: &str, to: &str) -> f64 {
    // If `from` equals `base`, return the basic exchange rate for the `to` currency
    if from == base {
        return rates[to];
    }

    // If `to` equals `base`, return the basic inverse rate of the `from` currency
    if to == base {
        return 1.0 / rates[from];
    }

    // Otherwise, return the `to` rate multipled by the inverse of the `from` rate to get the relative exchange rate between the two currencies.
    rates[to] * (1.0 / rates[from])
}
