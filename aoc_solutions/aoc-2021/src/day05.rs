use std::time::Duration;

const FIELD_SIZE: usize = 1248;
// const FIELD_SIZE: usize = 16;

pub fn solve_first(input: String) {
    let mut lines = input.lines();
    let mut field = [[0; FIELD_SIZE]; FIELD_SIZE];
    for line in lines {
        let points: Vec<(usize, usize)> = points_horizontal(line);
        for (c, r) in points {
            field[r][c] += 1;
        }
    }
    let mut n_overlap = 0;

    for row in field.iter() {
        for n in row.iter() {
            if *n >= 2 {
                n_overlap += 1;
            }
        }
    }

    print_field(&field);
    println!("n_overlap: {n_overlap}");
}

pub fn solve_second(input: String) {
    let mut lines = input.lines();
    let mut field = [[0; FIELD_SIZE]; FIELD_SIZE];
    for line in lines {
        let points: Vec<(usize, usize)> = points(line);
        for (c, r) in points {
            field[r][c] += 1;
        }
    }
    let mut n_overlap = 0;

    for row in field.iter() {
        for n in row.iter() {
            if *n >= 2 {
                n_overlap += 1;
            }
        }
    }
    println!("n_overlap: {n_overlap}");
}

fn print_field(field: &[[u64; FIELD_SIZE]; FIELD_SIZE]) {
    for r in 0..FIELD_SIZE {
        for c in 0..FIELD_SIZE {
            let val = field[r][c];
            if val == 0 {
                print!(".");
            } else {
                print!("{val}");
            }
        }
        print!("\n");
    }
}

// "1,1 -> 1,3" should return 1,1, 1,2, and 1,3.
// "1,1 -> 3,3" should return 1,1 2,2 and 3,3
fn points(s: &str) -> Vec<(usize, usize)> {
    // for now we only deal with horizontal/vertical movements.
    let mut point_strs = s.split_terminator(" -> ");
    let mut p1 = point_strs.next().unwrap().split(',');
    let x1: i64 = p1.next().unwrap().parse().unwrap();
    let y1: i64 = p1.next().unwrap().parse().unwrap();

    let mut p2 = point_strs.next().unwrap().split(',');
    let x2: i64 = p2.next().unwrap().parse().unwrap();
    let y2: i64 = p2.next().unwrap().parse().unwrap();

    let mut output = Vec::new();
    let mut dir = "dir";
    if x1 == x2 {
        dir = "vert";
        let x = x1;
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in y1..=y2 {
            output.push((x as usize, y as usize));
        }
    } else if y1 == y2 {
        dir = "hor";
        let y = y1;
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in x1..=x2 {
            output.push((x as usize, y as usize));
        }
    } else if (x2 - x1).abs() == (y2 - y1).abs() {
        let dx;
        let dy = if y1 < y2 { 1 } else { -1 };
        let mut x = x1;
        let mut y = y1;
        let x_end = x2;
        let y_end = y2;

        if x1 < x2 {
            dx = 1;
        } else {
            dx = -1;
        }
        while x != x_end {
            output.push((x as usize, y as usize));
            x += dx;
            y += dy;
        }
        output.push((x as usize, y as usize));
    }
    output
}
// "1,1 -> 1,3" should return 1,1, 1,2, and 1,3.
fn points_horizontal(s: &str) -> Vec<(usize, usize)> {
    // for now we only deal with horizontal/vertical movements.
    let mut point_strs = s.split_terminator(" -> ");
    let mut p1 = point_strs.next().unwrap().split(',');
    let x1: usize = p1.next().unwrap().parse().unwrap();
    let y1: usize = p1.next().unwrap().parse().unwrap();

    let mut p2 = point_strs.next().unwrap().split(',');
    let x2: usize = p2.next().unwrap().parse().unwrap();
    let y2: usize = p2.next().unwrap().parse().unwrap();

    let mut output = Vec::new();
    if x1 == x2 {
        let x = x1;
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in y1..=y2 {
            output.push((x, y));
        }
    } else if y1 == y2 {
        let y = y1;
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in x1..=x2 {
            output.push((x, y));
        }
    }
    output
}
