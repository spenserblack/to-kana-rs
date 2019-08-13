use to_kana::{
    hira,
    kata,
};

#[test]
fn hiragana_konnichiwa_per_syllable() {
    let ko = hira("ko").unwrap();
    let n = hira("n").unwrap();
    let ni = hira("ni").unwrap();
    let chi = hira("ti").unwrap();
    let wa = hira("ha").unwrap();

    let konnichiwa = format!("{}{}{}{}{}", ko, n, ni, chi, wa);

    assert_eq!("こんにちは", konnichiwa);
}

#[test]
fn hiragana_konnichiwa() {
    assert_eq!("こんにちは", hira("konnitiha").unwrap());
}

#[test]
fn katakana_konnichiwa_per_syllable() {
    let ko = kata("ko").unwrap();
    let n = kata("n").unwrap();
    let ni = kata("ni").unwrap();
    let chi = kata("ti").unwrap();
    let wa = kata("ha").unwrap();

    let konnichiwa = format!("{}{}{}{}{}", ko, n, ni, chi, wa);

    assert_eq!("コンニチハ", konnichiwa);
}

#[test]
fn katakana_konnichiwa() {
    assert_eq!("コンニチハ", kata("konnitiha").unwrap());
}
