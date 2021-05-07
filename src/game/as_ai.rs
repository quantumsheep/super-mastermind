use crate::mastermind::*;
use itertools::Itertools;
use rand::Rng;
use size_format::SizeFormatterSI;
use std::{convert::TryInto, mem::size_of};
use thousands::Separable;

fn random_index(len: usize) -> usize {
    rand::thread_rng().gen_range(0..len)
}

pub fn run(len: u32) {
    let len_usize = len.try_into().unwrap();

    let mut game = MasterMind::new(len_usize);
    // game.print_welcome();

    let items = vec![
        Color::Blue,
        Color::Cyan,
        Color::Green,
        Color::Orange,
        Color::Purple,
        Color::Red,
        Color::White,
        Color::Yellow,
    ];

    let combinations_count = items.len().pow(len);
    let combinations_memory_count = size_of::<Vec<Vec<Color>>>()
        * combinations_count
        * size_of::<Vec<Color>>()
        * size_of::<Color>()
        * len_usize
        / 8;

    println!(
        "Generating {} possible combinations... This should use around {}B of memory",
        combinations_count.separate_with_commas(),
        SizeFormatterSI::new(combinations_memory_count.try_into().unwrap())
    );

    let mut combinations = std::iter::repeat(items)
        .take(len_usize)
        .multi_cartesian_product()
        .collect_vec();

    let mut best_score = 0;
    let mut combination_index = random_index(combinations.len());

    loop {
        let combination = combinations[combination_index].clone();
        let res = game.guess(&combinations[combination_index]);

        print!(
            "Try N°{} ({} possibilities left): ",
            game.tries,
            combinations.len()
        );
        MasterMind::fancy_print_guess(&res.guess);

        println!("{} good, {} wrong", res.good, res.wrong);

        if res.valid {
            break;
        }

        if res.good > best_score {
            best_score = res.good;

            combinations.remove(combination_index);
            combinations = combinations
                .into_iter()
                .filter(|permutation| {
                    MasterMind::number_of_well_placed_pawns(&combination, &permutation) >= res.good
                })
                .collect();
        } else {
            combinations.remove(combination_index);
        }

        if combinations.is_empty() {
            panic!("The AI couldn't find the solution");
        }

        combination_index = random_index(combinations.len());
    }

    println!("Congratulation! Your beat the MasterMind!");
}
