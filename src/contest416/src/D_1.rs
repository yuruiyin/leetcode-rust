impl Solution {
    pub fn valid_substring_count(s: String, t: String) -> i64 {
        let mut count_arr = vec![0; 26];
        t.as_bytes().into_iter().for_each(|&c| {
            count_arr[(c - b'a') as usize] += 1;
        });

        let mut less = 0;
        for &cnt in &count_arr {
            if cnt > 0 {
                less += 1;
            }
        }

        // 滑动窗口
        let mut ans = 0;
        let mut l = 0;
        let mut s = s.as_bytes();
        for b in s {
            let idx = (b - b'a') as usize;
            count_arr[idx] -= 1;
            if count_arr[idx] == 0 {
                // 窗口内 b 出现的次数和 word2 中 b 出现的次数相等
                less -= 1;
            }
            while less == 0 {
                let idx = (s[l] - b'a') as usize;
                l += 1;
                if count_arr[idx] == 0 {
                    // s[left] 移出窗口后，窗口内 s[left] 的出现次数比 t 的少
                    less += 1;
                }
                count_arr[idx] += 1;
            }
            ans += l as i64;
        }

        ans
    }
}

struct Solution;
