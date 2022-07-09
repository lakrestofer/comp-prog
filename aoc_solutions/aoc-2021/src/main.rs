#[macro_use]
extern crate dotenv_codegen;
use utils::get_input;

mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    // let input = get_input(2021, 1);
    // crate::day1::solve_first(input.clone());
    // crate::day1::solve_second(input);

    // let input = get_input(2021, 2);
    // crate::day2::solve_first(input.clone());
    // crate::day2::solve_second(input);
    let input = get_input(2021, 3);
    crate::day3::solve_first(input.clone());
}
