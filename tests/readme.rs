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
