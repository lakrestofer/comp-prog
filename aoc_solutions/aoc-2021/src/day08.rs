// number - number of segments
// 0 - 6
// 1 - 2 - unique
// 2 - 5
// 3 - 5
// 4 - 4 - unique
// 5 - 5
// 6 - 6
// 7 - 3 - unique
// 8 - 7 - unique
// 9 - 6

use std::collections::HashMap;

pub fn solve_first(input: String) {
    let res: usize = input
        .lines()
        .map(|line| {
            let (_, right_raw) = line.split_once('|').unwrap();
            let mut times = 0;
            right_raw
                .split_whitespace()
                .filter(|s| {
                    let len = s.len();
                    len == 2 || len == 4 || len == 3 || len == 7
                })
                .count()
        })
        .sum();
    println!("res: {res}");
}

// number - number of segments
// 0 - 6 - #s == 6
// 1 - 2 - unique
// 2 - 5 - #s == 5
// 3 - 5 - #s == 5
// 4 - 4 - unique
// 5 - 5 - #s == 5
// 6 - 6 - #s == 6
// 7 - 3 - unique
// 8 - 7 - unique
// 9 - 6 - $s == 6 - the segments for four is whole contained in nine

// segment - numbers containing that segment
// a - 8
// b - 6*
// c - 8
// d - 7
// e - 4*
// f - 9*
// g - 7

pub fn solve_second(input: String) {
    let res: usize = input
        .lines()
        .map(|line| {
            let (segments, code) = line.split_once('|').unwrap();

            decode(compute_decoder(segments), code)
        })
        .sum();
    println!("res: {res}");
}

fn decode(decoder: HashMap<usize, usize>, code: &str) -> usize {
    let mut x = 1000;
    let mut num = 0;

    for segment in code.split_whitespace() {
        num += decoder.get(&segment_to_mask(segment)).unwrap() * x;
        x = x / 10;
    }
    num
}

fn compute_decoder(segments: &str) -> HashMap<usize, usize> {
    let mut segments = segments.split_whitespace();
    let mut decoder: HashMap<usize, usize> = HashMap::new();

    let four = segments.clone().find(|s| s.len() == 4).unwrap();
    let seven = segments.clone().find(|s| s.len() == 3).unwrap();
    let one = segments.clone().find(|s| s.len() == 2).unwrap();
    let eight = segments.clone().find(|s| s.len() == 7).unwrap();

    let four_mask = segment_to_mask(four);
    let seven_mask = segment_to_mask(seven);
    let one_mask = segment_to_mask(one);
    let eight_mask = segment_to_mask(eight);

    let len_six = segments
        .clone()
        .filter(|s| s.len() == 6)
        .collect::<Vec<&str>>();
    let mut len_six = len_six.into_iter();

    let mut len_five = segments
        .clone()
        .filter(|s| s.len() == 5)
        .collect::<Vec<&str>>();
    let mut len_five = len_five.into_iter();

    // and we begin
    let (nine, nine_mask) = len_six
        .clone()
        .map(|s| (s, segment_to_mask(s)))
        .find(|(_, m)| *m == (*m | four_mask))
        .unwrap();

    let (six, six_mask) = len_six
        .clone()
        .filter(|s| *s != nine)
        .map(|s| (s, segment_to_mask(s)))
        .find(|(_, m)| *m != (m | one_mask))
        .unwrap();

    let (zero, zero_mask) = len_six
        .find(|s| *s != nine && *s != six)
        .map(|s| (s, segment_to_mask(s)))
        .unwrap();

    let (three, three_mask) = len_five
        .clone()
        .map(|s| (s, segment_to_mask(s)))
        .find(|(_, m)| *m == (m | seven_mask))
        .unwrap();

    let (five, five_mask) = len_five
        .clone()
        .filter(|s| *s != three)
        .map(|s| (s, segment_to_mask(s)))
        .find(|(_, m)| nine_mask == (m | one_mask))
        .unwrap();

    let two = len_five.find(|s| *s != five && *s != three).unwrap();
    decoder.insert(segment_to_mask(zero), 0);
    decoder.insert(segment_to_mask(one), 1);
    decoder.insert(segment_to_mask(two), 2);
    decoder.insert(segment_to_mask(three), 3);
    decoder.insert(segment_to_mask(four), 4);
    decoder.insert(segment_to_mask(five), 5);
    decoder.insert(segment_to_mask(six), 6);
    decoder.insert(segment_to_mask(seven), 7);
    decoder.insert(segment_to_mask(eight), 8);
    decoder.insert(segment_to_mask(nine), 9);

    decoder
}

fn segment_to_mask(s: &str) -> usize {
    s.chars().map(char_to_mask).sum()
}

fn char_to_mask(c: char) -> usize {
    match c {
        'a' => 0b00000001,
        'b' => 0b00000010,
        'c' => 0b00000100,
        'd' => 0b00001000,
        'e' => 0b00010000,
        'f' => 0b00100000,
        'g' => 0b01000000,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s1 = "ecbdgf";
        let s2 = "bfdg";
        let s1m = segment_to_mask(s1);
        let s2m = segment_to_mask(s2);
        let res = s1m & s2m;

        assert!(s1m == (s1m & s2m));
    }
}
