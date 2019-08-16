# to-kana
[![Crates.io](https://img.shields.io/crates/v/to-kana)](https://crates.io/crates/to-kana)
[![docs.rs](https://docs.rs/to-kana/badge.svg)](https://docs.rs/to-kana/)
![Crates.io](https://img.shields.io/crates/d/to-kana)
![Crates.io](https://img.shields.io/crates/dv/to-kana)
![Crates.io](https://img.shields.io/crates/l/to-kana)

Converts English lettering to Kana.

## Basic Usage
### As Library
```rust
use to_kana::ToKana;

let hello = "konnichiha,".hira().unwrap(); // Works on &str
let world = String::from("wa-rudo!").kata().unwrap(); // And String!

println!("{}{}", hello, world)); // こんにちは、ワールド！
```
Or, if you prefer the old way
```rust
use to_kana::{
    hira, // Hiragana
    kata, // Katakana
};

// は is pronounced "wa" in this context, but is usually pronounced "ha"
let hello = hira("konnichiha,").unwrap();
let world = kata("wa-rudo!").unwrap();

println!("{}{}", hello, world); // こんにちは、ワールド！
```

### As Binary
```bash
# Install
make install

# Get Help
to-kana -h
to-kana hira -h
to-kana kata -h

# Convert to Hiragana
to-kana hira <STRING>

# Convert to Katakana
to-kana kata <STRING>
```
