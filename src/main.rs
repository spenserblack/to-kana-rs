use clap::{
    App,
    Arg,
    SubCommand,
};

use to_kana::{
    hira,
    kata,
};

const HIRA: &str = "hira";
const KATA: &str = "kata";
const INPUT: &str = "STRING";

fn main() {
    let app = {
        let str_arg = Arg::with_name(INPUT)
            .help("The string to be converted to kana")
            .required(true)
            .index(1);
        let hira_subcommand = SubCommand::with_name(HIRA)
            .about("converts to hiragana")
            .arg(str_arg.clone());
        let kata_subcommand = SubCommand::with_name(KATA)
            .about("converts to katakana")
            .arg(str_arg.clone());
        let app = App::new("to-kana")
            .version("0.3.0")
            .author("Spenser Black <spenserblack01@gmail.com>")
            .about("Converts English lettering to kana")
            .subcommand(hira_subcommand)
            .subcommand(kata_subcommand);
        app
    };
    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches(HIRA) {
        let s = matches.value_of(INPUT).unwrap();
        println!("{}", hira(s).unwrap());
    } else if let Some(matches) = matches.subcommand_matches(KATA) {
        let s = matches.value_of(INPUT).unwrap();
        println!("{}", kata(s).unwrap());
    } else {
        panic!("Subcommands {:?} or {:?} are required", HIRA, KATA);
    }
}
