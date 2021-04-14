use ansi_term::Colour::{self, Blue, Cyan, Green, Purple, Red, White, Yellow, RGB};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io::{self, Write};

#[allow(non_upper_case_globals)]
const Orange: Colour = RGB(255, 165, 0);

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Purple,
    Orange,
    Yellow,
    White,
    Cyan,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=7) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::Purple,
            4 => Color::Orange,
            5 => Color::Yellow,
            6 => Color::White,
            _ => Color::Cyan,
        }
    }
}

fn fancy_print_guess(guess: &[Color]) {
    for color in guess {
        match color {
            Color::Red => print!("{}", Red.paint("R")),
            Color::Green => print!("{}", Green.paint("G")),
            Color::Blue => print!("{}", Blue.paint("B")),
            Color::Purple => print!("{}", Purple.paint("P")),
            Color::Orange => print!("{}", Orange.paint("O")),
            Color::Yellow => print!("{}", Yellow.paint("Y")),
            Color::White => print!("{}", White.paint("W")),
            Color::Cyan => print!("{}", Cyan.paint("C")),
        }
    }

    println!("");
}

fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32 {
    secret
        .iter()
        .enumerate()
        .filter(|&(i, color)| guess[i] == *color)
        .count() as i32
}

fn number_of_not_well_placed_pawns(secret: &Vec<Color>, guess: &Vec<Color>) -> i32 {
    (secret.len() as i32) - number_of_well_placed_pawns(secret, guess)
}

fn print_welcome(secret_len: usize) {
    println!(" __    __     ______     ______     ______   ______     ______     __    __     __     __   __     _____");
    println!("/\\ \"-./  \\   /\\  __ \\   /\\  ___\\   /\\__  _\\ /\\  ___\\   /\\  == \\   /\\ \"-./  \\   /\\ \\   /\\ \"-.\\ \\   /\\  __-.");
    println!("\\ \\ \\-./\\ \\  \\ \\  __ \\  \\ \\___  \\  \\/_/\\ \\/ \\ \\  __\\   \\ \\  __<   \\ \\ \\-./\\ \\  \\ \\ \\  \\ \\ \\-.  \\  \\ \\ \\/\\ \\");
    println!(" \\ \\_\\ \\ \\_\\  \\ \\_\\ \\_\\  \\/\\_____\\    \\ \\_\\  \\ \\_____\\  \\ \\_\\ \\_\\  \\ \\_\\ \\ \\_\\  \\ \\_\\  \\ \\_\\\\\"\\_\\  \\ \\____-");
    println!("  \\/_/  \\/_/   \\/_/\\/_/   \\/_____/     \\/_/   \\/_____/   \\/_/ /_/   \\/_/  \\/_/   \\/_/   \\/_/ \\/_/   \\/____/");
    println!("");
    println!(
        "Welcome against the MasterMind! You need to find the color combination constitued of {} colors with a minimum of tries. Good luck.",
        secret_len
    );
    println!("");
    println!("Possible colors are:");
    print_possible_colors();
}

fn print_possible_colors() {
    println!("{} for {}", Red.bold().paint("R"), Red.bold().paint("Red"));
    println!("{} for {}", Green.bold().paint("G"), Green.bold().paint("Green"));
    println!("{} for {}", Blue.bold().paint("B"), Blue.bold().paint("Blue"));
    println!("{} for {}", Purple.bold().paint("P"), Purple.bold().paint("Purple"));
    println!("{} for {}", Orange.bold().paint("O"), Orange.bold().paint("Orange"));
    println!("{} for {}", Yellow.bold().paint("Y"), Yellow.bold().paint("Yellow"));
    println!("{} for {}", White.bold().paint("W"), White.bold().paint("White"));
    println!("{} for {}", Cyan.bold().paint("C"), Cyan.bold().paint("Cyan"));
    println!("");
}

fn main() -> Result<(), io::Error> {
    let secret: Vec<Color> = (0..4).map(|_| rand::random()).collect();
    let mut tries = 1u64;

    print_welcome(secret.len());

    loop {
        let mut guess = vec![];

        print!("Enter your guess: ");
        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        let letters = input.trim();

        for letter in letters.chars() {
            let color = match letter.to_ascii_uppercase() {
                'R' => Color::Red,
                'G' => Color::Green,
                'B' => Color::Blue,
                'P' => Color::Purple,
                'O' => Color::Orange,
                'Y' => Color::Yellow,
                'W' => Color::White,
                'C' => Color::Cyan,
                _ => {
                    println!("Color '{}' is invalid - you can only use the following colors:", letter);

                    print_possible_colors();

                    break;
                }
            };

            guess.push(color);
        }

        if letters.len() != secret.len() {
            println!("A minimum of {} colors should be given", secret.len());
            println!("");

            continue;
        }

        if guess.len() != secret.len() {
            continue;
        }

        print!("Try N°{}: ", tries);

        fancy_print_guess(&guess);

        if guess == secret {
            break;
        }

        let good = number_of_well_placed_pawns(&secret, &guess);
        let wrong = number_of_not_well_placed_pawns(&secret, &guess);

        println!("{} good, {} wrong", good, wrong);
        println!("");

        tries += 1;
    }

    println!("Congratulation! Your beat the MasterMind!");

    Ok(())
}
