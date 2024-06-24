
impl Solution {

    fn calc_min_area(grid: &Vec<Vec<i32>>, left_row: i32, left_col: i32, right_row: i32, right_col: i32) -> i32 {
        let mut min_row = right_row;
        let mut max_row = left_row;
        let mut min_col = right_col;
        let mut max_col = left_col;
        let mut hash_one = false;
        for i in left_row..=right_row {
            for j in left_col..=right_col {
                if grid[i as usize][j as usize] == 1 {
                    hash_one = true;
                    min_row = min_row.min(i);
                    max_row = max_row.max(i);
                    min_col = min_col.min(j);
                    max_col = max_col.max(j);
                }
            }
        }

        if !hash_one {
            return -1;
        }

        (max_row - min_row + 1) * (max_col - min_col + 1)
    }

    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut ans_min = m * n;
        for col1 in 0..n - 2 {
            for col2 in col1 + 1..n - 1 {
                let area1 = Self::calc_min_area(&grid, 0, 0, m - 1, col1);
                let area2 = Self::calc_min_area(&grid, 0, col1 + 1, m - 1, col2);
                let area3 = Self::calc_min_area(&grid, 0, col2 + 1, m - 1, n - 1);
                if (area1 != -1 && area2 != -1 && area3 != -1) {
                    ans_min = ans_min.min(area1 + area2 + area3);
                }
            }
        }

        for row1 in 0..m - 2 {
            for row2 in row1 + 1..m - 1 {
                let area1 = Self::calc_min_area(&grid, 0, 0, row1, n - 1);
                let area2 = Self::calc_min_area(&grid, row1 + 1, 0, row2, n - 1);
                let area3 = Self::calc_min_area(&grid, row2 + 1, 0, m - 1, n - 1);
                if (area1 != -1 && area2 != -1 && area3 != -1) {
                    ans_min = ans_min.min(area1 + area2 + area3);
                }
            }
        }

        for row in 0..m - 1 {
            for col in 0..n - 1 {
                let area1 = Self::calc_min_area(&grid, 0, 0, row, col);
                let area2 = Self::calc_min_area(&grid, 0, col + 1, row, n - 1);
                let area3 = Self::calc_min_area(&grid, row + 1, 0, m - 1, n - 1);
                if (area1 != -1 && area2 != -1 && area3 != -1) {
                    ans_min = ans_min.min(area1 + area2 + area3);
                }
            }
        }

        for row in 0..m - 1 {
            for col in 0..n - 1 {
                let area1 = Self::calc_min_area(&grid, 0, 0, row, n - 1);
                let area2 = Self::calc_min_area(&grid, row + 1, 0, m - 1, col);
                let area3 = Self::calc_min_area(&grid, row + 1, col + 1, m - 1, n - 1);
                if (area1 != -1 && area2 != -1 && area3 != -1) {
                    ans_min = ans_min.min(area1 + area2 + area3);
                }
            }
        }

        for row in 0..m - 1 {
            for col in 0..n - 1 {
                let area1 = Self::calc_min_area(&grid, 0, 0, m - 1, col);
                let area2 = Self::calc_min_area(&grid, 0, col + 1, row, n - 1);
                let area3 = Self::calc_min_area(&grid, row + 1, col + 1, m - 1, n - 1);
                if (area1 != -1 && area2 != -1 && area3 != -1) {
                    ans_min = ans_min.min(area1 + area2 + area3);
                }
            }
        }

        for row in 0..m - 1 {
            for col in 0..n - 1 {
                let area1 = Self::calc_min_area(&grid, 0, 0, row, col);
                let area2 = Self::calc_min_area(&grid, row + 1, 0, m - 1, col);
                let area3 = Self::calc_min_area(&grid, 0, col + 1, m - 1, n - 1);
                if (area1 != -1 && area2 != -1 && area3 != -1) {
                    ans_min = ans_min.min(area1 + area2 + area3);
                }
            }
        }

        ans_min
    }
}

struct Solution;
