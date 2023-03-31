
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut l = 0;
        let mut r = 1e15 as i64;
        let mut ans = -1;
        while l <= r {
            let mid = (l + r) >> 1;
            if is_ok(&ranks, mid as f64, cars) {
                ans = mid as i64;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}

fn is_ok(ranks: &Vec<i32>, max: f64, cars: i32) -> bool {
    let mut count = 0;
    let cars = cars as i64;
    for rank in ranks {
        let left = (max / *rank as f64);
        count += left.sqrt() as i64;
        if count >= cars {
            return true;
        }
    }
    return false;
}