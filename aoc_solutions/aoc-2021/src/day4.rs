use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve_first(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let length = lines.len();
    let n_boards = (length - 1) / 6;

    let mut lines = lines.into_iter();
    let mut numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<VecDeque<u64>>();

    let mut boards = Vec::with_capacity(n_boards);
    for _ in 0..n_boards {
        let _ = lines.next();
        let mut score: u64 = 0;
        let mut val_to_pos = HashMap::new();
        let board = vec![vec![Marker::UnMarked; 5]; 5];
        for r in 0..5 {
            let mut row: Vec<u64> = Vec::with_capacity(5);
            for (c, num) in lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .enumerate()
            {
                score += num;
                val_to_pos.insert(num, (r, c));
                row.push(num);
            }
        }
        let board = Board {
            score,
            val_to_pos,
            board,
        };
        boards.push(board);
    }

    let mut num_iter = numbers.into_iter();
    let first_four = num_iter.by_ref().take(4);

    for num in first_four {
        for board in &mut boards {
            if board.val_to_pos.contains_key(&num) {
                let (row, col) = board.val_to_pos.get(&num).unwrap();
                board.board[*row][*col] = Marker::Marked;
                board.score -= num
            }
        }
    }

    let mut final_score = 0;
    'outer: for num in num_iter {
        for board in &mut boards {
            if board.val_to_pos.contains_key(&num) {
                let (row, col) = board.val_to_pos.get(&num).unwrap();
                board.board[*row][*col] = Marker::Marked;
                board.score -= num;
                if board.bingo() {
                    final_score = num * board.score;
                    break 'outer;
                }
            }
        }
    }
    println!("final score: {final_score}");
}

pub fn solve_second(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let length = lines.len();
    let n_boards = (length - 1) / 6;

    let mut lines = lines.into_iter();
    let mut numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<VecDeque<u64>>();

    // max heap pq
    // tupple contains (
    //    number of turns before this board winns,
    //    score,
    //)
    let mut boards_pq: BinaryHeap<(usize, u64)> = BinaryHeap::new();
    for _ in 0..n_boards {
        let _ = lines.next();
        let mut score: u64 = 0;
        let mut val_to_pos = HashMap::new();
        let board = vec![vec![Marker::UnMarked; 5]; 5];
        for r in 0..5 {
            let mut row: Vec<u64> = Vec::with_capacity(5);
            for (c, num) in lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .enumerate()
            {
                score += num;
                val_to_pos.insert(num, (r, c));
                row.push(num);
            }
        }
        let mut board = Board {
            score,
            val_to_pos,
            board,
        };
        // we now check how many number we have to pull until this board winns
        for (index, num) in numbers.iter().enumerate() {
            if board.val_to_pos.contains_key(num) {
                let (row, col) = board.val_to_pos.get(num).unwrap();
                board.board[*row][*col] = Marker::Marked;
                board.score -= num;
                if board.bingo() {
                    boards_pq.push((index, num * board.score));
                    break;
                }
            }
        }
    }
    println!("boards: {:?}", boards_pq);
    let (turn_won, score) = boards_pq.pop().unwrap();
    println!("board won on turn {turn_won} with score {score}");
}
pub struct Board {
    score: u64,
    val_to_pos: HashMap<u64, (usize, usize)>,
    board: Vec<Vec<Marker>>,
}

impl Board {
    pub fn bingo(&self) -> bool {
        for i in 0..5 {
            let row_sum = self.board[i]
                .iter()
                .map(|marker| if let Marker::Marked = marker { 1 } else { 0 })
                .sum::<u64>();
            let col_sum = self
                .board
                .iter()
                .map(|row| if let Marker::Marked = row[i] { 1 } else { 0 })
                .sum::<u64>();
            if row_sum == 5 || col_sum == 5 {
                return true;
            }
        }
        false
    }
}

#[derive(Copy, Clone)]
pub enum Marker {
    Marked,
    UnMarked,
}
