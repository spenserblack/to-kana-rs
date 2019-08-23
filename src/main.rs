use clap::{
    App,
    Arg,
    SubCommand,
};

use to_kana::{
    hira,
    kata,
    SmallKana,
};

const HIRA: &str = "hira";
const KATA: &str = "kata";
const INPUT: &str = "STRING";
const SMALL: &str = "small";

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

        let hira_subcommand = SubCommand::with_name(HIRA)
            .about("converts to hiragana")
            .arg(str_arg.clone());
        let kata_subcommand = SubCommand::with_name(KATA)
            .about("converts to katakana")
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

        let mut kana = kata(s);
        if small_kana {
            kana = kana.small();
        }
        println!("{}", kana.unwrap());
    } else {
        panic!("Subcommands {:?} or {:?} are required", HIRA, KATA);
    }
}
