#[macro_use]
extern crate dotenv_codegen;
use utils::{get_input, input_string};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod utils;

fn main() {
    // uncomment this string to read from stdin
    // let input = input_string();

    let input = get_input(2022, 1);
    crate::day01::solve_first(input.clone());
    crate::day01::solve_second(input);
}
