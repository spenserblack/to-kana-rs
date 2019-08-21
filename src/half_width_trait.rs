use crate::Error;
use crate::Result;

pub trait HalfWidth {
    fn half_width(self) -> Result;
}

impl HalfWidth for &str {
    fn half_width(self) -> Result {
        let mut half_width = String::new();
        let utf16: Vec<u16> = self.encode_utf16().collect();
        Ok(String::from(""))
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
