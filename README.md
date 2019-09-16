# to-kana
[![Crates.io](https://img.shields.io/crates/v/to-kana)](https://crates.io/crates/to-kana)
[![docs.rs](https://docs.rs/to-kana/badge.svg)](https://docs.rs/to-kana/)
[![dependency status](https://deps.rs/repo/github/spenserblack/to-kana-rs/status.svg)](https://deps.rs/repo/github/spenserblack/to-kana-rs)
![Crates.io downloads](https://img.shields.io/crates/d/to-kana)
![Crates.io downloads of latest version](https://img.shields.io/crates/dv/to-kana)
![License](https://img.shields.io/crates/l/to-kana)


![GitHub Release Date](https://img.shields.io/github/release-date/spenserblack/to-kana-rs)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/spenserblack/to-kana-rs/latest)

Converts English lettering to Kana.
Please see the [wiki](https://github.com/spenserblack/to-kana-rs/wiki) to get supported inputs.

## Basic Usage
### As Library
#### Using `to_kana!`
Check out [docs.rs](https://docs.rs/to-kana/) to get all available formats for `to_kana!`

```rust
use to_kana::to_kana;

// こんにちは、ワールド！
let hello_world = to_kana!("{:H}{:K}", "konnichiha,", "wa-rudo!").unwrap();

// Half-width ｶﾀｶﾅ works too!
let half = to_kana!("Half-width {:K/2} works too!", "katakana").unwrap();
```

#### The Old Ways
```rust
use to_kana::ToKana;

let hello = "konnichiha,".hira().unwrap(); // Works on &str
let world = String::from("wa-rudo!").kata().unwrap(); // And String!

println!("{}{}", hello, world)); // こんにちは、ワールド！
```
```rust
use to_kana::{
    hira, // Hiragana
    kata, // Katakana
};

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
