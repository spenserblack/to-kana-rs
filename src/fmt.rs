use regex::Regex;

use super::{
    ToKana,
    HalfWidth,
    SmallKana,
    Result,
};

pub fn format_kana(format_str: &str, args: Vec<&str>) -> Result {
    let format_finder = Regex::new(r"\{:[A-z]\}").unwrap(); // TODO Use lazy_static to improve performance
    let mut index: usize = 0;
    let mut out = String::from(format_str);
    println!("Format finder: {:?}", format_finder.find(&out));
    panic!("Args not implemented: {:?}", args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_kana_test() {
        let s = format_kana("konnichiha in hiragana: {:h}, konbanha in katakana: {:k}", vec!["konnichiha", "konbanha"]).unwrap();
        assert_eq!("konnichiha in hiragana: こんにちは, konbanha in katakana: コンバンハ", s);
    }
}
