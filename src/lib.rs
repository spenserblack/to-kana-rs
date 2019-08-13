pub type Error = String;
pub type Result = std::result::Result<&'static str, Error>;

pub fn hira(s: &str) -> Result {
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
    Ok(kana)
}

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
    Ok(kana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hiragana() {
        assert_eq!(hira("a"), Ok("あ"));
        assert_eq!(hira("i"), Ok("い"));
        assert_eq!(hira("u"), Ok("う"));
        assert_eq!(hira("e"), Ok("え"));
        assert_eq!(hira("o"), Ok("お"));
        assert_eq!(hira("ka"), Ok("か"));
        assert_eq!(hira("ki"), Ok("き"));
        assert_eq!(hira("ku"), Ok("く"));
        assert_eq!(hira("ke"), Ok("け"));
        assert_eq!(hira("ko"), Ok("こ"));
        assert_eq!(hira("sa"), Ok("さ"));
        assert_eq!(hira("si"), Ok("し"));
        assert_eq!(hira("su"), Ok("す"));
        assert_eq!(hira("se"), Ok("せ"));
        assert_eq!(hira("so"), Ok("そ"));
        assert_eq!(hira("ta"), Ok("た"));
        assert_eq!(hira("ti"), Ok("ち"));
        assert_eq!(hira("tu"), Ok("つ"));
        assert_eq!(hira("te"), Ok("て"));
        assert_eq!(hira("to"), Ok("と"));
        assert_eq!(hira("na"), Ok("な"));
        assert_eq!(hira("ni"), Ok("に"));
        assert_eq!(hira("nu"), Ok("ぬ"));
        assert_eq!(hira("ne"), Ok("ね"));
        assert_eq!(hira("no"), Ok("の"));
        assert_eq!(hira("ha"), Ok("は"));
        assert_eq!(hira("hi"), Ok("ひ"));
        assert_eq!(hira("hu"), Ok("ふ"));
        assert_eq!(hira("he"), Ok("へ"));
        assert_eq!(hira("ho"), Ok("ほ"));
        assert_eq!(hira("ma"), Ok("ま"));
        assert_eq!(hira("mi"), Ok("み"));
        assert_eq!(hira("mu"), Ok("む"));
        assert_eq!(hira("me"), Ok("め"));
        assert_eq!(hira("mo"), Ok("も"));
        assert_eq!(hira("ya"), Ok("や"));
        assert_eq!(hira("yu"), Ok("ゆ"));
        assert_eq!(hira("yo"), Ok("よ"));
        assert_eq!(hira("ra"), Ok("ら"));
        assert_eq!(hira("ri"), Ok("り"));
        assert_eq!(hira("ru"), Ok("る"));
        assert_eq!(hira("re"), Ok("れ"));
        assert_eq!(hira("ro"), Ok("ろ"));
        assert_eq!(hira("wa"), Ok("わ"));
        assert_eq!(hira("wo"), Ok("を"));
        assert_eq!(hira("n"), Ok("ん"));
    }

    #[test]
    fn hiragana_little_tsu() {
        assert_eq!(hira("tte"), Ok("って"));
    }

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
        assert_eq!(kata("na"), Ok("ナ"));
        assert_eq!(kata("ni"), Ok("ニ"));
        assert_eq!(kata("nu"), Ok("ヌ"));
        assert_eq!(kata("ne"), Ok("ネ"));
        assert_eq!(kata("no"), Ok("ノ"));
        assert_eq!(kata("ha"), Ok("ハ"));
        assert_eq!(kata("hi"), Ok("ヒ"));
        assert_eq!(kata("hu"), Ok("フ"));
        assert_eq!(kata("he"), Ok("ヘ"));
        assert_eq!(kata("ho"), Ok("ホ"));
        assert_eq!(kata("ma"), Ok("マ"));
        assert_eq!(kata("mi"), Ok("ミ"));
        assert_eq!(kata("mu"), Ok("ム"));
        assert_eq!(kata("me"), Ok("メ"));
        assert_eq!(kata("mo"), Ok("モ"));
        assert_eq!(kata("ya"), Ok("ヤ"));
        assert_eq!(kata("yu"), Ok("ユ"));
        assert_eq!(kata("yo"), Ok("ヨ"));
        assert_eq!(kata("ra"), Ok("ラ"));
        assert_eq!(kata("ri"), Ok("リ"));
        assert_eq!(kata("ru"), Ok("ル"));
        assert_eq!(kata("re"), Ok("レ"));
        assert_eq!(kata("ro"), Ok("ロ"));
        assert_eq!(kata("wa"), Ok("ワ"));
        assert_eq!(kata("wo"), Ok("ヲ"));
        assert_eq!(kata("n"), Ok("ン"));
    }

    #[test]
    fn katakana_little_tsu() {
        assert_eq!(kata("tte"), Ok("ッテ"));
    }
}
