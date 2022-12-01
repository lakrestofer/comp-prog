const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Clone, Copy)]
struct Octopus {
    has_flashed: bool,
    will_flash: bool,
    energy_level: u32,
}

type Octopi = [[Octopus; WIDTH]; HEIGHT];
impl Default for Octopus {
    fn default() -> Self {
        Self {
            has_flashed: false,
            will_flash: false,
            energy_level: 0,
        }
    }
}

pub fn solve_first(input: String) {
    let mut octopi = parse_input(input);
    let n_flashes: i32 = (0..100).map(|_| simulate_one_step(&mut octopi)).sum();
    println!("n_flashes: {n_flashes}");
}

fn parse_input(input: String) -> Octopi {
    let mut octopi: Octopi = [[Octopus::default(); WIDTH]; HEIGHT];

    for (row, line) in input.lines().enumerate() {
        for (col, num) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            octopi[row][col].energy_level = num;
        }
    }

    octopi
}
fn simulate_one_step(mut octopi: &mut Octopi) -> i32 {
    let mut n_flashed = 0;

    for (r, c) in (0..HEIGHT).zip(0..WIDTH) {
        octopi[r][c].energy_level += 1;
    }
    loop {
        let fs = flash(&mut octopi);
        if fs == 0 {
            break;
        } else {
            n_flashed += fs;
        }
    }
    for (r, c) in (0..HEIGHT).zip(0..WIDTH) {
        if octopi[r][c].has_flashed {
            octopi[r][c].energy_level = 0;
            octopi[r][c].has_flashed = false;
        }
    }
    n_flashed
}

fn flash(octopi: &mut Octopi) -> i32 {
    let mut flashes = 0;
    for (r, c) in (0..HEIGHT).zip((0..WIDTH)) {
        if octopi[r][c].energy_level > 0 && !octopi[r][c].has_flashed {
            flashes += 1;
            octopi[r][c].has_flashed = true;
            for (r, c) in valid_neighbors(r, c) {
                octopi[r][c].energy_level += 1;
            }
        }
    }
    flashes
}

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
