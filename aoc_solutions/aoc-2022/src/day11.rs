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
    let mut octopi: [[Octopus; WIDTH]; HEIGHT] = [[Octopus::default(); WIDTH]; HEIGHT];

    for (row, line) in input.lines().enumerate() {
        for (col, num) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            octopi[row][col].energy_level = num;
        }
    }

    let n_steps = 100;
    let n_flashes = simulate_for(octopi, n_steps);
    println!("n_flashes: {n_flashes}");
}

fn simulate_for(mut octopi: [[Octopus; WIDTH]; HEIGHT], n_steps: i32) -> i32 {
    let mut flashes_count = 0;

    for _ in 0..n_steps {
        let mut to_flash = Vec::new();
        for (r, c) in (0..WIDTH).zip(0..HEIGHT) {
            let Octopus {
                has_flashed,
                will_flash,
                energy_level,
            } = &mut octopi[r][c];
            *energy_level += 1;
            if *energy_level > 9 {
                *will_flash = true;
                to_flash.push((r, c));
            }
        }

        let mut flashed = Vec::new();
        while !to_flash.is_empty() {
            let (r, c) = to_flash.pop().unwrap();
            let Octopus {
                has_flashed,
                will_flash,
                energy_level,
            } = &mut octopi[r][c];

            if !*has_flashed {
                // it was to flash, and has not flashed, flash!
                *has_flashed = true;
                *will_flash = false;
                flashes_count += 1;
                flashed.push((r, c));
            }

            // update neighbors
            let neighbors = valid_neighbors(r, c);
            for (r, c) in neighbors {
                let Octopus {
                    has_flashed,
                    will_flash,
                    energy_level,
                } = &mut octopi[r][c];
                *energy_level += 1;

                if !*has_flashed && !*will_flash && *energy_level > 9 {
                    *will_flash = true;
                    to_flash.push((r, c));
                }
            }
        }

        for (r, c) in flashed {
            let Octopus {
                has_flashed,
                will_flash,
                energy_level,
            } = &mut octopi[r][c];
            *has_flashed = false;
            *will_flash = false;
            *energy_level = 0;
        }
    }

    flashes_count
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
