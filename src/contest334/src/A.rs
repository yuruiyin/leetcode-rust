struct Solution;

impl Solution {
    pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut  pre_sum_arr = vec![0; len];
        let mut ans_arr = vec![0; len];
        pre_sum_arr[0] = nums[0];
        for i in 1..len {
            pre_sum_arr[i] = pre_sum_arr[i - 1] + nums[i];
        }

        ans_arr[0] = pre_sum_arr[len - 1] - pre_sum_arr[0];
        for i in 1..len {
            ans_arr[i] = (pre_sum_arr[i - 1] - pre_sum_arr[len - 1] + pre_sum_arr[i]).abs();
        }
        ans_arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = Solution::xxx();
        // assert_eq!(result, 0);
    }
}