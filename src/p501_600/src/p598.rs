// use std::cmp::min;

pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    let (mut min_x, mut min_y) = (m, n);
    for item in &ops {
        min_x = min_x.min(item[0]);
        min_y = min_y.min(item[1]);
    }
    min_x * min_y
}

// pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
//     let (mut min_x, mut min_y) = (m, n);
//     for item in ops.iter() {
//         min_x = min_x.min(item[0]);
//         min_y = min_y.min(item[1]);
//     }
//     return min_x * min_y;
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        let result = max_count(3, 3, vec![vec![2, 2], vec![3, 3]]);
        assert_eq!(result, 4);
        // 3, n = 3, ops = [[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3]]
        let result = max_count(3, 3, vec![vec![2, 2], vec![3, 3], vec![3, 3],
                                          vec![3, 3], vec![2, 2], vec![3, 3], vec![3, 3],
                                          vec![3, 3], vec![2, 2], vec![3, 3], vec![3, 3], vec![3, 3]],
        );
        assert_eq!(result, 4);
        let result = max_count(3, 3, vec![]);
        assert_eq!(result, 9);
    }
}

