use crate::{
    has_softened_diacritic,
    has_hardened_diacritic,
};
use crate::Error;
use crate::Result;

pub trait HalfWidth {
    fn half_width(self) -> Result;
}

impl HalfWidth for &str {
    fn half_width(self) -> Result {
        let mut s = String::new();
        for c in self.chars() {
            let has_softened_diacritic = has_softened_diacritic(&c);
            let has_hardened_diacritic = has_hardened_diacritic(&c);
            let c = match c {
                'ー' => 'ｰ',
                'ア' => 'ｱ',
                'イ' => 'ｲ',
                'ウ' | 'ヴ' => 'ｳ',
                'エ' => 'ｴ',
                'オ' => 'ｵ',
                'カ' | 'ガ' => 'ｶ',
                'キ' | 'ギ' => 'ｷ',
                'ク' | 'グ' => 'ｸ',
                'ケ' | 'ゲ' => 'ｹ',
                'コ' | 'ゴ' => 'ｺ',
                'サ' | 'ザ' => 'ｻ',
                'シ' | 'ジ' => 'ｼ',
                'ス' | 'ズ' => 'ｽ',
                'セ' | 'ゼ' => 'ｾ',
                'ソ' | 'ゾ' => 'ｿ',
                _ => c,
            };
            s.push(c);
            if has_softened_diacritic {
                s.push('ﾞ');
            } else if has_hardened_diacritic {
                s.push('ﾟ');
            }
        }
        Ok(s)
    }
}
impl HalfWidth for String {
    fn half_width(self) -> Result {
        let s: &str = &self;
        s.half_width()
    }
}
impl HalfWidth for Result {
    fn half_width(self) -> Result {
        self?.half_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToKana;

    #[test]
    fn half_width_aiueo() {
        assert_eq!("ｱｲｳｴｵ", "アイウエオ".half_width().unwrap());
    }

    #[test]
    fn half_width_kakikukeko() {
        assert_eq!("ｶﾞｷﾞｸﾞｹﾞｺﾞ", "ガギグゲゴ".half_width().unwrap());
    }
}
