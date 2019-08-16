use crate::{
    hira,
    kata,
    Result,
};

pub trait ToKana {
    fn hira(&self) -> Result;
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
}