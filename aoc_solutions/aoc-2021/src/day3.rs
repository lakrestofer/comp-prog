pub fn solve_first(input: String) {
    let mut lines = input.lines().peekable();

    let width = lines.peek().unwrap().len();
    let mut counter = vec![0; width];
    let mut length = 0;

    for line in input.lines() {
        length += 1;
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                counter[index] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for (exp, bit_count) in counter.iter().rev().enumerate() {
        if *bit_count > length / 2 {
            gamma += 1 << exp;
        } else {
            epsilon += 1 << exp;
        }
    }
    let consumption_rate = gamma * epsilon;
    println!("rate: {}", consumption_rate);
}
pub fn solve_second(input: String) {
    let mut lines = input.lines().peekable(); // lines of binary numbers
    let width = lines.peek().unwrap().len();
    let n_possible_values = 1 << width; // with 5 bits then 2 ^ 5 values can be expressed

    let input_vec = lines.map(str_to_int).collect::<Vec<usize>>();
    println!("input: {:?}", input_vec);
    // we take the input, convert each base 2 number and then sort
    let sorted_input = counting_sort(n_possible_values, input_vec);
    println!("output: {:?}", sorted_input);

    // TODO Now that I have the input sorted, wtf do I do now?
}

fn counting_sort(max: usize, input: Vec<usize>) -> Vec<usize> {
    // we sort the values using count sort
    let mut register = vec![0; max + 1];
    let mut length = input.len();
    for num in &input {
        register[*num] = 1;
    }
    for i in 1..register.len() {
        register[i] += register[i - 1];
    }
    let mut output = vec![0; length];
    for i in (0..length).rev() {
        let value = input[i];
        let output_index = register[value] - 1;
        output[output_index] = value;
        register[value] -= 1;
    }
    output
}

fn str_to_int(s: &str) -> usize {
    let mut sum = 0;
    for (exp, char) in s.chars().rev().enumerate() {
        if char == '1' {
            sum += if exp == 0 { 1 } else { 1 << exp };
        }
    }
    sum
}
