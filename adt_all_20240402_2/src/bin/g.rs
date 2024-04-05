use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn dfs(
    tree: &HashMap<usize, Vec<(usize, (i64, i64))>>,
    ans: &mut Vec<Option<(i64, i64)>>,
    now: usize,
) {
    if let Some(dests) = tree.get(&now) {
        for &(next, (x, y)) in dests {
            if ans[next].is_some() {
                continue;
            }

            ans[next] = Some((ans[now].unwrap().0 + x, ans[now].unwrap().1 + y));
            dfs(tree, ans, next);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: [(Usize1, Usize1, i64, i64); m],
    };

    let mut ans: Vec<Option<(i64, i64)>> = vec![None; n];
    ans[0] = Some((0, 0));

    let mut tree = HashMap::new();
    for (a, b, x, y) in q {
        tree.entry(a).or_insert(Vec::new()).push((b, (x, y)));
        tree.entry(b).or_insert(Vec::new()).push((a, (-x, -y)));
    }

    dfs(&tree, &mut ans, 0);

    for a in ans {
        if let Some(a) = a {
            println!("{} {}", a.0, a.1);
        } else {
            println!("undecidable");
        }
    }
}
