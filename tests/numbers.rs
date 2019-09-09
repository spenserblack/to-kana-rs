use to_kana::*;

#[test]
fn hiragana_ichi1ni2san3() {
    assert_eq!("いち１に２さん３", "ichi1ni2san3".hira().unwrap());
}
