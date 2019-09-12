use super::{
    Result,
    SmallKana,
    syllable_divider,
    unexpected_char_error,
    unexpected_end_of_string,
};

use k::k;
use g::g;
use small::small;

mod k;
mod g;
mod s;
mod z;
mod j;
mod t;
mod c;
mod small;

/// Converts an English alphabet string to Hiragana
///
/// # Example
///
/// ```
/// use to_kana::hira;
///
/// let si = hira("si").unwrap();
/// let shi = hira("shi").unwrap();
/// let ssi = hira("ssi").unwrap();
/// let sshi = hira("sshi").unwrap();
///
/// assert_eq!("し", si);
/// assert_eq!("し", shi);
/// assert_eq!("っし", ssi);
/// assert_eq!("っし", sshi);
/// ```
pub fn hira(s: &str) -> Result {
    let mut hiragana = String::new();
    let mut characters = s.chars().enumerate();

    while let Some((i, c)) = characters.next() {
        match c {
            ',' => hiragana.push('、'),
            '.' => hiragana.push('。'),
            '!' => hiragana.push('！'),
            '?' => hiragana.push('？'),
            '-' => hiragana.push('ー'),
            '1' => hiragana.push('１'),
            '2' => hiragana.push('２'),
            '3' => hiragana.push('３'),
            '4' => hiragana.push('４'),
            '5' => hiragana.push('５'),
            '6' => hiragana.push('６'),
            '7' => hiragana.push('７'),
            '8' => hiragana.push('８'),
            '9' => hiragana.push('９'),
            '0' => hiragana.push('０'),
            'a' => hiragana.push('あ'),
            'i' => hiragana.push('い'),
            'u' => hiragana.push('う'),
            'e' => hiragana.push('え'),
            'o' => hiragana.push('お'),
            'k' => k(&mut hiragana, &mut characters)?,
            'g' => g(&mut hiragana, &mut characters)?,
            's' => s::s(&mut hiragana, &mut characters)?,
            'z' => z::z(&mut hiragana, &mut characters)?,
            'j' => j::j(&mut hiragana, &mut characters)?,
            't' => t::t(&mut hiragana, &mut characters)?,
            'c' => c::c(&mut hiragana, &mut characters)?,
            'n' => hiragana.push('ん'),
            'x' => small(&mut hiragana, &mut characters)?,
            // "ka" => "か",
            // "ki" => "き",
            // "ku" => "く",
            // "ke" => "け",
            // "ko" => "こ",
            // "kya" => "きゃ",
            // "kyu" => "きゅ",
            // "kyo" => "きょ",
            // "ga" => "が",
            // "gi" => "ぎ",
            // "gu" => "ぐ",
            // "ge" => "げ",
            // "go" => "ご",
            // "gya" => "ぎゃ",
            // "gyu" => "ぎゅ",
            // "gyo" => "ぎょ",
            // "sa" => "さ",
            // "si" | "shi" => "し",
            // "su" => "す",
            // "se" => "せ",
            // "so" => "そ",
            // "sha" => "しゃ",
            // "shu" => "しゅ",
            // "she" => "しぇ",
            // "sho" => "しょ",
            // "za" => "ざ",
            // "zi" | "ji" => "じ",
            // "zu" => "ず",
            // "ze" => "ぜ",
            // "zo" => "ぞ",
            // "ja" => "じゃ",
            // "ju" => "じゅ",
            // "je" => "じぇ",
            // "jo" => "じょ",
            // "ta" => "た",
            // "ti" | "chi" => "ち",
            // "tu" | "tsu" => "つ",
            // "te" => "て",
            // "to" => "と",
            // "cha" => "ちゃ",
            // "chu" => "ちゅ",
            // "che" => "ちぇ",
            // "cho" => "ちょ",
            // "thi" => "てぃ",
            // "thu" => "てゅ",
            // "da" => "だ",
            // "di" => "ぢ",
            // "du" => "づ",
            // "de" => "で",
            // "do" => "ど",
            // "dhi" => "でぃ",
            // "dhu" => "でゅ",
            // "na" => "な",
            // "ni" => "に",
            // "nu" => "ぬ",
            // "ne" => "ね",
            // "no" => "の",
            // "nya" => "にゃ",
            // "nyu" => "にゅ",
            // "nyo" => "にょ",
            // "ha" => "は",
            // "hi" => "ひ",
            // "hu" => "ふ",
            // "he" => "へ",
            // "ho" => "ほ",
            // "hya" => "ひゃ",
            // "hyu" => "ひゅ",
            // "hyo" => "ひょ",
            // "ba" => "ば",
            // "bi" => "び",
            // "bu" => "ぶ",
            // "be" => "べ",
            // "bo" => "ぼ",
            // "bya" => "びゃ",
            // "byu" => "びゅ",
            // "byo" => "びょ",
            // "pa" => "ぱ",
            // "pi" => "ぴ",
            // "pu" => "ぷ",
            // "pe" => "ぺ",
            // "po" => "ぽ",
            // "pya" => "ぴゃ",
            // "pyu" => "ぴゅ",
            // "pyo" => "ぴょ",
            // "fa" => "ふぁ",
            // "fi" => "ふぃ",
            // "fe" => "ふぇ",
            // "fo" => "ふぉ",
            // "va" => "ゔぁ",
            // "vi" => "ゔぃ",
            // "vu" => "ゔ",
            // "ve" => "ゔぇ",
            // "vo" => "ゔぉ",
            // "ma" => "ま",
            // "mi" => "み",
            // "mu" => "む",
            // "me" => "め",
            // "mo" => "も",
            // "mya" => "みゃ",
            // "myu" => "みゅ",
            // "myo" => "みょ",
            // "ya" => "や",
            // "yu" => "ゆ",
            // "yo" => "よ",
            // "ra" => "ら",
            // "ri" => "り",
            // "ru" => "る",
            // "re" => "れ",
            // "ro" => "ろ",
            // "rya" => "りゃ",
            // "ryu" => "りゅ",
            // "ryo" => "りょ",
            // "wa" => "わ",
            // "wo" => "を",
            // s if s.starts_with('x') && s.len() > 1 => {
            //     hiragana.push_str(&hira(&s[1..]).small()?);
            //     continue;
            // },
            // s if s.len() > 2 => {
            //     hiragana.push_str(&add_hira_little_tsu(s)?);
            //     continue;
            // },
            _ => return Err(unexpected_char_error(i, c)),
        }
    }
    Ok(hiragana)
}

