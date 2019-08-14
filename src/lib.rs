//! English lettering to Kana converter.
//!
//! # Example
//!
//! ```
//! use to_kana::{
//!     hira, // Hiragana
//!     kata, // Katakana
//! };
//!
//! let e = hira("e").unwrap();
//! let ka = hira("ka").unwrap();
//! let no = kata("no").unwrap();
//! let tte = hira("tte").unwrap();
//!
//! assert_eq!("え", e);
//! assert_eq!("か", ka);
//! assert_eq!("ノ", no);
//! assert_eq!("って", tte);
//! ```

/// The type inside `Result::Err` returned by this library's functions
pub type Error = String;

/// The Result type returned by this library's functions
pub type Result = std::result::Result<String, Error>;

fn syllable_divider<'a>(s: &'a str) -> Vec<&'a str> {
    const VOWELS: [&str;6] = [
        "a",
        "e",
        "i",
        "o",
        "u",
        "y", // Sometimes
    ];
    let mut vec = Vec::new();
    let mut start_index = 0;
    for (end_index, c) in s.char_indices() {
        match c {
            '-' | 'a' | 'e' | 'i' | 'o' | 'u' => {
                vec.push(&s[start_index..=end_index]);
                start_index = end_index + 1;
            },
            'n' => {
                // if n and next char is not vowel, push n and continue
                match s.get(end_index + 1..=end_index + 1) {
                    Some(v) if VOWELS.contains(&v) => {},
                    _ => {
                        vec.push("n");
                        start_index = end_index + 1;
                    },
                }
            }
            _ => {},
        }
    }
    vec
}

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
pub fn hira<'a>(s: &'a str) -> Result {
    let mut hiragana = String::new();

    for s in syllable_divider(s) {
        let kana = match s {
            "-" => String::from("ー"),
            "a" => String::from("あ"),
            "i" => String::from("い"),
            "u" => String::from("う"),
            "e" => String::from("え"),
            "o" => String::from("お"),
            "ka" => String::from("か"),
            "ki" => String::from("き"),
            "ku" => String::from("く"),
            "ke" => String::from("け"),
            "ko" => String::from("こ"),
            "kya" => String::from("きゃ"),
            "kyu" => String::from("きゅ"),
            "kyo" => String::from("きょ"),
            "ga" => String::from("が"),
            "gi" => String::from("ぎ"),
            "gu" => String::from("ぐ"),
            "ge" => String::from("げ"),
            "go" => String::from("ご"),
            "gya" => String::from("ぎゃ"),
            "gyu" => String::from("ぎゅ"),
            "gyo" => String::from("ぎょ"),
            "sa" => String::from("さ"),
            "si" | "shi" => String::from("し"),
            "su" => String::from("す"),
            "se" => String::from("せ"),
            "so" => String::from("そ"),
            "sha" => String::from("しゃ"),
            "shu" => String::from("しゅ"),
            "she" => String::from("しぇ"),
            "sho" => String::from("しょ"),
            "za" => String::from("ざ"),
            "zi" | "ji" => String::from("じ"),
            "zu" => String::from("ず"),
            "ze" => String::from("ぜ"),
            "zo" => String::from("ぞ"),
            "ja" => String::from("じゃ"),
            "ju" => String::from("じゅ"),
            "je" => String::from("じぇ"),
            "jo" => String::from("じょ"),
            "ta" => String::from("た"),
            "ti" | "chi" => String::from("ち"),
            "tu" | "tsu" => String::from("つ"),
            "te" => String::from("て"),
            "to" => String::from("と"),
            "thi" => String::from("てぃ"),
            "thu" => String::from("てゅ"),
            "da" => String::from("だ"),
            "di" => String::from("ぢ"),
            "du" => String::from("づ"),
            "de" => String::from("で"),
            "do" => String::from("ど"),
            "dhi" => String::from("でぃ"),
            "dhu" => String::from("でゅ"),
            "na" => String::from("な"),
            "ni" => String::from("に"),
            "nu" => String::from("ぬ"),
            "ne" => String::from("ね"),
            "no" => String::from("の"),
            "nya" => String::from("にゃ"),
            "nyu" => String::from("にゅ"),
            "nyo" => String::from("にょ"),
            "ha" => String::from("は"),
            "hi" => String::from("ひ"),
            "hu" => String::from("ふ"),
            "he" => String::from("へ"),
            "ho" => String::from("ほ"),
            "hya" => String::from("ひゃ"),
            "hyu" => String::from("ひゅ"),
            "hyo" => String::from("ひょ"),
            "ba" => String::from("ば"),
            "bi" => String::from("び"),
            "bu" => String::from("ぶ"),
            "be" => String::from("べ"),
            "bo" => String::from("ぼ"),
            "bya" => String::from("びゃ"),
            "byu" => String::from("びゅ"),
            "byo" => String::from("びょ"),
            "pa" => String::from("ぱ"),
            "pi" => String::from("ぴ"),
            "pu" => String::from("ぷ"),
            "pe" => String::from("ぺ"),
            "po" => String::from("ぽ"),
            "pya" => String::from("ぴゃ"),
            "pyu" => String::from("ぴゅ"),
            "pyo" => String::from("ぴょ"),
            "fa" => String::from("ふぁ"),
            "fi" => String::from("ふぃ"),
            "fe" => String::from("ふぇ"),
            "fo" => String::from("ふぉ"),
            "ma" => String::from("ま"),
            "mi" => String::from("み"),
            "mu" => String::from("む"),
            "me" => String::from("め"),
            "mo" => String::from("も"),
            "mya" => String::from("みゃ"),
            "myu" => String::from("みゅ"),
            "myo" => String::from("みょ"),
            "ya" => String::from("や"),
            "yu" => String::from("ゆ"),
            "yo" => String::from("よ"),
            "ra" => String::from("ら"),
            "ri" => String::from("り"),
            "ru" => String::from("る"),
            "re" => String::from("れ"),
            "ro" => String::from("ろ"),
            "rya" => String::from("りゃ"),
            "ryu" => String::from("りゅ"),
            "ryo" => String::from("りょ"),
            "wa" => String::from("わ"),
            "wo" => String::from("を"),
            "n" => String::from("ん"),
            s if s.len() > 2 => add_hira_little_tsu(s)?,
            _ => return Err(String::from("Pattern not recognized")),
        };
        hiragana.push_str(&kana);
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

/// Converts an English alphabet string to Katakana
///
/// # Example
///
/// ```
/// use to_kana::kata;
///
/// let si = kata("si").unwrap();
/// let shi = kata("shi").unwrap();
/// let ssi = kata("ssi").unwrap();
/// let sshi = kata("sshi").unwrap();
///
/// assert_eq!("シ", si);
/// assert_eq!("シ", shi);
/// assert_eq!("ッシ", ssi);
/// assert_eq!("ッシ", sshi);
/// ```
pub fn kata(s: &str) -> Result {
    let mut katakana = String::new();

    for s in syllable_divider(s) {
        let kana = match s {
            "-" => String::from("ー"),
            "a" => String::from("ア"),
            "i" => String::from("イ"),
            "u" => String::from("ウ"),
            "e" => String::from("エ"),
            "o" => String::from("オ"),
            "ka" => String::from("カ"),
            "ki" => String::from("キ"),
            "ku" => String::from("ク"),
            "ke" => String::from("ケ"),
            "ko" => String::from("コ"),
            "sa" => String::from("サ"),
            "si" | "shi" => String::from("シ"),
            "su" => String::from("ス"),
            "se" => String::from("セ"),
            "so" => String::from("ソ"),
            "ta" => String::from("タ"),
            "ti" | "chi" => String::from("チ"),
            "tu" | "tsu" => String::from("ツ"),
            "te" => String::from("テ"),
            "to" => String::from("ト"),
            "na" => String::from("ナ"),
            "ni" => String::from("ニ"),
            "nu" => String::from("ヌ"),
            "ne" => String::from("ネ"),
            "no" => String::from("ノ"),
            "ha" => String::from("ハ"),
            "hi" => String::from("ヒ"),
            "hu" | "fu" => String::from("フ"),
            "he" => String::from("ヘ"),
            "ho" => String::from("ホ"),
            "ma" => String::from("マ"),
            "mi" => String::from("ミ"),
            "mu" => String::from("ム"),
            "me" => String::from("メ"),
            "mo" => String::from("モ"),
            "ya" => String::from("ヤ"),
            "yu" => String::from("ユ"),
            "yo" => String::from("ヨ"),
            "ra" => String::from("ラ"),
            "ri" => String::from("リ"),
            "ru" => String::from("ル"),
            "re" => String::from("レ"),
            "ro" => String::from("ロ"),
            "wa" => String::from("ワ"),
            "wo" => String::from("ヲ"),
            "n" => String::from("ン"),
            s if s.len() > 2 => return add_kata_little_tsu(s),
            _ => return Err(String::from("Pattern not recognized")),
        };
        katakana.push_str(&kana);
    }
    Ok(katakana)
}

fn add_kata_little_tsu(s: &str) -> Result {
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
        return Err(format!("3+ katakana char pattern not recognized: {}", s));
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
        assert_eq!(hira("te"), Ok(String::from("て")));
        assert_eq!(hira("to"), Ok(String::from("と")));
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
        assert_eq!(kata("kya"), Ok(String::from("キャ")));
        assert_eq!(kata("kyu"), Ok(String::from("キュ")));
        assert_eq!(kata("kyo"), Ok(String::from("キョ")));
        assert_eq!(kata("ga"), Ok(String::from("ガ")));
        assert_eq!(kata("gi"), Ok(String::from("ギ")));
        assert_eq!(kata("gu"), Ok(String::from("グ")));
        assert_eq!(kata("ge"), Ok(String::from("ゲ")));
        assert_eq!(kata("go"), Ok(String::from("ゴ")));
        assert_eq!(kata("gya"), Ok(String::from("ギャ")));
        assert_eq!(kata("gyu"), Ok(String::from("ギュ")));
        assert_eq!(kata("gyo"), Ok(String::from("ギョ")));
        assert_eq!(kata("sa"), Ok(String::from("サ")));
        assert_eq!(kata("si"), Ok(String::from("シ")));
        assert_eq!(kata("su"), Ok(String::from("ス")));
        assert_eq!(kata("se"), Ok(String::from("セ")));
        assert_eq!(kata("so"), Ok(String::from("ソ")));
        assert_eq!(kata("sha"), Ok(String::from("シャ")));
        assert_eq!(kata("shu"), Ok(String::from("シュ")));
        assert_eq!(kata("she"), Ok(String::from("シェ")));
        assert_eq!(kata("sho"), Ok(String::from("ショ")));
        assert_eq!(kata("za"), Ok(String::from("ザ")));
        assert_eq!(kata("zi"), Ok(String::from("ジ")));
        assert_eq!(kata("zu"), Ok(String::from("ズ")));
        assert_eq!(kata("ze"), Ok(String::from("ゼ")));
        assert_eq!(kata("zo"), Ok(String::from("ゾ")));
        assert_eq!(kata("ja"), Ok(String::from("ジャ")));
        assert_eq!(kata("ju"), Ok(String::from("ジュ")));
        assert_eq!(kata("je"), Ok(String::from("ジェ")));
        assert_eq!(kata("jo"), Ok(String::from("ジョ")));
        assert_eq!(kata("ta"), Ok(String::from("タ")));
        assert_eq!(kata("ti"), Ok(String::from("チ")));
        assert_eq!(kata("tu"), Ok(String::from("ツ")));
        assert_eq!(kata("te"), Ok(String::from("テ")));
        assert_eq!(kata("to"), Ok(String::from("ト")));
        assert_eq!(kata("thi"), Ok(String::from("ティ")));
        assert_eq!(kata("thu"), Ok(String::from("テュ")));
        assert_eq!(kata("da"), Ok(String::from("ダ")));
        assert_eq!(kata("di"), Ok(String::from("ヂ")));
        assert_eq!(kata("du"), Ok(String::from("ヅ")));
        assert_eq!(kata("de"), Ok(String::from("デ")));
        assert_eq!(kata("do"), Ok(String::from("ド")));
        assert_eq!(kata("dhi"), Ok(String::from("ディ")));
        assert_eq!(kata("thu"), Ok(String::from("デュ")));
        assert_eq!(kata("na"), Ok(String::from("ナ")));
        assert_eq!(kata("ni"), Ok(String::from("ニ")));
        assert_eq!(kata("nu"), Ok(String::from("ヌ")));
        assert_eq!(kata("ne"), Ok(String::from("ネ")));
        assert_eq!(kata("no"), Ok(String::from("ノ")));
        assert_eq!(kata("nya"), Ok(String::from("ニャ")));
        assert_eq!(kata("nyu"), Ok(String::from("ニュ")));
        assert_eq!(kata("nyo"), Ok(String::from("ニョ")));
        assert_eq!(kata("ha"), Ok(String::from("ハ")));
        assert_eq!(kata("hi"), Ok(String::from("ヒ")));
        assert_eq!(kata("hu"), Ok(String::from("フ")));
        assert_eq!(kata("he"), Ok(String::from("ヘ")));
        assert_eq!(kata("ho"), Ok(String::from("ホ")));
        assert_eq!(kata("hya"), Ok(String::from("ヒャ")));
        assert_eq!(kata("hyu"), Ok(String::from("ヒュ")));
        assert_eq!(kata("hyo"), Ok(String::from("ヒョ")));
        assert_eq!(kata("ba"), Ok(String::from("バ")));
        assert_eq!(kata("bi"), Ok(String::from("ビ")));
        assert_eq!(kata("bu"), Ok(String::from("ブ")));
        assert_eq!(kata("be"), Ok(String::from("ベ")));
        assert_eq!(kata("bo"), Ok(String::from("ボ")));
        assert_eq!(kata("bya"), Ok(String::from("ビャ")));
        assert_eq!(kata("byu"), Ok(String::from("ビュ")));
        assert_eq!(kata("byo"), Ok(String::from("ビョ")));
        assert_eq!(kata("pa"), Ok(String::from("パ")));
        assert_eq!(kata("pi"), Ok(String::from("ピ")));
        assert_eq!(kata("pu"), Ok(String::from("プ")));
        assert_eq!(kata("pe"), Ok(String::from("ペ")));
        assert_eq!(kata("po"), Ok(String::from("ポ")));
        assert_eq!(kata("pya"), Ok(String::from("ピャ")));
        assert_eq!(kata("pyu"), Ok(String::from("ピュ")));
        assert_eq!(kata("pyo"), Ok(String::from("ピョ")));
        assert_eq!(kata("fa"), Ok(String::from("ファ")));
        assert_eq!(kata("fi"), Ok(String::from("フィ")));
        assert_eq!(kata("fe"), Ok(String::from("フェ")));
        assert_eq!(kata("fo"), Ok(String::from("フォ")));
        assert_eq!(kata("ma"), Ok(String::from("マ")));
        assert_eq!(kata("mi"), Ok(String::from("ミ")));
        assert_eq!(kata("mu"), Ok(String::from("ム")));
        assert_eq!(kata("me"), Ok(String::from("メ")));
        assert_eq!(kata("mo"), Ok(String::from("モ")));
        assert_eq!(kata("mya"), Ok(String::from("ミャ")));
        assert_eq!(kata("myu"), Ok(String::from("ミュ")));
        assert_eq!(kata("myo"), Ok(String::from("ミョ")));
        assert_eq!(kata("ya"), Ok(String::from("ヤ")));
        assert_eq!(kata("yu"), Ok(String::from("ユ")));
        assert_eq!(kata("yo"), Ok(String::from("ヨ")));
        assert_eq!(kata("ra"), Ok(String::from("ラ")));
        assert_eq!(kata("ri"), Ok(String::from("リ")));
        assert_eq!(kata("ru"), Ok(String::from("ル")));
        assert_eq!(kata("re"), Ok(String::from("レ")));
        assert_eq!(kata("ro"), Ok(String::from("ロ")));
        assert_eq!(kata("rya"), Ok(String::from("リャ")));
        assert_eq!(kata("ryu"), Ok(String::from("リュ")));
        assert_eq!(kata("ryo"), Ok(String::from("リョ")));
        assert_eq!(kata("wa"), Ok(String::from("ワ")));
        assert_eq!(kata("wo"), Ok(String::from("ヲ")));
        assert_eq!(kata("n"), Ok(String::from("ン")));
    }

    #[test]
    fn katakana_little_tsu() {
        assert_eq!(kata("tte"), Ok(String::from("ッテ")));
        assert_eq!(kata("sshi"), Ok(String::from("ッシ")));
    }

    #[test]
    fn syllable_division() {
        let syllables = syllable_divider("konbanha");

        assert_eq!(vec!["ko", "n", "ba", "n", "ha"], syllables);
    }

    #[test]
    fn turn_katakana() {
        assert_eq!(Ok(String::from("ターン")), kata("ta-n"));
    }
}
