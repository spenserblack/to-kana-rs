/// Functions similarly to `format!`, except it converts args to Hiragana and
/// Katakana.
///
/// # Example
///
/// ```
/// let s = to_kana!("konnichiha in hiragana: {:h}, konbanha in katakana: {:k}", "konnichiha", "konbanha");
/// assert_eq!("konnichiha in hiragana: こんにちは, konbanha in katakana: コンバンハ", s);
/// ```
#[macro_export]
macro_rules! to_kana {
    ($format_str:tt, $($args:expr),*) => (panic!("Macro not working yet!"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_kana_macro() {
        let s = to_kana!("konnichiha in hiragana: {:h}, konbanha in katakana: {:k}", "konnichiha", "konbanha");
        // assert_eq!("konnichiha in hiragana: こんにちは, konbanha in katakana: コンバンハ", s);
    }
}
