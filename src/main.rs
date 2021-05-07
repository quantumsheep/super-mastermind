mod game;
mod mastermind;

use clap::{App, Arg};
use std::io;

const DEFAULT_COLORS_COUNT: u32 = 4;

fn main() -> Result<(), io::Error> {
    let matches = App::new("MasterMind")
        .args(&[
            Arg::new("len")
                .long("len")
                .short('l')
                .about("Sets the length of the hidden composition")
                .default_value(&DEFAULT_COLORS_COUNT.to_string())
                .takes_value(true),
            Arg::new("ai")
                .long("ai")
                .short('i')
                .about("Let an AI solve the game"),
        ])
        .get_matches();

    let len: u32 = matches.value_of_t("len").unwrap();

    if matches.is_present("ai") {
        game::as_ai::run(len);
    } else {
        game::as_human::run(len)?;
    }

    Ok(())
}
