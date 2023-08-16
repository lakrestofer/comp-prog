struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // how many reports got at least n citations?
        let mut citations_counter: [usize; 1001] = [0; 1001];
        citations.iter().for_each(|n_citations| {
            for n_citations in 0..=*n_citations {
                citations_counter[n_citations as usize] += 1
            }
        });

        let h_index = citations_counter
            .into_iter()
            .enumerate()
            .filter(|(n_citations, n_reports)| n_citations <= n_reports)
            .max_by(|(nc1, nr1), (nc2, nr2)| match nc1.cmp(nc2) {
                std::cmp::Ordering::Equal => nr1.cmp(nr2),
                ord => ord,
            })
            .map(|(nc, _)| nc)
            .unwrap_or(0);

        h_index as i32
    }
}

fn main() {
    // let input = vec![3, 0, 6, 1, 5];
    // let input = vec![1, 3, 1];
    // let input = vec![1, 1];
    let input = vec![3, 0, 6, 1, 5];

    let res = Solution::h_index(input.clone());
    println!("h-index of {:?} is {}", input, res);
}
