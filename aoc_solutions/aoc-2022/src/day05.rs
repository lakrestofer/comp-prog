use std::collections::{HashSet};

pub fn solve_first(input: String) {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);
    let stacks = move_crates_by_one(stacks, instructions);
    let mut res = String::new();
    for mut stack in stacks {
        if let Some(top) = stack.pop() {
            res.push(top);
        }
    }
    println!("{res}");
}

pub fn solve_second(input: String) {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);
    let stacks = move_crates_by_bulk(stacks, instructions);
    let mut res = String::new();
    for mut stack in stacks {
        if let Some(top) = stack.pop() {
            res.push(top);
        }
    }
    println!("{res}");
}

fn print_stacks(stacks: &Stacks) {
    println!("[");
    for (col, stack) in stacks.iter().enumerate() {
        println!("{col}    {} - {:?}", stack.len(), stack);
    }
    println!("]");
}

fn move_crates_by_one(mut stacks: Stacks, instructions: Instructions) -> Stacks {
    println!("before:");
    print_stacks(&stacks);
    for instruction in instructions {
        println!("\n{:?}", instruction);
        let (amount, (from, to)) = instruction;
        (0..amount).for_each(|_| {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        });
        print_stacks(&stacks);
    }
    println!("after:");
    print_stacks(&stacks);
    stacks
}

fn move_crates_by_bulk(mut stacks: Stacks, instructions: Instructions) -> Stacks {
    println!("before:");
    print_stacks(&stacks);

    let mut stack = Vec::new();
    for instruction in instructions {
        println!("\n{:?}", instruction);
        let (amount, (from, to)) = instruction;

        (0..amount).for_each(|_| {
            let c = stacks[from].pop().unwrap();
            stack.push(c);
        });
        while !stack.is_empty() {
            stacks[to].push(stack.pop().unwrap());
        }
        print_stacks(&stacks);
    }
    println!("after:");
    print_stacks(&stacks);
    stacks
}

type Stacks = Vec<Vec<char>>;
type Instructions = Vec<(usize, (usize, usize))>;

fn parse_stacks(input: &str) -> Stacks {
    let mut stacks = vec![Vec::new(); 10];
    // we parse the
    let mut lines = input.lines().rev();

    // we use the first line as a mask into the rest of the lines.
    let column_indicies: HashSet<usize> = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_i, c)| c.is_digit(10))
        .map(|(i, _)| i)
        .collect();

    lines.for_each(|line| {
        line.chars()
            .enumerate()
            .filter(|(i, _c)| column_indicies.contains(i))
            .map(|(_i, c)| c)
            .enumerate()
            .map(|(i, c)| (i + 1, c))
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c));
    });

    stacks
}

fn parse_instructions(input: &str) -> Instructions {
    let lines = input.lines();
    let mut instructions = Vec::new();
    for line in lines {
        let mut nums = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok());
        let col = nums.next().unwrap();
        let from = nums.next().unwrap();
        let to = nums.next().unwrap();
        let instruction = (col, (from, to));
        instructions.push(instruction);
    }
    instructions
}
