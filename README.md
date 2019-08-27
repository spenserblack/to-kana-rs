# to-kana
[![Crates.io](https://img.shields.io/crates/v/to-kana)](https://crates.io/crates/to-kana)
[![docs.rs](https://docs.rs/to-kana/badge.svg)](https://docs.rs/to-kana/)
![Crates.io downloads](https://img.shields.io/crates/d/to-kana)
![Crates.io downloads of latest version](https://img.shields.io/crates/dv/to-kana)
![License](https://img.shields.io/crates/l/to-kana)

![GitHub Release Date](https://img.shields.io/github/release-date/spenserblack/to-kana-rs)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/spenserblack/to-kana-rs/latest)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/spenserblack/to-kana-rs)
![GitHub repo size](https://img.shields.io/github/repo-size/spenserblack/to-kana-rs)

Converts English lettering to Kana.
Please see the [wiki](github.com/spenserblack/to-kana-rs/wiki) to get supported inputs.

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

What if you don't want to just convert to Kana? What if you want to convert to small, half-width Katakana? You can!
```rust
use to_kana::{
    ToKana,
    SmallKana,
    HalfWidth,
};

// Not to be confused with "ヤ", "ャ" or "ﾔ"
assert_eq!("ｬ", "ya".kata().small().half_width().unwrap());
```

### From Command Line
```bash
# Install
cargo install to-kana

# Get Help
to-kana -h
to-kana hira -h
to-kana kata -h

# Convert to Hiragana
to-kana hira <STRING>

# Convert to Katakana
to-kana kata <STRING>

# Convert to small, half-width Katakana
to-kana --small kata yo --half
```
