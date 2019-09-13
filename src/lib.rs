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

pub use half_width::HalfWidth;
pub use hiragana::hira;
pub use katakana::kata;
pub use small_kana::SmallKana;
pub use to_kana_trait::ToKana;

mod half_width;
mod hiragana;
mod katakana;
mod small_kana;
mod to_kana_trait;

/// The type inside `Result::Err` returned by this library's functions
pub type Error = String;

/// The Result type returned by this library's functions
pub type Result = std::result::Result<String, Error>;

const KATAKANA_SOFTENED_DIACRITICS: [char; 21] = [
    'ガ', 'ギ', 'グ', 'ゲ', 'ゴ', 'ザ', 'ジ', 'ズ', 'ゼ', 'ゾ', 'ダ', 'ヂ', 'ヅ',
    'デ', 'ド', 'バ', 'ビ', 'ブ', 'ベ', 'ボ', 'ヴ',
];

const KATAKANA_HARDENED_DIACRITICS: [char; 5] = ['パ', 'ピ', 'プ', 'ペ', 'ポ'];

fn has_softened_diacritic(c: char) -> bool {
    KATAKANA_SOFTENED_DIACRITICS.contains(&c)
}

fn has_hardened_diacritic(c: char) -> bool {
    KATAKANA_HARDENED_DIACRITICS.contains(&c)
}

fn unexpected_char_error(i: usize, c: char) -> Error {
    format!("Unexpected char at {}: {}", i, c)
}

fn unexpected_end_of_string() -> Error {
    String::from("Unexpected end of string")
}

#[cfg(test)]
mod tests {
    use super::*;

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
