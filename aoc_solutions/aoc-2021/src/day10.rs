pub fn solve_first(input: String) {
    let mut lines = input.lines();
    let mut score = 0;
    for line in lines {
        score += first_illegal_character(line);
    }
    println!("score: {score}");
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
                score += score_table(new_paren);
                break;
            } else {
                stack.pop();
            }
        }
    }

    score
}
fn score_table(c: char) -> i32 {
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
