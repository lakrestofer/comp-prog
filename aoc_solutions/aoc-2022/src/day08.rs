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

    fn tree_line(&self, r: usize, c: usize, direction: Direction, from_tree: bool) -> Vec<u32> {
        let mut r = r as isize;
        let mut c = c as isize;
        let mut slice = VecDeque::new();
        while 0 <= r && r < self.width as isize && 0 <= c && c < self.height as isize {
            if from_tree {
                slice.push_back(self.forest[r as usize][c as usize]);
            } else {
                slice.push_front(self.forest[r as usize][c as usize]);
            }
            match direction {
                Direction::Up => r -= 1,
                Direction::Down => r += 1,
                Direction::Left => c -= 1,
                Direction::Right => c += 1,
            }
        }
        if from_tree {
            slice.pop_front();
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
                .find(|dir| {
                    self.tree_line(r, c, **dir, false)
                        .windows(2)
                        .all(|w| w[0] < tree)
                })
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

    fn max_scenic_score(&self) -> usize {
        let mut max = 0;
        for r in 1..(self.height - 1) {
            for c in 1..(self.width - 1) {
                let mut score = 1;
                let tree_height = self.forest[r][c];
                println!("({r},{c}), tree height: {tree_height}");
                for dir in DIRECTIONS.iter() {
                    let line = self.tree_line(r, c, *dir, true);
                    println!("   {dir}-{:?}", line);
                    let mut view_distance = 0;
                    for height in line.into_iter() {
                        view_distance += 1;
                        if height >= tree_height {
                            break;
                        }
                    }
                    println!("   view_distance: {view_distance}");
                    score *= view_distance;
                }
                println!("   scienic_score: {score}");

                max = max.max(score);
            }
        }
        max
    }
}

fn print_forest(forest: &Forest) {
    for row in &forest.forest {
        println!("{:?}", row);
    }
}

const INPUT: &'static str = "30373\n25512\n65332\n33549\n35390";

pub fn solve_first(input: String) {
    let forest = Forest::parse_to_forest(input);
    println!("{}", forest.visible_trees());
}

pub fn solve_second(input: String) {
    let forest = Forest::parse_to_forest(input);
    print_forest(&forest);
    let max = forest.max_scenic_score();
    println!("{max}");
}
