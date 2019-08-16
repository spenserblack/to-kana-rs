use to_kana::*;

#[test]
fn no_apostrophe() {
    assert_eq!(hira("shinichi").unwrap(), "しにち");
}

#[test]
fn apostrophe() {
    assert_eq!(hira("shin'ichi").unwrap(), "しんいち");
}

#[test]
fn double_n_and_apostrophe() {
    assert_eq!(hira("shin'nichi").unwrap(), "しんにち");
}
