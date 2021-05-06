use crate::mastermind::*;
use std::{
    convert::TryInto,
    io::{self, Write},
};

pub fn run(len: u32) -> Result<(), io::Error> {
    let len_usize: usize = len.try_into().unwrap();

    let mut game = MasterMind::new(len_usize);
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
