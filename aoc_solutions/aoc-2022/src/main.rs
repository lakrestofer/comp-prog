#[macro_use]
extern crate dotenv_codegen;
use utils::{get_input, input_string};

mod day01;
mod utils;

fn main() {
    // uncomment this string to read from stdin
    // let input = input_string();

    let input = get_input(2022, 1);
    crate::day01::solve_first(input.clone());
    crate::day01::solve_second(input);
}
