use crate::Error;
use crate::Result;

pub trait SmallKana {
    fn small(self) -> Result;
}

impl SmallKana for &str {
    fn small(self) -> Result {
        let utf16: Vec<u16> = self.encode_utf16().map(|u16char| {
            match u16char {
                0x3042 ... 0x304A if u16char % 2 == 0 => {
                    u16char - 0x0001
                },
                _ => u16char,
            }
        }).collect();
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

    #[test]
    fn aiueo_hira() {
        assert_eq!("ぁぃぅぇぉ", "あいうえお".small().unwrap());
    }
}
