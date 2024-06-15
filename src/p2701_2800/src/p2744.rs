impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut v = [[false; 26]; 26];
        let mut ans = 0;
        for word in words {
            let bytes = word.as_bytes();
            let first = (bytes[0] - b'a') as usize;
            let second = (bytes[1] - b'a') as usize;
            if v[second][first] {
                ans += 1;
            } else {
                v[first][second] = true;
            }
        }
        return ans;
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::maximum_number_of_string_pairs(vec![
            "cd".to_string(), "ac".to_string(), "dc".to_string(), "ca".to_string(), "zz".to_string(),
        ]), 2);
    }

}
