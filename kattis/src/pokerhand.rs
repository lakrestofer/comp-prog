use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let words = buffer.split_whitespace();

    let mut counter = [0; 13];
    for word in words {
        let first_char: char = word.chars().next().unwrap();
        counter[rank_to_index(first_char)] += 1;
    }

    let k = counter.into_iter().max().unwrap();

    println!("{}", k);
}

fn rank_to_index(c: char) -> usize {
    match c {
        'A' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        _ => unreachable!(),
    }
}
