use to_kana::*;

#[test]
fn last_is_number_1() {
    let rust = "[rasuto".kata().unwrap();
    let ga = "ga".hira().unwrap();
    let number_1 = "nanba-1]".kata().unwrap();
    let i_think = "toomoimasu.".hira().unwrap();

    //
    assert_eq!(
        "「ラストがナンバー１」とおもいます。",
        format!("{}{}{}{}", rust, ga, number_1, i_think),
    );
}

#[test]
fn the_new_way_main() {
    let hello_world = to_kana!("{:H}{:K}", "konnichiha,", "wa-rudo!").unwrap();
    assert_eq!("こんにちは、ワールド！", hello_world);
}

#[test]
fn the_new_way_half_width() {
    let katakana = to_kana!("Half-width: {:K/2} works too!", "katakana")
        .unwrap();
    assert_eq!("Half-width: ｶﾀｶﾅ works too!", katakana);
}
