mod mastermind;

use clap::{App, Arg};
use mastermind::*;
use std::io::{self, Write};

fn mastermind_as_human(len: usize) -> Result<(), io::Error> {
    let mut game = MasterMind::new(len);
    game.print_welcome();

    loop {
        println!();
        print!("Enter your guess: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match game.to_mastermind_colors(&input) {
            Ok(combination) => {
                let res = game.guess(&combination);

                print!("Try N°{}: ", game.tries);
                MasterMind::fancy_print_guess(&res.guess);

                println!("{} good, {} wrong", res.good, res.wrong);

                if res.valid {
                    break;
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    println!();
    println!("Congratulation! Your beat the MasterMind!");

    Ok(())
}

const DEFAULT_COLORS_COUNT: i32 = 4;

fn main() -> Result<(), io::Error> {
    let matches = App::new("MasterMind")
        .args(&[Arg::new("len")
            .long("len")
            .short('l')
            .about("Sets the length of the hidden composition")
            .default_value(&DEFAULT_COLORS_COUNT.to_string())
            .takes_value(true)])
        .get_matches();

    let len = matches.value_of_t("len").unwrap_or(4);

    mastermind_as_human(len)?;

    Ok(())
}
