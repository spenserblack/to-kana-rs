use to_kana::*;

#[test]
fn hiragana() {
    assert_eq!("まって", hira("matte").unwrap());
}

#[test]
fn katakana() {
    assert_eq!("マッテ", kata("matte").unwrap());
}
