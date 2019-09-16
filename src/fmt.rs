use lazy_static::lazy_static;
use regex::{
    Captures,
    Regex,
};

use super::{
    ToKana,
    HalfWidth,
    SmallKana,
    Result,
};

/// Uses a format string to convert arguments to their Kana counterparts.
///
/// It is recommended that you use the [`to_kana!`] macro instead, which will
/// call this function.
///
/// [`to_kana!`]: ../macro.to_kana.html
pub fn format_kana(format_str: &str, args: Vec<&str>) -> Result {
    lazy_static! {
        static ref FORMAT_FINDER: Regex = Regex::new(r"\{:[A-z]+(/2)?\}").unwrap();
    }
    let mut index: usize = 0;
    let mut error: Option<Result> = None;
    let out = FORMAT_FINDER.replace_all(format_str, |caps: &Captures| {
        let input = if let Some(s) = args.get(index) {
            index += 1;
            s
        } else {
            error = Some(Err(format!("Too few arguments: {}", index)));
            return String::new();
        };
        let conversion_attempt = match &caps[0] {
            "{:H}" => input.hira(),
            "{:K}" => input.kata(),
            "{:h}" => input.hira().small(),
            "{:k}" => input.kata().small(),
            "{:K/2}" => input.kata().half_width(),
            "{:k/2}" => input.kata().small().half_width(),
            s => {
                error = Some(Err(format!("Kana formatter not found: {}", s)));
                return String::new();
            }
        };
        match conversion_attempt {
            Err(e) => {
                error = Some(Err(e));
                return String::new();
            }
            Ok(s) => s,
        }
    });
    if let Some(error) = error {
        error
    } else {
        Ok(out.to_string())
    }
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
