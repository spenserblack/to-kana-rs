pub type Error = String;
pub type Result = std::result::Result<&'static str, Error>;

pub fn kata(s: &str) -> Result {
    let kana = match s {
        "a" => "ア",
        "i" => "イ",
        "u" => "ウ",
        "e" => "エ",
        "o" => "オ",
        _ => return Err(String::from("Pattern not recognized")),
    };
    Ok(kana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_katakana() {
        assert_eq!(kata("a"), Ok("ア"));
        assert_eq!(kata("i"), Ok("イ"));
        assert_eq!(kata("u"), Ok("ウ"));
        assert_eq!(kata("e"), Ok("エ"));
        assert_eq!(kata("o"), Ok("オ"));
        panic!("No K katakana");
        panic!("No S katakana");
        panic!("No T katakana");
        panic!("No N katakana");
        panic!("No H katakana");
        panic!("No M katakana");
        panic!("No Y katakana");
        panic!("No R katakana");
        panic!("No W katakana");
    }
}
