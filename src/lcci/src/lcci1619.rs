impl Solution {

    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let row_count = land.len();
        let col_count = land.get(0).unwrap().len();
        let mut ans_list = vec![];
        for i in 0..row_count {
            for j in 0..col_count {
                if land[i][j] != 0 {
                    continue;
                }
                ans_list.push(dfs(&mut land, i, j, row_count, col_count));
            }
        }
        ans_list.sort_unstable();
        ans_list
    }
}

fn dfs(land: &mut Vec<Vec<i32>>, r: usize, c: usize, row_count: usize, col_count: usize) -> i32 {
    if r < 0 || r >= row_count || c < 0 || c >= col_count || land[r][c] != 0 {
        return 0;
    }

    land[r][c] = -1;
    1 + dfs(land, r - 1, c - 1, row_count, col_count) +
        dfs(land, r - 1, c, row_count, col_count) +
        dfs(land, r - 1, c + 1, row_count, col_count) +
        dfs(land, r, c - 1, row_count, col_count) +
        dfs(land, r, c + 1, row_count, col_count) +
        dfs(land, r + 1, c - 1, row_count, col_count) +
        dfs(land, r + 1, c, row_count, col_count) +
        dfs(land, r + 1, c + 1, row_count, col_count)
}