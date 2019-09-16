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
#[macro_export]
macro_rules! to_kana {
    ($format_str:tt, $($args:expr),*) => ($crate::fmt::format_kana($format_str, vec![$($args),*]))
}

#[cfg(test)]
mod tests {

    #[test]
    fn to_kana_macro() {
        let s = to_kana!("hiragana: {:h}, katakana: {:k}", "konnichiha", "konbanha").unwrap();
        assert_eq!("hiragana: こんにちは, katakana: コンバンハ", s);
    }
}
