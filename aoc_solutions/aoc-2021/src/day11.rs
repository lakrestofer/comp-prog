const WIDTH: usize = 10;
const HEIGHT: usize = 10;

pub fn solve_first(input: String) {
    println!("{input}");
    let mut octopi: [[(bool, bool, u32); WIDTH]; HEIGHT] = [[(false, false, 0); WIDTH]; HEIGHT];

    for (row, line) in input.lines().enumerate() {
        for (col, num) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            println!("({},{})", row, col);
            octopi[row][col].2 = num;
        }
    }

    let n_steps = 100;
    let n_flashes = simulate_for(octopi, n_steps);
    println!("n_flashes: {n_flashes}");
}

fn simulate_for(mut octopi: [[(bool, bool, u32); WIDTH]; HEIGHT], n_steps: i32) -> i32 {
    let mut n_flashes = 0;
    for _ in 0..n_steps {
        let mut to_flash = Vec::new();

        // first every octopi update their energy level with one
        for r in 0..WIDTH {
            for c in 0..HEIGHT {
                let (has_flashed, will_flash, energy_level) = &mut octopi[r][c];
                *energy_level += 1;
                if *energy_level > 9 {
                    *will_flash = true;
                    to_flash.push((r, c));
                }
            }
        }

        // while there exists octopi_that needs to flash, flash them
        while !to_flash.is_empty() {
            let (r, c) = to_flash.pop().unwrap();
            let (has_flashed, will_flash, energy_level) = &mut octopi[r][c];
            // flash!
            n_flashes += 1;
            *has_flashed = true;
            // we now update all the heighbors
            let neighbors = valid_neighbors(r, c);
            for neighbor_coord in neighbors {
                let (r, c) = neighbor_coord;
                let (has_flashed, will_flash, energy_level) = &mut octopi[r][c];

                *energy_level += 1;
                // if the ocotopus has not flashed, isn't already marked to be flashed and it's energy level just increased over 9,
                // then add it to the flash list
                if !*has_flashed && !*will_flash && *energy_level > 9 {
                    *will_flash = true;
                    to_flash.push((r, c));
                }
            }
        }

        for r in 0..WIDTH {
            for c in 0..HEIGHT {
                let (has_flashed, will_flash, energy_level) = &mut octopi[r][c];
                if *has_flashed {
                    *energy_level = 0;
                    *has_flashed = false;
                }
            }
        }
    }
    n_flashes
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
