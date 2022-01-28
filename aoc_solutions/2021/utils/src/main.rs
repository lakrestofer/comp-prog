use reqwest::blocking;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let token = fs::read_to_string("token.txt").expect("could not read token");

        let day = args[1].clone();
        println!("getting input from day: {}", day);

        let client = blocking::Client::new();
        let url = format!("https://adventofcode.com/2021/day/{}/input", day);
        let res = client
            .get(url)
            .bearer_auth(token)
            .send()
            .expect("could not get response");
        let content = res.text().expect("could not retrieve text");
        println!("{}", content);
    } else {
        println!("Please enter correct number of args");
        std::process::exit(1);
    }
}
