use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn rotated_digits(n: i32) -> i32 {
        let mut ans_count = 0;
        let mirror_set = [2, 5, 6, 9];
        let none_set = [3, 4, 7];
        for mut num in 1..=n {
            let mut is_ok = false;
            while num > 0 {
                let mold = num % 10;
                if none_set.contains(&mold) {
                    is_ok = false;
                    break;
                }
                if mirror_set.contains(&mold) {
                    is_ok = true;
                }
                num /= 10;
            }
            if is_ok {
                ans_count += 1;
            }
        }
        ans_count
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::rotated_digits(10);
        assert_eq!(result, 4);
    }
}
