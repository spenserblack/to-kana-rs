pub type Error = String;
pub type Result = std::result::Result<&'static str, Error>;

pub fn kata(s: &str) -> Result {
    let kana = match s {
        "a" => "ア",
        "i" => "イ",
        "u" => "ウ",
        "e" => "エ",
        "o" => "オ",
        "ka" => "カ",
        "ki" => "キ",
        "ku" => "ク",
        "ke" => "ケ",
        "ko" => "コ",
        "sa" => "サ",
        "si" | "shi" => "シ",
        "su" => "ス",
        "se" => "セ",
        "so" => "ソ",
        "ta" => "タ",
        "ti" | "chi" => "チ",
        "tu" | "tsu" => "ツ",
        "te" => "テ",
        "to" => "ト",
        "n" => "ン",
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
        assert_eq!(kata("ka"), Ok("カ"));
        assert_eq!(kata("ki"), Ok("キ"));
        assert_eq!(kata("ku"), Ok("ク"));
        assert_eq!(kata("ke"), Ok("ケ"));
        assert_eq!(kata("ko"), Ok("コ"));
        assert_eq!(kata("sa"), Ok("サ"));
        assert_eq!(kata("si"), Ok("シ"));
        assert_eq!(kata("su"), Ok("ス"));
        assert_eq!(kata("se"), Ok("セ"));
        assert_eq!(kata("so"), Ok("ソ"));
        assert_eq!(kata("ta"), Ok("タ"));
        assert_eq!(kata("ti"), Ok("チ"));
        assert_eq!(kata("tu"), Ok("ツ"));
        assert_eq!(kata("te"), Ok("テ"));
        assert_eq!(kata("to"), Ok("ト"));
        panic!("No N katakana");
        panic!("No H katakana");
        panic!("No M katakana");
        panic!("No Y katakana");
        panic!("No R katakana");
        panic!("No W katakana");
        assert_eq!(kata("n"), Ok("ン"));
    }
}
