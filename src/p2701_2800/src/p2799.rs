impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        let mut end_queue = vec![0; nums.len()];
        nums.sort_unstable();
        let n = nums.len();
        end_queue[0] = nums[0] + k;
        let mut start = 0;
        let mut end = 0;
        let mut max_count = 1;
        for i in 1..n {
            let l = nums[i] - k;
            let r = nums[i] + k;
            if l > end_queue[start] {
                start += 1;
            }
            end += 1;
            end_queue[end] = r;
            max_count = max_count.max(end - start + 1);
        }
        max_count as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let nums = vec![4, 6, 1, 2];
        let k = 2;
        let res = super::Solution::maximum_beauty(nums, k);
        assert_eq!(res, 3);
    }
}
