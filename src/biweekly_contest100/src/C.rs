use std::cmp::{max, min};
use std::mem::transmute;
use std::process::id;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut datas: Vec<(i64, i64)> = vec![];
        let len = nums.len();
        for i in 0..len {
            datas.push((i as i64, nums[i] as i64));
        }
        datas.sort_unstable_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut visited = vec![false; len];
        let mut ans = 0i64;
        for data in datas {
            let idx = data.0 as usize;
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            ans += data.1;
            if idx > 0 {
                visited[idx - 1] = true;
            }
            if idx + 1 < len{
                visited[idx + 1] = true;
            }
        }
        ans
    }
}