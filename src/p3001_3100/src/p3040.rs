use std::collections::{HashMap, HashSet};

impl Solution {
    fn rec(l: i32, idx: i32, score: i32, nums: &[i32], n: i32, memo: &mut HashMap<i32, i32>, done: &mut bool) -> i32 {
        if *done {
            return 0;
        }
        let r = n - idx * 2 + l - 1;
        if l >= r {
            *done = true;
            return 0;
        }

        let key = l * 1000 + idx;
        if memo.contains_key(&key) {
            return *memo.get(&key).unwrap();
        }

        let mut res_max = 0;
        if nums[l as usize] + nums[(l + 1) as usize] == score {
            res_max = 1 + Self::rec(l + 2, idx + 1, score, nums, n, memo, done);
        }

        if nums[(r - 1) as usize] + nums[r as usize] == score {
            res_max = res_max.max(1 + Self::rec(l, idx + 1, score, nums, n, memo, done));
        }

        if nums[l as usize] + nums[r as usize] == score {
            res_max = res_max.max(1 + Self::rec(l + 1, idx + 1, score, nums, n, memo, done));
        }

        memo.insert(key, res_max);
        res_max
    }

    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut score_set = HashSet::new();
        score_set.insert(nums[0] + nums[1]);
        score_set.insert(nums[n - 2] + nums[n - 1]);
        score_set.insert(nums[0] + nums[n - 1]);
        let mut done = false;
        let mut ans_max = 0;
        for score in score_set {
            ans_max = ans_max.max(Self::rec(0, 0, score, &nums, n as i32, &mut HashMap::new(), &mut done));
        }
        ans_max
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {}
