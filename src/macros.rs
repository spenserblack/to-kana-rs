/// Functions similarly to `format!`, except it converts args to Hiragana and
/// Katakana.
///
/// # Example
///
/// ```
/// use to_kana::to_kana;
///
/// assert_eq!(
///     "ひらがな and カタカナ",
///     to_kana!("{:H} and {:K}", "hiragana", "katakana").unwrap()
/// );
/// ```
///
/// # Supported formats
/// - `:H` => Hiragana (ひらがな)
/// - `:K` => Katakana (カタカナ)
/// - `:K/2` => Half-width Katakana (ｶﾀｶﾅ)
/// - `:h` => Small Hiragana (ぁぃぅぇぉゃゅょっ)
/// - `:k` => Small Katakana (ァィゥェォャュョッ)
/// - `:k/2` => Small, Half-width Katakana (ｧｨｩｪｫｬｭｮｯ)
#[macro_export]
macro_rules! to_kana {
    ($format_str:tt, $($args:expr),* $(,)?) => ($crate::fmt::format_kana($format_str, vec![$($args),*]))
}

#[cfg(test)]
mod tests {

    #[test]
    fn to_kana_macro() {
        let s = to_kana!("hiragana: {:H}, katakana: {:K}", "konnichiha", "konbanha").unwrap();
        assert_eq!("hiragana: こんにちは, katakana: コンバンハ", s);
    }

    #[test]
    fn to_kana_macro_3_args() {
        let s = to_kana!(
            "{:H},{:K},{:k/2}",
            "konnichiha",
            "konbanha",
            "yayuyo",
        ).unwrap();
        assert_eq!("こんにちは,コンバンハ,ｬｭｮ", s);
    }
}
