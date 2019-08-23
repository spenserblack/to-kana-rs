use clap::{
    App,
    Arg,
    SubCommand,
};

use to_kana::{
    hira,
    kata,
    HalfWidth,
    SmallKana,
};

const HIRA: &str = "hira";
const KATA: &str = "kata";
const INPUT: &str = "STRING";
const SMALL: &str = "small";
const HALF: &str = "half";

fn main() {
    let app = {
        let str_arg = Arg::with_name(INPUT)
            .help("The string to be converted to kana")
            .required(true)
            .index(1);
        let small_arg = Arg::with_name(SMALL)
            .short("s")
            .long(SMALL)
            .help("Sets the input to be converted to small kana");
        let half_arg = Arg::with_name(HALF)
            .long(HALF)
            .help("Sets the input to be converted to half-width kana");

        let hira_subcommand = SubCommand::with_name(HIRA)
            .about("converts to hiragana")
            .arg(str_arg.clone());
        let kata_subcommand = SubCommand::with_name(KATA)
            .about("converts to katakana")
            .arg(half_arg)
            .arg(str_arg.clone());
        let app = App::new("to-kana")
            .version("0.4.0")
            .about("Converts English lettering to kana")
            .subcommand(hira_subcommand)
            .subcommand(kata_subcommand)
            .arg(small_arg);
        app
    };
    let matches = app.get_matches();
    let small_kana = matches.is_present(SMALL);

    if let Some(matches) = matches.subcommand_matches(HIRA) {
        let s = matches.value_of(INPUT).unwrap();

        let mut kana = hira(s);
        if small_kana {
            kana = kana.small();
        }
        println!("{}", kana.unwrap());
    } else if let Some(matches) = matches.subcommand_matches(KATA) {
        let s = matches.value_of(INPUT).unwrap();
        let half_width = matches.is_present(HALF);

        let mut kana = kata(s);
        if small_kana {
            kana = kana.small();
        }
        if half_width {
            kana = kana.half_width();
        }
        println!("{}", kana.unwrap());
    } else {
        panic!("Subcommands {:?} or {:?} are required", HIRA, KATA);
    }
}
