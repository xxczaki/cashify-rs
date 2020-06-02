# Cashify 💸

> Lightweight currency conversion library.

[![Build Status](https://travis-ci.org/xxczaki/cashify-rs.svg?branch=master)](https://travis-ci.org/xxczaki/cashify-rs)
[![Crates](http://meritbadge.herokuapp.com/cashify)](https://crates.io/crates/cashify)
[![cashify at docs.rs](https://docs.rs/cashify/badge.svg)](https://docs.rs/cashify)

This Rust crate is a port of the [Cashify](https://github.com/xxczaki/cashify/) npm package from the same author. API is not the same.

## Documentation

- [Full API documentation](https://docs.rs/cashify)

## Installation

Simply add the corresponding entry to your `Cargo.toml` dependency list:

```toml
[dependencies]
cashify = "0.1"
```

## Usage

The following example uses [Serde JSON](https://github.com/serde-rs/json) as strongly typed data structures. Instead of manually specifying rates, you can obtain them from an API, like [Exchange Rates API](https://exchangeratesapi.io/).

```rust
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
```

## Roadmap

The goal is to try and implement as much features from the original Cashify as possible.

- [x] `convert` Function
- [ ] Constructor
- [ ] Parsing

## Related projects

- [Cashify (TypeScript)](https://github.com/xxczaki/cashify/)

## License

MIT
