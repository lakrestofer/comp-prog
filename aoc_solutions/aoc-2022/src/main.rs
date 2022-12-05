#![feature(iter_array_chunks)]
extern crate dotenv_codegen;
use utils::get_input;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod utils;

fn main() {
    // uncomment this string to read from stdin
    // let input = input_string();

    // let input = get_input(2022, 1);
    // crate::day01::solve_first(input.clone());
    // crate::day01::solve_second(input);

    // let input = get_input(2022, 2);
    // crate::day02::solve_first(input.clone());
    // crate::day02::solve_second(input);

    // let input = get_input(2022, 3);
    // crate::day03::solve_first(input.clone());
    // crate::day03::solve_second(input);

    // let input = get_input(2022, 4);
    // crate::day04::solve_first(input.clone());
    // crate::day04::solve_second(input);

    let input = get_input(2022, 5);
    // crate::day05::solve_first(input.clone());
    crate::day05::solve_second(input);
}
