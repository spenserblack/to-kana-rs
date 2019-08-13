pub type Error = String;
pub type Result = std::result::Result<String, Error>;

pub fn hira(s: &str) -> Result {
    if s.len() == 3 {
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        let next_char = chars.next().unwrap();
        if first_char == next_char {
            let main_hira = match hira(&s[1..]) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let s = format!(
                "{}{}",
                "っ",
                main_hira,
            );
            return Ok(s);
        } else {
            return Err(String::from("3-character pattern not recognized"));
        }
    }

    let kana = match s {
        "a" => "あ",
        "i" => "い",
        "u" => "う",
        "e" => "え",
        "o" => "お",
        "ka" => "か",
        "ki" => "き",
        "ku" => "く",
        "ke" => "け",
        "ko" => "こ",
        "sa" => "さ",
        "si" | "shi" => "し",
        "su" => "す",
        "se" => "せ",
        "so" => "そ",
        "ta" => "た",
        "ti" | "chi" => "ち",
        "tu" | "tsu" => "つ",
        "te" => "て",
        "to" => "と",
        "na" => "な",
        "ni" => "に",
        "nu" => "ぬ",
        "ne" => "ね",
        "no" => "の",
        "ha" => "は",
        "hi" => "ひ",
        "hu" => "ふ",
        "he" => "へ",
        "ho" => "ほ",
        "ma" => "ま",
        "mi" => "み",
        "mu" => "む",
        "me" => "め",
        "mo" => "も",
        "ya" => "や",
        "yu" => "ゆ",
        "yo" => "よ",
        "ra" => "ら",
        "ri" => "り",
        "ru" => "る",
        "re" => "れ",
        "ro" => "ろ",
        "wa" => "わ",
        "wo" => "を",
        "n" => "ん",
        _ => return Err(String::from("Pattern not recognized")),
    };
    Ok(String::from(kana))
}

