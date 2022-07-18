#[macro_use]
extern crate dotenv_codegen;
use utils::{get_input, input_string};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() {
    // uncomment this string to read from stdin
    // let input = input_string();

    // let input = get_input(2021, 1);
    // crate::day1::solve_first(input.clone());
    // crate::day1::solve_second(input);

    // let input = get_input(2021, 2);
    // crate::day2::solve_first(input.clone());
    // crate::day2::solve_second(input);

    // let input = get_input(2021, 3);
    //crate::day3::solve_first(input.clone());
    // crate::day3::solve_second(input.clone());

    //let input = get_input(2021, 4);
    // crate::day4::solve_first(input.clone());
    //crate::day4::solve_second(input.clone());

    //let input = get_input(2021, 5);
    // crate::day5::solve_first(input.clone());
    //crate::day5::solve_second(input.clone());

    //let input = get_input(2021, 6);
    //crate::day6::solve_first(input.clone());
    //crate::day6::solve_second(input.clone());

    let input = get_input(2021, 7);
    // crate::day7::solve_first(input.clone());
    crate::day7::solve_second(input.clone());
}
