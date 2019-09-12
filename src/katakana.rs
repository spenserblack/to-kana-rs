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
mod d;
mod n;
mod h;
mod b;
mod p;
mod f;
mod v;
mod m;
mod y;
mod r;
mod w;
mod small;

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
    let mut characters = s.chars().enumerate().peekable();

    while let Some((i, c)) = characters.next() {
        if let Some((_, c2)) = characters.peek() {
            if c == *c2 && c != 'n' {
                katakana.push('ッ');
                continue;
            }
        }
        match c {
            '\'' => {},
            ',' => katakana.push('、'),
            '.' => katakana.push('。'),
            '!' => katakana.push('！'),
            '?' => katakana.push('？'),
            '-' => katakana.push('ー'),
            '1' => katakana.push('１'),
            '2' => katakana.push('２'),
            '3' => katakana.push('３'),
            '4' => katakana.push('４'),
            '5' => katakana.push('５'),
            '6' => katakana.push('６'),
            '7' => katakana.push('７'),
            '8' => katakana.push('８'),
            '9' => katakana.push('９'),
            '0' => katakana.push('０'),
            'a' => katakana.push('ア'),
            'i' => katakana.push('イ'),
            'u' => katakana.push('ウ'),
            'e' => katakana.push('エ'),
            'o' => katakana.push('オ'),
            'k' => k(&mut katakana, &mut characters)?,
            'g' => g(&mut katakana, &mut characters)?,
            's' => s::s(&mut katakana, &mut characters)?,
            'z' => z::z(&mut katakana, &mut characters)?,
            'j' => j::j(&mut katakana, &mut characters)?,
            't' => t::t(&mut katakana, &mut characters)?,
            'c' => c::c(&mut katakana, &mut characters)?,
            'd' => d::d(&mut katakana, &mut characters)?,
            'n' => n::n(&mut katakana, &mut characters)?,
            'h' => h::h(&mut katakana, &mut characters)?,
            'b' => b::b(&mut katakana, &mut characters)?,
            'p' => p::p(&mut katakana, &mut characters)?,
            'f' => f::f(&mut katakana, &mut characters)?,
            'v' => v::v(&mut katakana, &mut characters)?,
            'm' => m::m(&mut katakana, &mut characters)?,
            'y' => y::y(&mut katakana, &mut characters)?,
            'r' => r::r(&mut katakana, &mut characters)?,
            'w' => w::w(&mut katakana, &mut characters)?,
            'x' => small(&mut katakana, &mut characters)?,
            _ => return Err(unexpected_char_error(i, c)),
        }
    }
    Ok(katakana)
}

fn add_kata_little_tsu(s: &str) -> Result {
    let mut chars = s.chars();
    let first_char = chars.next().unwrap();
    let next_char = chars.next().unwrap();
    if first_char == next_char {
        let main_kata = match kata(&s[1..]) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let s = format!(
            "{}{}",
            "ッ",
            main_kata,
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
    fn to_katakana() {
        assert_eq!(kata("a"), Ok(String::from("ア")));
        assert_eq!(kata("i"), Ok(String::from("イ")));
        assert_eq!(kata("u"), Ok(String::from("ウ")));
        assert_eq!(kata("e"), Ok(String::from("エ")));
        assert_eq!(kata("o"), Ok(String::from("オ")));
        assert_eq!(kata("xa"), Ok(String::from("ァ")));
        assert_eq!(kata("xi"), Ok(String::from("ィ")));
        assert_eq!(kata("xu"), Ok(String::from("ゥ")));
        assert_eq!(kata("xe"), Ok(String::from("ェ")));
        assert_eq!(kata("xo"), Ok(String::from("ォ")));
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
        assert_eq!(kata("xtu"), Ok(String::from("ッ")));
        assert_eq!(kata("te"), Ok(String::from("テ")));
        assert_eq!(kata("to"), Ok(String::from("ト")));
        assert_eq!(kata("cha"), Ok(String::from("チャ")));
        assert_eq!(kata("chu"), Ok(String::from("チュ")));
        assert_eq!(kata("che"), Ok(String::from("チェ")));
        assert_eq!(kata("cho"), Ok(String::from("チョ")));
        assert_eq!(kata("thi"), Ok(String::from("ティ")));
        assert_eq!(kata("thu"), Ok(String::from("テュ")));
        assert_eq!(kata("da"), Ok(String::from("ダ")));
        assert_eq!(kata("di"), Ok(String::from("ヂ")));
        assert_eq!(kata("du"), Ok(String::from("ヅ")));
        assert_eq!(kata("de"), Ok(String::from("デ")));
        assert_eq!(kata("do"), Ok(String::from("ド")));
        assert_eq!(kata("dhi"), Ok(String::from("ディ")));
        assert_eq!(kata("dhu"), Ok(String::from("デュ")));
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
        assert_eq!(kata("va"), Ok(String::from("ヴァ")));
        assert_eq!(kata("vi"), Ok(String::from("ヴィ")));
        assert_eq!(kata("vu"), Ok(String::from("ヴ")));
        assert_eq!(kata("ve"), Ok(String::from("ヴェ")));
        assert_eq!(kata("vo"), Ok(String::from("ヴォ")));
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
        assert_eq!(kata("xya"), Ok(String::from("ャ")));
        assert_eq!(kata("xyu"), Ok(String::from("ュ")));
        assert_eq!(kata("xyo"), Ok(String::from("ョ")));
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
    fn turn_katakana() {
        assert_eq!(Ok(String::from("ターン")), kata("ta-n"));
    }
}
