use std::fmt::Display;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const AROUND_DIF: [(isize, isize); 8] = [
    (-1, -1), // NW
    (0, -1),  // N
    (1, -1),  // NE
    (1, 0),   // E
    (1, 1),   // SE
    (0, 1),   // S
    (-1, 1),  // SW
    (-1, 0),  // W
];

pub fn solve_first(input: String) {
    let mut octopi = parse_input(input);
    let n_flashes: i32 = (0..100).map(|_| step(&mut octopi)).sum();
    println!("n_flashes: {n_flashes}");
}

/// performs one "step" and returns how many flashed
fn step(mut octopi: &mut Octopi) -> i32 {
    let mut n_flashed = 0;

    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            octopi[r][c].energy += 1;
        }
    }

    loop {
        let fs = flash(&mut octopi);

        if fs == 0 {
            break;
        } else {
            n_flashed += fs;
        }
    }

    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            if octopi[r][c].flashed {
                octopi[r][c].energy = 0;
                octopi[r][c].flashed = false;
            }
        }
    }

    n_flashed
}

fn flash(octopi: &mut Octopi) -> i32 {
    let mut flashes = 0;
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            if octopi[r][c].energy > 9 && !octopi[r][c].flashed {
                flashes += 1;
                octopi[r][c].flashed = true;
                for (r, c) in valid_neighbors(r, c) {
                    octopi[r][c].energy += 1;
                }
            }
        }
    }
    flashes
}

fn valid_neighbors(r: usize, c: usize) -> Vec<(usize, usize)> {
    let (r, c) = (r as isize, c as isize);
    AROUND_DIF
        .iter()
        .map(|(rd, cd)| (r + rd, c + cd))
        .filter(|(nr, nc)| {
            0 <= *nr && *nr < (WIDTH as isize) && 0 <= *nc && *nc < (HEIGHT as isize)
        })
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

fn parse_input(input: String) -> Octopi {
    let mut octopi: Octopi = [[Octopus::default(); WIDTH]; HEIGHT];
    for (row, line) in input.lines().enumerate() {
        for (col, num) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            octopi[row][col].energy = num;
        }
    }
    octopi
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
struct Octopus {
    flashed: bool,
    energy: u32,
}

impl Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.energy)?;
        Ok(())
    }
}

fn print_octopi(octopi: &Octopi) {
    for row in octopi {
        let mut output = String::new();
        for octopus in row {
            output.push_str(&format!(
                "{}{:02} ",
                if octopus.flashed { "!" } else { " " },
                octopus.energy
            ))
        }
        println!("{output}");
    }
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
}

type Octopi = [[Octopus; WIDTH]; HEIGHT];
impl Default for Octopus {
    fn default() -> Self {
        Self {
            flashed: false,
            energy: 0,
        }
    }
}
