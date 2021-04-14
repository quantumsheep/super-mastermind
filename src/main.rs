mod mastermind;

use mastermind::*;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let mut game = MasterMind::new(4);
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
