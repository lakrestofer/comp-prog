fn parse_hands(s: &str) -> (Hand, Hand) {
    let (h1, h2) = s.split_once(' ').unwrap();
    (hand(h1), hand(h2))
}

pub fn solve_first(input: String) {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (oh, h) = parse_hands(line);
            hand_score(h) + games_score(rock_paper_scissors(oh, h))
        })
        .sum();
    println!("{sum}");
}

pub fn solve_second(input: String) {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (oh, gr) = line.split_once(" ").unwrap();
            let (oh, gr) = (hand(oh), game_result(gr));
            let h = pick_hand(oh, gr);
            hand_score(h) + games_score(gr)
        })
        .sum();
    println!("{sum}");
}

fn game_result(s: &str) -> GameResult {
    use GameResult::*;
    match s {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => unreachable!(),
    }
}

fn pick_hand(oh: Hand, gr: GameResult) -> Hand {
    use GameResult::*;
    use Hand::*;
    match (oh, gr) {
        (Rock, Lose) => Scissor,
        (Rock, Win) => Paper,
        (Paper, Lose) => Rock,
        (Paper, Win) => Scissor,
        (Scissor, Lose) => Paper,
        (Scissor, Win) => Rock,
        (h, Draw) => h,
    }
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

fn hand(code: &str) -> Hand {
    match code {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissor,
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissor,
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

fn games_score(game_result: GameResult) -> u32 {
    match game_result {
        GameResult::Lose => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6,
    }
}

fn rock_paper_scissors(h1: Hand, h2: Hand) -> GameResult {
    use GameResult::*;
    match (h1, h2) {
        (Hand::Rock, Hand::Paper) => Win,
        (Hand::Paper, Hand::Scissor) => Win,
        (Hand::Scissor, Hand::Rock) => Win,
        (Hand::Rock, Hand::Scissor) => Lose,
        (Hand::Paper, Hand::Rock) => Lose,
        (Hand::Scissor, Hand::Paper) => Lose,
        (_, _) => Draw,
    }
}

fn hand_score(hand: Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissor => 3,
    }
}
