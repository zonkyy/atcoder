use proconio::input;

fn dfs(n: usize, start_node: usize, adj_list: &Vec<Vec<usize>>) -> Vec<isize> {
    let mut dists: Vec<isize> = vec![-1; n + 1];
    dists[start_node] = 0;
    let mut st = vec![start_node];
    while !st.is_empty() {
        let node = st.pop().unwrap();
        for adj in &adj_list[node] {
            if dists[*adj] == -1 {
                dists[*adj] = dists[node] + 1;
                st.push(*adj);
            }
        }
    }

    dists
}

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize); n-1],
    }

    let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for (a, b) in edges {
        adj_list[a].push(b);
        adj_list[b].push(a);
    }

    let dists = dfs(n, 1, &adj_list);
    let (max_index, _) =
        dists
            .iter()
            .enumerate()
            .fold((std::usize::MIN, std::isize::MIN), |(ai, a), (bi, &b)| {
                if b > a {
                    (bi, b)
                } else {
                    (ai, a)
                }
            });

    let dists2 = dfs(n, max_index, &adj_list);
    println!("{}", dists2.iter().max().unwrap() + 1);
}
