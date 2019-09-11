//! English lettering to Kana converter.
//! Please see the [wiki](https://github.com/spenserblack/to-kana-rs/wiki) to
//! view supported inputs.
//!
//! # Examples
//!
//! ```
//! use to_kana::ToKana;
//!
//! let hello = "konnichiha,".hira().unwrap(); // Works on &str
//! let world = String::from("wa-rudo!").kata().unwrap(); // And String!
//!
//! assert_eq!("こんにちは、ワールド！", format!("{}{}", hello, world));
//! ```
//!
//! ```
//! use to_kana::{
//!     hira, // Hiragana
//!     kata, // Katakana
//! };
//!
//! let hello = hira("konnichiha,").unwrap();
//! let world = kata("wa-rudo!").unwrap();
//!
//! assert_eq!("こんにちは、ワールド！", format!("{}{}", hello, world));
//! ```
//!
//! ```
//! use to_kana::{
//!     ToKana,
//!     SmallKana,
//!     HalfWidth,
//! };
//!
//! assert_eq!("ｬ", "ya".kata().small().half_width().unwrap());
//! ```

pub use to_kana_trait::ToKana;
pub use half_width::HalfWidth;
pub use hiragana::hira;
pub use katakana::kata;
pub use small_kana::SmallKana;

mod to_kana_trait;
mod half_width;
mod hiragana;
mod katakana;
mod small_kana;

/// The type inside `Result::Err` returned by this library's functions
pub type Error = String;

/// The Result type returned by this library's functions
pub type Result = std::result::Result<String, Error>;

const KATAKANA_SOFTENED_DIACRITICS: [char;21] = [
    'ガ', 'ギ', 'グ', 'ゲ', 'ゴ',
    'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ',
    'ダ', 'ヂ', 'ヅ', 'デ', 'ド',
    'バ', 'ビ', 'ブ', 'ベ', 'ボ',
    'ヴ',
];

const KATAKANA_HARDENED_DIACRITICS: [char;5] = [
    'パ', 'ピ', 'プ', 'ペ', 'ポ',
];

fn has_softened_diacritic(c: &char) -> bool {
    KATAKANA_SOFTENED_DIACRITICS.contains(c)
}

fn has_hardened_diacritic(c: &char) -> bool {
    KATAKANA_HARDENED_DIACRITICS.contains(c)
}

fn syllable_divider<'a>(s: &'a str) -> Vec<&'a str> {
    const VOWELS: [&str;6] = [
        "a",
        "e",
        "i",
        "o",
        "u",
        "y", // Sometimes
    ];

    let mut vec = Vec::new();
    let mut start_index = 0;
    for (end_index, c) in s.char_indices() {
        match c {
            '-' | 'a' | 'e' | 'i' | 'o' | 'u' | ',' | '.' | '!' | '?' |
            // TODO Make this prettier
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'=> {
                vec.push(&s[start_index..=end_index]);
                start_index = end_index + 1;
            },
            '\'' => {
                // Works as long as preceding character is 'n'
                // If it wasn't, then it's an invalid pattern
                start_index = end_index + 1;
            },
            'n' => {
                // if n and next char is not vowel, push n and continue
                match s.get(end_index + 1..=end_index + 1) {
                    Some(v) if VOWELS.contains(&v) => {},
                    _ => {
                        vec.push("n");
                        start_index = end_index + 1;
                    },
                }
            }
            _ => {},
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syllable_division() {
        let syllables = syllable_divider("konbanha");

        assert_eq!(vec!["ko", "n", "ba", "n", "ha"], syllables);
    }

    #[test]
    fn comma() {
        assert_eq!(Ok(String::from("、")), hira(","));
        assert_eq!(Ok(String::from("、")), kata(","));
    }

    #[test]
    fn period() {
        assert_eq!(Ok(String::from("。")), hira("."));
        assert_eq!(Ok(String::from("。")), kata("."));
    }

    #[test]
    fn exclamation_point() {
        assert_eq!(Ok(String::from("！")), hira("!"));
        assert_eq!(Ok(String::from("！")), kata("!"));
    }

    #[test]
    fn question_mark() {
        assert_eq!(Ok(String::from("？")), hira("?"));
        assert_eq!(Ok(String::from("？")), kata("?"));
    }
}
