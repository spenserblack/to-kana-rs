use lazy_static::lazy_static;
use regex::Regex;

use super::{
    ToKana,
    HalfWidth,
    SmallKana,
    Result,
};

pub fn format_kana(format_str: &str, args: Vec<&str>) -> Result {
    lazy_static! {
        static ref FORMAT_FINDER: Regex = Regex::new(r"\{:.*\}").unwrap();
    }
    let mut index: usize = 0;
    let mut out = String::from(format_str);
    println!("Format finder: {:?}", FORMAT_FINDER.find(&out));
    panic!("Args not implemented: {:?}", args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_kana_test() {
        let s = format_kana("hiragana: {:H}, katakana: {:K}", vec!["konnichiha", "konbanha"]).unwrap();
        assert_eq!("hiragana: こんにちは, katakana: コンバンハ", s);
    }

    #[test]
    fn format_small_kana_test() {
        let s = format_kana("hiragana: {:h}, katakana: {:k}", vec!["yo", "yo"]).unwrap();
        assert_eq!("hiragana: ょ, katakana: ョ", s);
    }

    #[test]
    fn format_half_width_test() {
        let s = format_kana("katakana: {:K/2}", vec!["konbanha"]).unwrap();
        assert_eq!("katakana: ｺﾝﾊﾞﾝﾊ", s);
    }

    #[test]
    fn format_half_width_small_test() {
        let s = format_kana("katakana: {:k/2}", vec!["yo"]).unwrap();
        assert_eq!("katakana: ｮ", s);
    }
}
