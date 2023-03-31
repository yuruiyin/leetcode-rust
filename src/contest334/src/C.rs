use std::io::Chain;

struct Solution;

impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        let mut ans_count = 0;
        nums.sort_unstable();
        let len = nums.len();
        let mut r = len >> 1;
        for l in 0..(len >> 1) {
            let r_value = nums[l] * 2;
            while r < len {
                if nums[r] >= r_value {
                    ans_count += 2;
                    r += 1;
                    break;
                }
                r += 1;
            }
            if r == len {
                break
            }
        }
        ans_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::divisibility_array(String::from("998244353"), 3);
        assert_eq!(result, vec![1, 1, 0, 0, 0, 1, 1, 0, 0]);
    }
}