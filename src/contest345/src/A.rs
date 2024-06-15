use std::mem::transmute;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut degree = vec![0; n];
        let mut list_arr: Vec<&Vec<usize>> = vec![&vec![]; n];
        // let res = list_arr[0].unwrap()[1];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if !list_arr[u].is_empty() && !list_arr[v].is_empty() {
                if list_arr[u] == list_arr[v] {
                    continue;
                }

                for node in *list_arr[v] {
                    (*list_arr[u]).push(node);
                    list_arr[*node] = list_arr[u];
                }
            } else if !list_arr[u].is_empty() {
                list_arr[u].push(v);
                list_arr[v] = list_arr[u];
            } else if !list_arr[v].is_empty() {
                list_arr[v].push(u);
                list_arr[u] = list_arr[v];
            } else {
                list_arr[u] = &mut vec![u, v];
                list_arr[v] = list_arr[u];
            }
        }

        let mut visited = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            if visited[i] {
                continue;
            }

            let mut list = list_arr[i];
            if list.is_empty() {
                ans += 1;
                continue;
            }

            let node_count = list.len();
            let mut is_ok = true;
            for node in list {
                if is_ok && degree[*node] != node_count - 1 {
                    is_ok = false;
                }
                visited[*node] = true;
            }

            if is_ok {
                ans += 1;
            }
        }

        ans
    }
}