use to_kana::{
    hira,
    kata,
};

#[test]
fn hiragana_konbanwa() {
    let konbanwa = hira("konbanha").unwrap();

    assert_eq!("こんばんは", konbanwa);
}

#[test]
fn katakana_konbanwa() {
    let konbanwa = kata("konbanha").unwrap();

    assert_eq!("コンバンハ", konbanwa);
}
