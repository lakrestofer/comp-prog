fn main() {
    let input = std::fs::read_to_string("./test.txt").expect("could not read input to string");
    // let input = std::fs::read_to_string("./input.txt").expect("could not read input to string");

    let bitvectors: Vec<Vec<u32>> = input
        .lines()
        .map(|line| parse_to_bits(line))
        .collect();
    
    let bitvector_width = {
        let first = &bitvectors.iter().next().expect("could not get the first elem");
        first.len()
    };
    let gamma = {
        let gamma_bits= Vec::new();
        for n in 0..bitvector_width {
            let mut n_ones = 0;
            let mut n_zeros = 0;    
            for vector in bitvectors.iter() {
                if vector[n] == 1 {
                    n_ones += 1;
                } else {
                    n_zeros += 1;
                }
            }
            println!("ones: {}", n_ones);
            println!("zeros: {}", n_zeros);
            if n_ones > n_zeros {
                gamma_bits.push(1)
            } else {

            }
            n_ones = 0;
            n_zeros = 0;
        }
    };
    
        
}

pub fn parse_to_bits(string: &str) -> Vec<u32> {
    let chars = string.chars();
    let bits: Vec<u32> = chars.map(|char| char.to_digit(2).expect("could not parse character as digit")).collect();
    bits
}
