use crate::{hira, kata, Result};

/// Trait to make a type translatable to kana
pub trait ToKana {
    /// Changes the implementing type to Hiragana
    fn hira(&self) -> Result;
    /// Changes the implementing type to Katakana
    fn kata(&self) -> Result;
}

impl ToKana for &str {
    fn hira(&self) -> Result {
        hira(&self)
    }

    fn kata(&self) -> Result {
        kata(&self)
    }
}

impl ToKana for String {
    fn hira(&self) -> Result {
        hira(&self)
    }

    fn kata(&self) -> Result {
        kata(&self)
    }
}

impl ToKana for u8 {
    fn hira(&self) -> Result {
        self.to_string().hira()
    }
    fn kata(&self) -> Result {
        self.to_string().kata()
    }
}

impl ToKana for u16 {
    fn hira(&self) -> Result {
        self.to_string().hira()
    }
    fn kata(&self) -> Result {
        self.to_string().kata()
    }
}

impl ToKana for u32 {
    fn hira(&self) -> Result {
        self.to_string().hira()
    }
    fn kata(&self) -> Result {
        self.to_string().kata()
    }
}

impl ToKana for u64 {
    fn hira(&self) -> Result {
        self.to_string().hira()
    }
    fn kata(&self) -> Result {
        self.to_string().kata()
    }
}

impl ToKana for u128 {
    fn hira(&self) -> Result {
        self.to_string().hira()
    }
    fn kata(&self) -> Result {
        self.to_string().kata()
    }
}

impl ToKana for usize {
    fn hira(&self) -> Result {
        self.to_string().hira()
    }
    fn kata(&self) -> Result {
        self.to_string().kata()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_hira_string() {
        let hello = String::from("konnitiha");
        assert_eq!("こんにちは", hello.hira().unwrap());
    }

    #[test]
    fn hello_hira_str() {
        assert_eq!("こんにちは", "konnitiha".hira().unwrap());
    }

    #[test]
    fn world_kata_string() {
        let world = String::from("wa-rudo");
        assert_eq!("ワールド", world.kata().unwrap());
    }

    #[test]
    fn world_kata_str() {
        assert_eq!("ワールド", "wa-rudo".kata().unwrap());
    }

    #[test]
    fn u32_to_kana() {
        assert_eq!("１０", 10_u32.hira().unwrap());
        assert_eq!("１０", 10_u32.kata().unwrap());
    }
}
