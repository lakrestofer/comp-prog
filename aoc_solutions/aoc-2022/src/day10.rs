pub fn solve_first(input: String) {
    let mut lines = input.lines();
    let mut score = 0;
    for line in lines {
        score += first_illegal_character(line);
    }
    println!("score: {score}");
}

pub fn solve_second(input: String) {
    let mut lines = input.lines();
    let mut scores: Vec<u64> = lines.map(completion_score).filter(|s| *s != 0).collect();
    scores.sort();
    println!("{:?}", scores);
    let mid_index = scores.len() / 2;
    println!("mid: {}", scores[mid_index]);
    println!("mid+1: {}", scores[mid_index + 1]);
}

fn first_illegal_character(line: &str) -> i32 {
    let chars = line.chars();

    let mut score = 0;
    let mut stack = Vec::new();
    for new_paren in chars {
        if is_opening(new_paren) {
            stack.push(new_paren);
        } else {
            let old_paren = stack.last().unwrap();
            if !is_pair(*old_paren, new_paren) {
                score += illegal_score_table(new_paren);
                break;
            } else {
                stack.pop();
            }
        }
    }

    score
}

fn completion_score(line: &str) -> u64 {
    let chars = line.chars();
    let mut stack = Vec::new();
    for new_paren in chars {
        if is_opening(new_paren) {
            stack.push(new_paren);
        } else {
            let old_paren = stack.last().unwrap();
            if !is_pair(*old_paren, new_paren) {
                // corrupted!
                return 0;
            } else {
                stack.pop();
            }
        }
    }

    let mut score = 0;
    while !stack.is_empty() {
        score *= 5;
        if let Some(open_paren) = stack.pop() {
            println!("open_paren: {open_paren}");
            score += completion_score_table(open_paren);
        }
    }

    if !stack.is_empty() {
        println!("We failed matching all the pairs");
    }
    score
}

fn completion_score_table(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn illegal_score_table(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn is_closing(c: char) -> bool {
    '}' == c || ')' == c || ']' == c || '>' == c
}

fn is_opening(c: char) -> bool {
    '{' == c || '(' == c || '[' == c || '<' == c
}

fn is_pair(op: char, cl: char) -> bool {
    '{' == op && '}' == cl
        || '(' == op && ')' == cl
        || '[' == op && ']' == cl
        || '<' == op && '>' == cl
}

fn try_pop(c: char, stack: &mut Vec<char>) -> bool {
    let top = stack[stack.len() - 1];
    if is_pair(top, c) {
        // it is ok to pop
        stack.pop();
        true
    } else {
        false
    }
}
