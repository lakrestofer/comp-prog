pub fn jump(nums: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    let n = nums.len();

    if n == 1 {
        return 0;
    }

    let graph: Vec<Vec<usize>> = nums
        .iter()
        .enumerate()
        .map(|(i, &nums)| {
            (1..=(nums as usize))
                .map(|jump| i + jump)
                .into_iter()
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    let mut pred: Vec<Option<usize>> = vec![None; n];
    visited[0] = true;
    queue.push_front(0);

    'outer: while !queue.is_empty() {
        // the next position we have jumpted to.
        let pos = queue.pop_back().unwrap();

        let possible_destinations = &graph[pos];

        for next_pos in possible_destinations.iter() {
            if !visited[*next_pos] {
                if n - 1 <= *next_pos {
                    visited[n - 1] = true;
                    pred[n - 1] = Some(pos);
                    break 'outer;
                }

                visited[*next_pos] = true;
                pred[*next_pos] = Some(pos);
                queue.push_front(*next_pos);
            }
        }
    }

    let mut n_jumps = 0;
    let mut pos = n - 1;
    while pos != 0 {
        pos = pred[pos].unwrap();
        n_jumps += 1;
    }

    return n_jumps;
}

fn main() {
    let input = vec![2, 3, 1, 1, 4];
    // let input = vec![3, 2, 1, 0, 4]; // you cannot
    let n_jumps = jump(input);
    println!("Number of jumps: {}", n_jumps);
}
