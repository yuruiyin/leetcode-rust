use std::io::Chain;

struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let len = word.len();
        let mut ans_arr = vec![0; len];
        let mut pre = 0i64;
        let bytes = word.as_bytes();
        let m = m as i64;
        for i in 0..len {
            pre = pre * 10 + (bytes[i] - b'0') as i64;
            pre %= m;
            if pre == 0 {
                ans_arr[i] = 1;
            }
        }
        ans_arr
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