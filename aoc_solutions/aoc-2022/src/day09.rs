pub fn solve_first(input: String) {
    let height_map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut sum = 0;
    for i in 0..100 {
        for j in 0..100 {
            let (ni, nj) = (if i == 0 { usize::MAX } else { i - 1 }, j);
            let (si, sj) = (i + 1, j);
            let (wi, wj) = (i, if j == 0 { usize::MAX } else { j - 1 });
            let (ei, ej) = (i, j + 1);
            let val = height_map[i][j];
            let n = get_first(&height_map, ni, nj);
            let s = get_first(&height_map, si, sj);
            let w = get_first(&height_map, wi, wj);
            let e = get_first(&height_map, ei, ej);
            if val < n && val < s && val < w && val < e {
                sum += 1 + val;
            }
        }
    }
    println!("sum: {sum}");
}

fn get_first(m: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if is_valid(i, j) {
        m[i][j]
    } else {
        9
    }
}

fn is_valid(i: usize, j: usize) -> bool {
    0 <= i && i < 100 && 0 <= j && j < 100
}

struct Point {
    visited: bool,
    height: u32,
}

pub fn solve_second(input: String) {
    let mut height_map: Vec<Vec<Point>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Point {
                    visited: false,
                    height: c.to_digit(10).unwrap(),
                })
                .collect()
        })
        .collect();

    let mut basins_sizes: Vec<u32> = Vec::new();
    for i in 0..100 {
        for j in 0..100 {
            let p: &mut Point = &mut height_map[i][j];
            if p.visited {
                continue;
            }
            let basin_size = visit(&mut height_map, i, j);
            if basin_size != 0 {
                basins_sizes.push(basin_size);
            }
        }
    }
    basins_sizes.sort();

    println! {"basins: {:?}", basins_sizes};
    let mut prod = 1;
    for _ in 0..3 {
        prod *= basins_sizes.pop().unwrap()
    }
    println!("prod: {prod}");
}

fn visit(hm: &mut Vec<Vec<Point>>, i: usize, j: usize) -> u32 {
    if !is_valid(i, j) {
        return 0;
    }
    let p: &mut Point = &mut hm[i][j];
    if p.visited {
        0
    } else if p.height == 9 {
        p.visited = true;
        0
    } else {
        p.visited = true;
        1 + visit(hm, if i == 0 { usize::MAX } else { i - 1 }, j)
            + visit(hm, i + 1, j)
            + visit(hm, i, if j == 0 { usize::MAX } else { j - 1 })
            + visit(hm, i, j + 1)
    }
}
