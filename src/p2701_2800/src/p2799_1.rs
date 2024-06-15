impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let n = nums.len();
        let mut l = 0;
        let mut r = 0;
        let mut max_count = 1;
        while r < n {
            if nums[r] - k > nums[l] + k {
                l += 1;
            } else {
                r += 1;
            }
            max_count = max_count.max(r - l);
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
