use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use cashify::{convert};

#[derive(Serialize, Deserialize)]
struct Rates<'a>{
    base: &'a str,
    rates: HashMap<&'a str, f64>
}

fn main() -> Result<()> {
      let data = r#"{
          "base": "EUR",
          "rates": {
              "GBP": 0.92,
              "EUR": 1
          }
      }"#;
  
      let r: Rates = serde_json::from_str(data)?;

      println!("The result is: {}", convert(10.0, "EUR", "GBP", r.base, r.rates));
      Ok(())
}