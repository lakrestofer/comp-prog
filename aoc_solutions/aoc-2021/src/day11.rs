const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Clone, Copy)]
struct Octopus {
    has_flashed: bool,
    will_flash: bool,
    energy_level: u32,
}

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

fn parse_input(input: String) -> [[Octopus; WIDTH]; HEIGHT] {
    let mut octopi: [[Octopus; WIDTH]; HEIGHT] = [[Octopus::default(); WIDTH]; HEIGHT];

    for (row, line) in input.lines().enumerate() {
        for (col, num) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            octopi[row][col].energy_level = num;
        }
    }

    octopi
}
fn simulate_one_step(mut octopi: &mut [[Octopus; WIDTH]; HEIGHT]) -> i32 {
    let mut n_flashed = 0;
    let mut will_flash = Vec::new();
    for (r, c) in (0..HEIGHT).zip(0..WIDTH) {
        let octopus = &mut octopi[r][c];
        octopus.energy_level += 1;
        if octopus.energy_level > 9 {
            will_flash.push((r, c));
            octopus.will_flash = true;
        }
    }
    let mut has_flashed = Vec::new();
    while !will_flash.is_empty() {
        let (r, c) = will_flash.pop().unwrap();
        let octopus = &mut octopi[r][c];

        if octopus.has_flashed {
            continue;
        }

        // flash!
        octopus.has_flashed = true;
        octopus.will_flash = false;
        has_flashed.push((r, c));

        // update all the neighbors and maybe add them to the flashlist
        for (nr, nc) in valid_neighbors(r, c) {
            let neighbor = &mut octopi[nr][nc];
            neighbor.energy_level += 1;
            if neighbor.energy_level > 9 && !neighbor.has_flashed && !neighbor.will_flash {
                will_flash.push((nr, nc));
            }
        }
    }

    for (r, c) in has_flashed {
        let octopus = &mut octopi[r][c];
        n_flashed += 1;
        octopus.energy_level = 0;
        octopus.has_flashed = false;
        octopus.will_flash = false;
    }
    n_flashed
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
