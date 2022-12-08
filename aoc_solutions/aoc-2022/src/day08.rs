use std::{collections::VecDeque, fmt::Display};

struct Forest {
    width: usize,
    height: usize,
    forest: Vec<Vec<u32>>,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Forest {
    fn parse_to_forest(input: String) -> Self {
        let forest = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let width = forest[0].len();
        let height = forest.len();
        Self {
            width,
            height,
            forest,
        }
    }

    fn tree_line(&self, r: usize, c: usize, direction: Direction) -> Vec<u32> {
        let mut r = r as isize;
        let mut c = c as isize;
        let mut slice = VecDeque::new();
        while 0 <= r && r < self.width as isize && 0 <= c && c < self.height as isize {
            slice.push_front(self.forest[r as usize][c as usize]);
            match direction {
                Direction::Up => r -= 1,
                Direction::Down => r += 1,
                Direction::Left => c -= 1,
                Direction::Right => c += 1,
            }
        }
        slice.into()
    }

    fn is_visible(&self, r: usize, c: usize) -> bool {
        if r == 0 || r == self.height - 1 || c == 0 || c == self.width - 1 {
            true
        } else {
            let tree = self.forest[r][c];
            DIRECTIONS
                .iter()
                .find(|dir| self.tree_line(r, c, **dir).windows(2).all(|w| w[0] < tree))
                .is_some()
        }
    }

    fn visible_trees(&self) -> usize {
        let mut n_visible = 0;
        for r in 0..self.height {
            for c in 0..self.width {
                if self.is_visible(r, c) {
                    n_visible += 1;
                }
            }
        }
        n_visible
    }
}

const INPUT: &'static str = "30373\n25512\n65332\n33549\n35390";

pub fn solve_first(input: String) {
    let forest = Forest::parse_to_forest(input);
    println!("{}", forest.visible_trees());
}

pub fn solve_second(input: String) {
    let forest = Forest::parse_to_forest(input);
}

fn is_visible(line: &[u32]) -> bool {
    if line.len() == 1 {
        true
    } else {
        line.windows(2).all(|w| w[0] < w[1])
    }
}
