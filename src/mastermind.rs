use ansi_term::Colour::{self, Blue, Cyan, Green, Purple, Red, White, Yellow, RGB};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[allow(non_upper_case_globals)]
const Orange: Colour = RGB(255, 165, 0);

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Color {
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

pub struct MasterMindResponse {
    pub guess: Vec<Color>,
    pub valid: bool,
    pub good: usize,
    pub wrong: usize,
}

pub struct MasterMind {
    secret_len: usize,
    secret: Vec<Color>,

    pub tries: u64,
}

impl MasterMind {
    pub fn new(secret_len: usize) -> Self {
        Self {
            secret_len,
            secret: (0..secret_len).map(|_| rand::random()).collect(),
            tries: 0,
        }
    }

    pub fn to_mastermind_colors(&self, input: &str) -> Result<Vec<Color>, String> {
        let mut colors = vec![];
        let letters = input.trim();

        if letters.len() != self.secret_len {
            return Err(format!(
                "A minimum of {} colors should be given",
                self.secret_len
            ));
        }

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
                    return Err(format!(
                        "Color '{}' is invalid - you can only use the following colors:",
                        letter
                    ));
                }
            };

            colors.push(color);
        }

        Ok(colors)
    }

    pub fn guess(&mut self, combination: &[Color]) -> MasterMindResponse {
        self.tries += 1;

        let good = Self::number_of_well_placed_pawns(&self.secret, combination);
        let wrong = Self::number_of_not_well_placed_pawns(&self.secret, combination);

        MasterMindResponse {
            guess: combination.to_vec(),
            valid: good == self.secret_len,
            good,
            wrong,
        }
    }

    pub fn number_of_well_placed_pawns(secret: &[Color], combination: &[Color]) -> usize {
        secret
            .iter()
            .enumerate()
            .filter(|&(i, color)| combination[i] == *color)
            .count()
    }

    pub fn number_of_not_well_placed_pawns(secret: &[Color], combination: &[Color]) -> usize {
        secret.len() - MasterMind::number_of_well_placed_pawns(secret, combination)
    }

    pub fn print_welcome(&self) {
        println!(
            " __    __     ______     ______     ______   ______     ______     __    __     __     __   __     _____"
        );
        println!("/\\ \"-./  \\   /\\  __ \\   /\\  ___\\   /\\__  _\\ /\\  ___\\   /\\  == \\   /\\ \"-./  \\   /\\ \\   /\\ \"-.\\ \\   /\\  __-.");
        println!("\\ \\ \\-./\\ \\  \\ \\  __ \\  \\ \\___  \\  \\/_/\\ \\/ \\ \\  __\\   \\ \\  __<   \\ \\ \\-./\\ \\  \\ \\ \\  \\ \\ \\-.  \\  \\ \\ \\/\\ \\");
        println!(" \\ \\_\\ \\ \\_\\  \\ \\_\\ \\_\\  \\/\\_____\\    \\ \\_\\  \\ \\_____\\  \\ \\_\\ \\_\\  \\ \\_\\ \\ \\_\\  \\ \\_\\  \\ \\_\\\\\"\\_\\  \\ \\____-");
        println!("  \\/_/  \\/_/   \\/_/\\/_/   \\/_____/     \\/_/   \\/_____/   \\/_/ /_/   \\/_/  \\/_/   \\/_/   \\/_/ \\/_/   \\/____/");
        println!();
        println!(
            "Welcome against the MasterMind! You need to find the color combination constitued of {} colors with a minimum of tries. Good luck.",
            self.secret_len
        );
        println!();
        println!("Possible colors are:");
        Self::print_possible_colors();
    }

    pub fn fancy_print_guess(guess: &[Color]) {
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

        println!();
    }

    fn print_possible_colors() {
        println!("{} for {}", Red.bold().paint("R"), Red.bold().paint("Red"));
        println!(
            "{} for {}",
            Green.bold().paint("G"),
            Green.bold().paint("Green")
        );
        println!(
            "{} for {}",
            Blue.bold().paint("B"),
            Blue.bold().paint("Blue")
        );
        println!(
            "{} for {}",
            Purple.bold().paint("P"),
            Purple.bold().paint("Purple")
        );
        println!(
            "{} for {}",
            Orange.bold().paint("O"),
            Orange.bold().paint("Orange")
        );
        println!(
            "{} for {}",
            Yellow.bold().paint("Y"),
            Yellow.bold().paint("Yellow")
        );
        println!(
            "{} for {}",
            White.bold().paint("W"),
            White.bold().paint("White")
        );
        println!(
            "{} for {}",
            Cyan.bold().paint("C"),
            Cyan.bold().paint("Cyan")
        );
    }
}
