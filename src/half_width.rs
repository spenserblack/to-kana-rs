use crate::Error;
use crate::Result;

pub trait HalfWidth {
    fn half_width(self) -> Result;
}

impl HalfWidth for &str {
    fn half_width(self) -> Result {
        Ok(String::new())
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
}
