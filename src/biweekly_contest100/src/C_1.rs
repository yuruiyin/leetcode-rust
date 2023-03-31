use std::cmp::{max, min};
use std::mem::transmute;
use std::process::id;

struct Data {
    idx: usize,
    num: i64,
}

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut datas = vec![];
        let len = nums.len();
        for i in 0..len {
            datas.push(Data {idx: i, num : nums[i] as i64});
        }
        datas.sort_unstable_by(|a, b| {
            if a.num == b.num {
                a.idx.cmp(&b.idx)
            } else {
                a.num.cmp(&b.num)
            }
        });

        let mut visited = vec![false; len];
        let mut ans = 0i64;
        for data in datas {
            let idx = data.idx;
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            ans += data.num;
            if idx > 0 {
                visited[idx - 1] = true;
            }
            if idx + 1 < len {
                visited[idx + 1] = true;
            }
        }
        ans
    }
}