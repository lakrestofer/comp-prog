#![feature(iter_array_chunks, is_sorted)]
extern crate dotenv_codegen;
use utils::get_input;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
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

    // let input = get_input(2022, 5);
    // crate::day05::solve_first(input.clone());
    // crate::day05::solve_second(input);

    // let input = get_input(2022, 6);
    // crate::day06::solve_first(input.clone());
    // crate::day06::solve_second(input);

    // let input = get_input(2022, 7);
    // crate::day07::solve_first(input.clone());
    // crate::day07::solve_second(input);

    let input = get_input(2022, 8);
    crate::day08::solve_first(input.clone());
    // crate::day08::solve_second(input);
}
