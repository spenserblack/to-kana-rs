use to_kana::hira;

#[test]
fn hiragana() {
    assert_eq!("わかっている", hira("wakatteiru").unwrap());
}
