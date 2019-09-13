use crate::Result;

/// Defines a type as being able to convert to small kana.
///
/// # Examples
///
/// ```
/// use to_kana::SmallKana;
///
/// assert_eq!("ゃゅょ", "やゆよ".small().unwrap());
/// ```
///
/// It also works on `to_kana::Result`!
///
/// ```
/// use to_kana::{ToKana, SmallKana};
///
/// assert_eq!("ッ", "tsu".kata().small().unwrap());
/// ```
pub trait SmallKana {
    fn small(self) -> Result;
}

impl SmallKana for &str {
    fn small(self) -> Result {
        let utf16: Vec<u16> = self
            .encode_utf16()
            .map(|u16char| match u16char {
                0x3042...0x304A
                | 0x3084...0x3088
                | 0x3064
                | 0x30C4
                | 0x30A2...0x30AA
                | 0x30E4...0x30E8
                    if u16char % 2 == 0 =>
                {
                    u16char - 0x0001
                }
                _ => u16char,
            })
            .collect();
        Ok(String::from_utf16(&utf16).unwrap())
    }
}
impl SmallKana for String {
    fn small(self) -> Result {
        let s: &str = &self;
        s.small()
    }
}
impl SmallKana for Result {
    fn small(self) -> Result {
        self?.small()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToKana;

    #[test]
    fn aiueo_hira() {
        assert_eq!("ぁぃぅぇぉ", "あいうえお".small().unwrap());
    }

    #[test]
    fn yayuyo_hira() {
        assert_eq!("ゃゅょ", "やゆよ".small().unwrap());
    }

    #[test]
    fn tsu_hira() {
        assert_eq!("っ", "つ".small().unwrap());
    }

    #[test]
    fn aiueo_kata() {
        assert_eq!("ァィゥェォ", "アイウエオ".small().unwrap());
    }

    #[test]
    fn yayuyo_kata() {
        assert_eq!("ャュョ", "ヤユヨ".small().unwrap());
    }

    #[test]
    fn tsu_kata() {
        assert_eq!("ッ", "ツ".small().unwrap());
    }

    #[test]
    fn unrecognized_pattern() {
        if let Ok(_) = "unrecognized".hira().small() {
            panic!("Unrecognized pattern was Ok");
        }
    }
}
