pub fn format_kana(format_str: &str, args: Vec<&str>) -> String {
    panic!("Args not implemented: {:?}", args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_kana_test() {
        let s = format_kana("konnichiha in hiragana: {:h}, konbanha in katakana: {:k}", vec!["konnichiha", "konbanha"]);
        assert_eq!("konnichiha in hiragana: こんにちは, konbanha in katakana: コンバンハ", s);
    }
}
