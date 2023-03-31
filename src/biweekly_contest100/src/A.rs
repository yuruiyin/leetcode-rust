use std::cmp::max;

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        let left = money - children;
        if left < 0 {
            return -1;
        }
        let count = left / 7;
        let mo = left % 7;
        if count > children {
            return children - 1;
        } else if count == children {
            if mo == 0 {
                return count;
            }
            return count - 1;
        } else {
            if mo == 3 {
                if children == 1 {
                    return -1;
                }
                if count == children - 1 {
                    return max(0, count - 1);
                }
                return count;
            }
            return count;
        }
    }
}