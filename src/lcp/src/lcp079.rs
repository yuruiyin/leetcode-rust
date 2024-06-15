use std::io::Read;

struct Solution;

impl Solution {

    pub fn extract_mantra(matrix: Vec<String>, mantra: String) -> i32 {
        let matrix = matrix.iter().map(|e| e.as_bytes()).collect::<Vec<_>>();
        let arr = mantra.as_bytes();
        let m = matrix.len();
        let n = matrix[0].len();
        let l = arr.len();
        let mut dis: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; n]; m]; l + 1];
        let mut node_arr = vec![(0, 0, 0); m * n * l];
        node_arr[0] = (0, 0, 0);
        let mut size = 1;
        let mut now_idx = 0;
        while now_idx < size {
            let top = node_arr[now_idx];
            now_idx += 1;
            let mut r: usize = top.0;
            let mut c: usize = top.1;
            let mut idx: usize = top.2;
            let cur = dis[idx][r][c];
            let cur_char = matrix[r][c];
            while idx < l && cur_char == arr[idx] {
                dis[idx + 1][r][c] = dis[idx][r][c];
                idx += 1;
            }
            if idx == l {
                return dis[idx][r][c] + l as i32;
            }

            if r >= 1 && dis[idx][r - 1][c] == 0 {
                dis[idx][r - 1][c] = cur + 1;
                node_arr[size] = (r - 1, c, idx);
                size += 1;
            }
            if r + 1 < m && dis[idx][r + 1][c] == 0 {
                dis[idx][r + 1][c] = cur + 1;
                node_arr[size] = (r + 1, c, idx);
                size += 1;
            }
            if c >= 1 && dis[idx][r][c - 1] == 0 {
                dis[idx][r][c - 1] = cur + 1;
                node_arr[size] = (r, c - 1, idx);
                size += 1;
            }
            if c + 1 < n && dis[idx][r][c + 1] == 0 {
                dis[idx][r][c + 1] = cur + 1;
                node_arr[size] = (r, c + 1, idx);
                size += 1;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Solution::extract_mantra(vec![], "".to_string());
        // let result = add(2, 2);
        assert_eq!(4, 4);
    }
}
