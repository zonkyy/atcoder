use proconio::input;

fn count_child(tree: &Vec<Vec<usize>>, node: usize, last: usize) -> usize {
    let mut cnt = 0;
    for i in tree[node].iter() {
        if i != &last {
            cnt += count_child(tree, *i, node);
        }
    }
    cnt + 1
}

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n-1],
    };

    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in uv {
        tree[u - 1].push(v - 1);
        tree[v - 1].push(u - 1);
    }

    // 頂点 1 が葉なら 1 回
    if tree[0].len() == 1 {
        println!("1");
        return;
    }

    // 頂点 1 が葉でないなら、頂点 1 の子の木のうち、最大のもののノード数の和 + 1 回
    let mut counts: Vec<usize> = vec![];
    for i in &tree[0] {
        let cnt = count_child(&tree, *i, 0);
        counts.push(cnt);
    }
    counts.sort();
    counts.pop();
    println!("{}", counts.iter().sum::<usize>() + 1);
}