fn add_hira_little_tsu(s: &str) -> Result {
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
        return Err(format!("3+ hiragana char pattern not recognized: {}", s));
    }
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
        assert_eq!(hira("xa"), Ok(String::from("ぁ")));
        assert_eq!(hira("xi"), Ok(String::from("ぃ")));
        assert_eq!(hira("xu"), Ok(String::from("ぅ")));
        assert_eq!(hira("xe"), Ok(String::from("ぇ")));
        assert_eq!(hira("xo"), Ok(String::from("ぉ")));
        assert_eq!(hira("ka"), Ok(String::from("か")));
        assert_eq!(hira("ki"), Ok(String::from("き")));
        assert_eq!(hira("ku"), Ok(String::from("く")));
        assert_eq!(hira("ke"), Ok(String::from("け")));
        assert_eq!(hira("ko"), Ok(String::from("こ")));
        assert_eq!(hira("kya"), Ok(String::from("きゃ")));
        assert_eq!(hira("kyu"), Ok(String::from("きゅ")));
        assert_eq!(hira("kyo"), Ok(String::from("きょ")));
        assert_eq!(hira("ga"), Ok(String::from("が")));
        assert_eq!(hira("gi"), Ok(String::from("ぎ")));
        assert_eq!(hira("gu"), Ok(String::from("ぐ")));
        assert_eq!(hira("ge"), Ok(String::from("げ")));
        assert_eq!(hira("go"), Ok(String::from("ご")));
        assert_eq!(hira("gya"), Ok(String::from("ぎゃ")));
        assert_eq!(hira("gyu"), Ok(String::from("ぎゅ")));
        assert_eq!(hira("gyo"), Ok(String::from("ぎょ")));
        assert_eq!(hira("sa"), Ok(String::from("さ")));
        assert_eq!(hira("si"), Ok(String::from("し")));
        assert_eq!(hira("su"), Ok(String::from("す")));
        assert_eq!(hira("se"), Ok(String::from("せ")));
        assert_eq!(hira("so"), Ok(String::from("そ")));
        assert_eq!(hira("sha"), Ok(String::from("しゃ")));
        assert_eq!(hira("shu"), Ok(String::from("しゅ")));
        assert_eq!(hira("she"), Ok(String::from("しぇ")));
        assert_eq!(hira("sho"), Ok(String::from("しょ")));
        assert_eq!(hira("za"), Ok(String::from("ざ")));
        assert_eq!(hira("zi"), Ok(String::from("じ")));
        assert_eq!(hira("zu"), Ok(String::from("ず")));
        assert_eq!(hira("ze"), Ok(String::from("ぜ")));
        assert_eq!(hira("zo"), Ok(String::from("ぞ")));
        assert_eq!(hira("ja"), Ok(String::from("じゃ")));
        assert_eq!(hira("ju"), Ok(String::from("じゅ")));
        assert_eq!(hira("je"), Ok(String::from("じぇ")));
        assert_eq!(hira("jo"), Ok(String::from("じょ")));
        assert_eq!(hira("ta"), Ok(String::from("た")));
        assert_eq!(hira("ti"), Ok(String::from("ち")));
        assert_eq!(hira("tu"), Ok(String::from("つ")));
        assert_eq!(hira("xtu"), Ok(String::from("っ")));
        assert_eq!(hira("te"), Ok(String::from("て")));
        assert_eq!(hira("to"), Ok(String::from("と")));
        assert_eq!(hira("cha"), Ok(String::from("ちゃ")));
        assert_eq!(hira("chu"), Ok(String::from("ちゅ")));
        assert_eq!(hira("che"), Ok(String::from("ちぇ")));
        assert_eq!(hira("cho"), Ok(String::from("ちょ")));
        assert_eq!(hira("thi"), Ok(String::from("てぃ")));
        assert_eq!(hira("thu"), Ok(String::from("てゅ")));
        assert_eq!(hira("da"), Ok(String::from("だ")));
        assert_eq!(hira("di"), Ok(String::from("ぢ")));
        assert_eq!(hira("du"), Ok(String::from("づ")));
        assert_eq!(hira("de"), Ok(String::from("で")));
        assert_eq!(hira("do"), Ok(String::from("ど")));
        assert_eq!(hira("dhi"), Ok(String::from("でぃ")));
        assert_eq!(hira("dhu"), Ok(String::from("でゅ")));
        assert_eq!(hira("na"), Ok(String::from("な")));
        assert_eq!(hira("ni"), Ok(String::from("に")));
        assert_eq!(hira("nu"), Ok(String::from("ぬ")));
        assert_eq!(hira("ne"), Ok(String::from("ね")));
        assert_eq!(hira("no"), Ok(String::from("の")));
        assert_eq!(hira("nya"), Ok(String::from("にゃ")));
        assert_eq!(hira("nyu"), Ok(String::from("にゅ")));
        assert_eq!(hira("nyo"), Ok(String::from("にょ")));
        assert_eq!(hira("ha"), Ok(String::from("は")));
        assert_eq!(hira("hi"), Ok(String::from("ひ")));
        assert_eq!(hira("hu"), Ok(String::from("ふ")));
        assert_eq!(hira("he"), Ok(String::from("へ")));
        assert_eq!(hira("ho"), Ok(String::from("ほ")));
        assert_eq!(hira("hya"), Ok(String::from("ひゃ")));
        assert_eq!(hira("hyu"), Ok(String::from("ひゅ")));
        assert_eq!(hira("hyo"), Ok(String::from("ひょ")));
        assert_eq!(hira("ba"), Ok(String::from("ば")));
        assert_eq!(hira("bi"), Ok(String::from("び")));
        assert_eq!(hira("bu"), Ok(String::from("ぶ")));
        assert_eq!(hira("be"), Ok(String::from("べ")));
        assert_eq!(hira("bo"), Ok(String::from("ぼ")));
        assert_eq!(hira("bya"), Ok(String::from("びゃ")));
        assert_eq!(hira("byu"), Ok(String::from("びゅ")));
        assert_eq!(hira("byo"), Ok(String::from("びょ")));
        assert_eq!(hira("pa"), Ok(String::from("ぱ")));
        assert_eq!(hira("pi"), Ok(String::from("ぴ")));
        assert_eq!(hira("pu"), Ok(String::from("ぷ")));
        assert_eq!(hira("pe"), Ok(String::from("ぺ")));
        assert_eq!(hira("po"), Ok(String::from("ぽ")));
        assert_eq!(hira("pya"), Ok(String::from("ぴゃ")));
        assert_eq!(hira("pyu"), Ok(String::from("ぴゅ")));
        assert_eq!(hira("pyo"), Ok(String::from("ぴょ")));
        assert_eq!(hira("fa"), Ok(String::from("ふぁ")));
        assert_eq!(hira("fi"), Ok(String::from("ふぃ")));
        assert_eq!(hira("fe"), Ok(String::from("ふぇ")));
        assert_eq!(hira("fo"), Ok(String::from("ふぉ")));
        assert_eq!(hira("va"), Ok(String::from("ゔぁ")));
        assert_eq!(hira("vi"), Ok(String::from("ゔぃ")));
        assert_eq!(hira("vu"), Ok(String::from("ゔ")));
        assert_eq!(hira("ve"), Ok(String::from("ゔぇ")));
        assert_eq!(hira("vo"), Ok(String::from("ゔぉ")));
        assert_eq!(hira("ma"), Ok(String::from("ま")));
        assert_eq!(hira("mi"), Ok(String::from("み")));
        assert_eq!(hira("mu"), Ok(String::from("む")));
        assert_eq!(hira("me"), Ok(String::from("め")));
        assert_eq!(hira("mo"), Ok(String::from("も")));
        assert_eq!(hira("mya"), Ok(String::from("みゃ")));
        assert_eq!(hira("myu"), Ok(String::from("みゅ")));
        assert_eq!(hira("myo"), Ok(String::from("みょ")));
        assert_eq!(hira("ya"), Ok(String::from("や")));
        assert_eq!(hira("yu"), Ok(String::from("ゆ")));
        assert_eq!(hira("yo"), Ok(String::from("よ")));
        assert_eq!(hira("xya"), Ok(String::from("ゃ")));
        assert_eq!(hira("xyu"), Ok(String::from("ゅ")));
        assert_eq!(hira("xyo"), Ok(String::from("ょ")));
        assert_eq!(hira("ra"), Ok(String::from("ら")));
        assert_eq!(hira("ri"), Ok(String::from("り")));
        assert_eq!(hira("ru"), Ok(String::from("る")));
        assert_eq!(hira("re"), Ok(String::from("れ")));
        assert_eq!(hira("ro"), Ok(String::from("ろ")));
        assert_eq!(hira("rya"), Ok(String::from("りゃ")));
        assert_eq!(hira("ryu"), Ok(String::from("りゅ")));
        assert_eq!(hira("ryo"), Ok(String::from("りょ")));
        assert_eq!(hira("wa"), Ok(String::from("わ")));
        assert_eq!(hira("wo"), Ok(String::from("を")));
        assert_eq!(hira("n"), Ok(String::from("ん")));
    }

    #[test]
    fn hiragana_little_tsu() {
        assert_eq!(hira("tte"), Ok(String::from("って")));
        assert_eq!(hira("sshi"), Ok(String::from("っし")));
    }
}
