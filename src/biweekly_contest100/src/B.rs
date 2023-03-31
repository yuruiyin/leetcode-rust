
impl Solution {
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        for x in &nums {
            if x > &nums[ans] {
                ans += 1;
            }
        }
        ans as i32
    }
}