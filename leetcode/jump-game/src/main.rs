pub fn can_jump(jumps_array: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut current_position = 0;
    let goal = jumps_array.len() - 1;

    // if we can reach the goal from the start, then just return early
    if current_position + jumps_array[current_position] as usize >= goal {
        return true;
    }

    // let mut stack = Vec::with_capacity(nums.len());
    let mut possible_jumps: Vec<usize> = Vec::new();
    let mut jumps_taken: Vec<usize> = Vec::new();
    possible_jumps.push(jumps_array[current_position] as usize);
    let mut impossible_jumps: HashSet<(usize, usize)> = HashSet::new();

    // for as long as there exists jumps to make and as long as we are behind the goal
    while !possible_jumps.is_empty() && current_position < goal {
        // how many jumps can we perform from our current position?
        let jumps = possible_jumps.pop().unwrap();

        // EXIT CONDITION
        // if it possible to jump to the goal from here, do that and exit the loop
        if goal < current_position + jumps || jumps == 0 && (current_position == 0) {
            current_position += jumps;
            jumps_taken.push(jumps);
            break;
        }

        // CONTINUE CONDITION
        if jumps == 0 {
            // if we can take no more jumps from here
            // then go back the way we came and try again
            impossible_jumps.insert((current_position, jumps));
            current_position -= jumps_taken.pop().unwrap();
            continue;
        }

        // JUMPING
        // jump to next location
        // the number of jumps available on the destination
        let destination = current_position + jumps;
        let next_jumps = jumps_array[destination];

        if impossible_jumps.contains(&(destination, next_jumps as usize)) {
            impossible_jumps.insert((current_position, jumps));
            current_position -= jumps_taken.pop().unwrap();
            continue;
        }

        possible_jumps.push(jumps - 1);
        possible_jumps.push(next_jumps as usize);
        jumps_taken.push(jumps);
        current_position = destination;
    }

    // return whether we reached beyond the goal
    return current_position >= goal;
}

fn main() {
    // let input = vec![2, 3, 1, 1, 4];
    let input = vec![3, 2, 1, 0, 4]; // you cannot
    println!(
        "can you jump? {}",
        if can_jump(input) {
            "yes I can"
        } else {
            "no I can't"
        }
    )
}
