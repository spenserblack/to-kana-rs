# to-kana
[![Crates.io](https://img.shields.io/crates/v/to-kana)](https://crates.io/crates/to-kana)
[![docs.rs](https://docs.rs/to-kana/badge.svg)](https://docs.rs/to-kana/)
![Crates.io](https://img.shields.io/crates/d/to-kana)
![Crates.io](https://img.shields.io/crates/l/to-kana)

Converts English lettering to Kana.
Currently functions on a per-syllable basis, including っ or ッ if needed.

## Basic Usage
```rust
use to_kana::{
    hira, // Hiragana
    kata, // Katakana
};

// は is pronounced "wa" in this context, but is usually pronounced "ha"
let hello = hira("konnichiha").unwrap(); // こんにちは
let world = kata("wa-rudo").unwrap(); // ワールド

println!("{}{}", hello, world);
```
