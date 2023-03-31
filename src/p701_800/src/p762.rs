
struct Solution;

impl Solution {
    // pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    //     let is_prime = [2, 3, 5, 7, 11, 13, 17, 19];
    //     let mut ans_count = 0;
    //     for mut num in left..=right {
    //         if is_prime.contains(&num.count_ones()) {
    //             ans_count += 1;
    //         }
    //     }
    //     ans_count
    // }

    // 更省内存的做法
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let prime_mask = 0b010100010100010101100;
        let mut ans_count = 0;
        for mut num in left..=right {
            if ((1 << num.count_ones()) & prime_mask) != 0 {
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
        let result = Solution::count_prime_set_bits(6, 10);
        assert_eq!(result, 4);

        let result = Solution::count_prime_set_bits(10, 15);
        assert_eq!(result, 5);

        let result = Solution::count_prime_set_bits(842, 888);
        assert_eq!(result, 23);
    }
}