pub fn kata(s: &str) -> Result {
    if s.len() == 3 {
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        let next_char = chars.next().unwrap();
        if first_char == next_char {
            let main_hira = match kata(&s[1..]) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let s = format!(
                "{}{}",
                "ッ",
                main_hira,
            );
            return Ok(s);
        } else {
            return Err(String::from("3-character pattern not recognized"));
        }
    }

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
        "na" => "ナ",
        "ni" => "ニ",
        "nu" => "ヌ",
        "ne" => "ネ",
        "no" => "ノ",
        "ha" => "ハ",
        "hi" => "ヒ",
        "hu" | "fu" => "フ",
        "he" => "ヘ",
        "ho" => "ホ",
        "ma" => "マ",
        "mi" => "ミ",
        "mu" => "ム",
        "me" => "メ",
        "mo" => "モ",
        "ya" => "ヤ",
        "yu" => "ユ",
        "yo" => "ヨ",
        "ra" => "ラ",
        "ri" => "リ",
        "ru" => "ル",
        "re" => "レ",
        "ro" => "ロ",
        "wa" => "ワ",
        "wo" => "ヲ",
        "n" => "ン",
        _ => return Err(String::from("Pattern not recognized")),
    };
    Ok(String::from(kana))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hiragana() {
        assert_eq!(hira("a"), Ok(String::from("あ")));
        assert_eq!(hira("i"), Ok(String::from("い")));
        assert_eq!(hira("u"), Ok(String::from("う")));
        assert_eq!(hira("e"), Ok(String::from("え")));
        assert_eq!(hira("o"), Ok(String::from("お")));
        assert_eq!(hira("ka"), Ok(String::from("か")));
        assert_eq!(hira("ki"), Ok(String::from("き")));
        assert_eq!(hira("ku"), Ok(String::from("く")));
        assert_eq!(hira("ke"), Ok(String::from("け")));
        assert_eq!(hira("ko"), Ok(String::from("こ")));
        assert_eq!(hira("sa"), Ok(String::from("さ")));
        assert_eq!(hira("si"), Ok(String::from("し")));
        assert_eq!(hira("su"), Ok(String::from("す")));
        assert_eq!(hira("se"), Ok(String::from("せ")));
        assert_eq!(hira("so"), Ok(String::from("そ")));
        assert_eq!(hira("ta"), Ok(String::from("た")));
        assert_eq!(hira("ti"), Ok(String::from("ち")));
        assert_eq!(hira("tu"), Ok(String::from("つ")));
        assert_eq!(hira("te"), Ok(String::from("て")));
        assert_eq!(hira("to"), Ok(String::from("と")));
        assert_eq!(hira("na"), Ok(String::from("な")));
        assert_eq!(hira("ni"), Ok(String::from("に")));
        assert_eq!(hira("nu"), Ok(String::from("ぬ")));
        assert_eq!(hira("ne"), Ok(String::from("ね")));
        assert_eq!(hira("no"), Ok(String::from("の")));
        assert_eq!(hira("ha"), Ok(String::from("は")));
        assert_eq!(hira("hi"), Ok(String::from("ひ")));
        assert_eq!(hira("hu"), Ok(String::from("ふ")));
        assert_eq!(hira("he"), Ok(String::from("へ")));
        assert_eq!(hira("ho"), Ok(String::from("ほ")));
        assert_eq!(hira("ma"), Ok(String::from("ま")));
        assert_eq!(hira("mi"), Ok(String::from("み")));
        assert_eq!(hira("mu"), Ok(String::from("む")));
        assert_eq!(hira("me"), Ok(String::from("め")));
        assert_eq!(hira("mo"), Ok(String::from("も")));
        assert_eq!(hira("ya"), Ok(String::from("や")));
        assert_eq!(hira("yu"), Ok(String::from("ゆ")));
        assert_eq!(hira("yo"), Ok(String::from("よ")));
        assert_eq!(hira("ra"), Ok(String::from("ら")));
        assert_eq!(hira("ri"), Ok(String::from("り")));
        assert_eq!(hira("ru"), Ok(String::from("る")));
        assert_eq!(hira("re"), Ok(String::from("れ")));
        assert_eq!(hira("ro"), Ok(String::from("ろ")));
        assert_eq!(hira("wa"), Ok(String::from("わ")));
        assert_eq!(hira("wo"), Ok(String::from("を")));
        assert_eq!(hira("n"), Ok(String::from("ん")));
    }

    #[test]
    fn hiragana_little_tsu() {
        assert_eq!(hira("tte"), Ok(String::from("って")));
    }

    #[test]
    fn to_katakana() {
        assert_eq!(kata("a"), Ok(String::from("ア")));
        assert_eq!(kata("i"), Ok(String::from("イ")));
        assert_eq!(kata("u"), Ok(String::from("ウ")));
        assert_eq!(kata("e"), Ok(String::from("エ")));
        assert_eq!(kata("o"), Ok(String::from("オ")));
        assert_eq!(kata("ka"), Ok(String::from("カ")));
        assert_eq!(kata("ki"), Ok(String::from("キ")));
        assert_eq!(kata("ku"), Ok(String::from("ク")));
        assert_eq!(kata("ke"), Ok(String::from("ケ")));
        assert_eq!(kata("ko"), Ok(String::from("コ")));
        assert_eq!(kata("sa"), Ok(String::from("サ")));
        assert_eq!(kata("si"), Ok(String::from("シ")));
        assert_eq!(kata("su"), Ok(String::from("ス")));
        assert_eq!(kata("se"), Ok(String::from("セ")));
        assert_eq!(kata("so"), Ok(String::from("ソ")));
        assert_eq!(kata("ta"), Ok(String::from("タ")));
        assert_eq!(kata("ti"), Ok(String::from("チ")));
        assert_eq!(kata("tu"), Ok(String::from("ツ")));
        assert_eq!(kata("te"), Ok(String::from("テ")));
        assert_eq!(kata("to"), Ok(String::from("ト")));
        assert_eq!(kata("na"), Ok(String::from("ナ")));
        assert_eq!(kata("ni"), Ok(String::from("ニ")));
        assert_eq!(kata("nu"), Ok(String::from("ヌ")));
        assert_eq!(kata("ne"), Ok(String::from("ネ")));
        assert_eq!(kata("no"), Ok(String::from("ノ")));
        assert_eq!(kata("ha"), Ok(String::from("ハ")));
        assert_eq!(kata("hi"), Ok(String::from("ヒ")));
        assert_eq!(kata("hu"), Ok(String::from("フ")));
        assert_eq!(kata("he"), Ok(String::from("ヘ")));
        assert_eq!(kata("ho"), Ok(String::from("ホ")));
        assert_eq!(kata("ma"), Ok(String::from("マ")));
        assert_eq!(kata("mi"), Ok(String::from("ミ")));
        assert_eq!(kata("mu"), Ok(String::from("ム")));
        assert_eq!(kata("me"), Ok(String::from("メ")));
        assert_eq!(kata("mo"), Ok(String::from("モ")));
        assert_eq!(kata("ya"), Ok(String::from("ヤ")));
        assert_eq!(kata("yu"), Ok(String::from("ユ")));
        assert_eq!(kata("yo"), Ok(String::from("ヨ")));
        assert_eq!(kata("ra"), Ok(String::from("ラ")));
        assert_eq!(kata("ri"), Ok(String::from("リ")));
        assert_eq!(kata("ru"), Ok(String::from("ル")));
        assert_eq!(kata("re"), Ok(String::from("レ")));
        assert_eq!(kata("ro"), Ok(String::from("ロ")));
        assert_eq!(kata("wa"), Ok(String::from("ワ")));
        assert_eq!(kata("wo"), Ok(String::from("ヲ")));
        assert_eq!(kata("n"), Ok(String::from("ン")));
    }

    #[test]
    fn katakana_little_tsu() {
        assert_eq!(kata("tte"), Ok(String::from("ッテ")));
    }
}
