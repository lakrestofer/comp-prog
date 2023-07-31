struct Solution;

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut friends: Vec<i32> = vec![0; n];

        let mut ball_location = 0;
        let mut round = 1;
        loop {
            friends[ball_location] += 1;
            if friends[ball_location] >= 2 {
                break;
            };
            ball_location = (ball_location + k * round) % n;
            round += 1;
        }

        let losers = friends
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v == 0)
            .map(|(i, _)| (i + 1) as i32)
            .collect();
        losers
    }
}
fn main() {
    let n = 5;
    let k = 2;
    let res = Solution::circular_game_losers(n, k);
    println!("Solution: {:?}", res);
}
