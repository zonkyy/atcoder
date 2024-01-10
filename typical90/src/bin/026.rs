use proconio::input;

fn output(group: &Vec<usize>, target: usize, size: usize) {
    let a = group
        .iter()
        .enumerate()
        .filter(|(_, i)| **i == target)
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<usize>>();

    println!(
        "{}",
        &a[0..size]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

    for (a, b) in ab {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    let mut group: Vec<usize> = vec![0; n];
    group[0] = 1;
    let mut cnt_one: usize = 0;
    let mut rest: Vec<usize> = vec![0];
    while let Some(i) = rest.pop() {
        let one_or_two = if group[i] == 1 { 2 } else { 1 };
        for j in adj[i].iter() {
            if group[*j] == 0 {
                group[*j] = one_or_two;
                rest.push(*j);
                if one_or_two == 1 {
                    cnt_one += 1;
                }
            }
        }
    }

    if cnt_one >= n / 2 {
        output(&group, 1, n / 2);
    } else {
        output(&group, 2, n / 2);
    }
}
