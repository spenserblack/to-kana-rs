use crate::Error;
use crate::Result;

pub trait HalfWidth {
    fn half_width(self) -> Result;
}

impl HalfWidth for &str {
    fn half_width(self) -> Result {
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

    #[test]
    fn aiueo_hira() {
        assert_eq!("ぁぃぅぇぉ", "あいうえお".half_width().unwrap());
    }
}
