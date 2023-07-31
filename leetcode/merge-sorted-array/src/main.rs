struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let mn = m + n;
        let mut buffer = [0; 200];
        let mut nums1_iter = nums1.iter().take(m).peekable();
        let mut nums2_iter = nums2.iter().peekable();

        for i in 0..mn {
            let v1 = nums1_iter.peek();
            let v2 = nums2_iter.peek();
            buffer[i] = match (v1, v2) {
                (None, None) => 0,
                (None, Some(&&v2)) => {
                    nums2_iter.next();
                    v2
                }
                (Some(&&v1), None) => {
                    nums1_iter.next();
                    v1
                }
                (Some(&&v1), Some(&&v2)) => {
                    if v1 < v2 {
                        nums1_iter.next();
                        v1
                    } else {
                        nums2_iter.next();
                        v2
                    }
                }
            }
        }

        for i in 0..mn {
            nums1[i] = buffer[i];
        }
    }
}
fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    println!("input: v1 {:?}, v2: {:?}", nums1, nums2);
    Solution::merge(&mut nums1, m as i32, &mut nums2, n as i32);
    println!("after: {:?}", nums1);
}
