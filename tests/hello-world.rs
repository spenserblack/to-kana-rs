use to_kana::{hira, kata};

#[test]
fn hello_world_mixed() {
    let hello = hira("konnichiha,").unwrap();
    let world = kata("wa-rudo!").unwrap();

    assert_eq!(
        "こんにちは、ワールド！",
        format!("{}{}", hello, world)
    );
}
