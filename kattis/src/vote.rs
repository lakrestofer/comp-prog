use std::{
    collections::{BinaryHeap, HashMap},
    io::{stdin, Read},
    str::FromStr,
};

#[derive(PartialEq, PartialOrd, Eq)]
struct CandVotes {
    pub cand_id: u32,
    pub votes: u32,
}

impl Ord for CandVotes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.votes.cmp(&other.votes)
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    let t: u32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n_cands = lines.next().unwrap().parse().unwrap();

        let mut total = 0;

        let mut pq = BinaryHeap::new();

        for i in 0..n_cands {
            let votes: u32 = lines.next().unwrap().parse().unwrap();
            total += votes;
            pq.push(CandVotes {
                cand_id: i,
                votes: votes,
            });
        }

        let cand1: CandVotes = pq.pop().unwrap();
        let cand2: CandVotes = pq.pop().unwrap();
        if cand1.votes == cand2.votes {
            println!("no winner")
        } else if cand1.votes as f32 / total as f32 > 0.5 {
            println!("majority winner {}", cand1.cand_id)
        } else {
            println!("minority winner {}", cand1.cand_id)
        }
    }
}

// fn main() {
//     let mut buffer = String::new();
//     stdin().read_to_string(&mut buffer).unwrap();

//     let mut lines = buffer.lines();

//     let T = lines.next().unwrap().parse().unwrap();

//     for _ in 0..T {
//         let n_candidates = lines.next().unwrap().parse().unwrap();
//         let mut total = 0;
//         let mut votes = Vec::with_capacity(n_candidates);

//         for _ in 0..n_candidates {
//             let vote: u32 = lines.next().unwrap().parse().unwrap();
//             total += vote;
//             votes.push(vote);
//         }

//         let (winner, max): (usize, u32) = votes
//             .iter()
//             .enumerate()
//             .map(|(c, v)| (c + 1, *v))
//             .max()
//             .unwrap();

//         let is_valid = votes.iter().filter(|v| **v == max).count() == 1;

//         if !is_valid {
//             println!("no winner");
//         } else {
//             if max > (total / 2) {
//                 println!("majority winner {}", winner);
//             } else {
//                 println!("minority winner {}", winner);
//             }
//         }
//     }
// }
