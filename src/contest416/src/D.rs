impl Solution {
    fn is_ok(count_arr1: &Vec<i32>, count_arr2: &Vec<i32>) -> bool {
        for i in 0..26 {
            if count_arr1[i] < count_arr2[i] {
                return false;
            }
        }
        true
    }

    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut count_arr1 = vec![0; 26];
        let mut count_arr2 = vec![0; 26];
        word2.as_bytes().into_iter().for_each(|&c| {
            count_arr2[(c - b'a') as usize] += 1;
        });

        let len1 = word1.len();
        let mut bytes1 = word1.as_bytes();
        let mut ans = 0;
        let mut l = 0;
        let mut r = 0;

        // 先计算一遍，使得count_arr1中的值大于等于count_arr2
        let mut is_found = false;
        while r < len1 {
            count_arr1[(bytes1[r] - b'a') as usize] += 1;
            r += 1;
            if Self::is_ok(&count_arr1, &count_arr2) {
                is_found = true;
                ans += len1 - r + 1;
                break;
            }
        }

        if !is_found {
            return 0;
        }

        let mut miss_idx = -1;

        while l < r {
            let idx = (bytes1[l] - b'a') as usize;
            count_arr1[idx] -= 1;
            l += 1;
            if count_arr1[idx] >= count_arr2[idx] {
                ans += len1 - r + 1;
            } else {
                miss_idx = idx as i32;
                break;
            }
        }

        while l <= r {
            let mut is_found = false;
            while r < len1 {
                let idx = (bytes1[r] - b'a') as usize;
                count_arr1[idx] += 1;
                r += 1;
                if idx as i32 == miss_idx {
                    is_found = true;
                    ans += len1 - r + 1;
                    break;
                }
            }

            if !is_found {
                break;
            }

            while l < r {
                let idx = (bytes1[l] - b'a') as usize;
                count_arr1[idx] -= 1;
                l += 1;
                if count_arr1[idx] >= count_arr2[idx] {
                    ans += len1 - r + 1;
                } else {
                    miss_idx = idx as i32;
                    break;
                }
            }
        }

        ans as i64
    }
}

struct Solution;
