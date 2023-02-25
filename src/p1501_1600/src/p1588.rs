pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut pre_sum_arr = vec![0; len];
    pre_sum_arr[0] = arr[0];
    for i in 1..len {
        pre_sum_arr[i] = pre_sum_arr[i - 1] + arr[i];
    }
    let mut res = 0;
    let mut l = 1;
    while l <= len {
        for e in (l - 1)..len {
            res += pre_sum_arr[e];
            if e != l - 1 {
                res -= pre_sum_arr[e - l];
            }
        }
        l += 2;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]);
        assert_eq!(result, 58);
    }
}
