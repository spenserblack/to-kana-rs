# to-kana
Converts English lettering to Kana.
Currently functions on a per-syllable basis, including っ or ッ if needed.

## Basic Usage
```rust
use to_kana::{
    hira, // Hiragana
    kata, // Katakana
};

let ko = hira("ko").unwrap();
let n = hira("n").unwrap();
let ni = hira("ni").unwrap();
let chi = hira("ti").unwrap(); // same as hira("chi")

// は is prounounced "wa" in "こんにちは", but is normally pronounced "ha"
let wa = hira("ha").unwrap();

println!("{}{}{}{}{}", ko, n, ni, chi, wa); // こんにちは
```
