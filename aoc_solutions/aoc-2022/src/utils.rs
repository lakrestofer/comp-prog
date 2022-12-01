use std::{
    fs,
    io::{stdin, Read},
    str::Lines,
};

use reqwest::blocking::Client;
use reqwest::header::COOKIE;

pub fn get_input(year: u32, day: u32) -> String {
    let temp_file = std::env::temp_dir().join(format!("aoc_input_{year}_{day}"));
    if temp_file.exists() && temp_file.is_file() {
        // the file exists, meaning we cached it previously
        std::fs::read_to_string(temp_file).expect("could not read from file to string")
    } else {
        // the file did not exist, so we need to download it and cache it
        dotenv::dotenv().ok();
        let session_key: String = dotenv::var("AOC_SESSION_KEY").unwrap();
        let client = Client::new();
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let cookie = format!("session={session_key}");
        let input = client
            .get(url)
            .header(COOKIE, cookie)
            .send()
            .unwrap()
            .text()
            .unwrap();
        fs::write(temp_file, &input).expect("could not write input to file cache");
        input
    }
}

/// reads from stdin and returns a string with its contents
pub fn input_string() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn fastParse<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse().unwrap()
}

pub fn fastNextParse<T>(lines: &mut Lines) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    lines.next().unwrap().parse().unwrap()
}
