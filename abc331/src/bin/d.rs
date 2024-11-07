use proconio::{fastout, input, marker::*};

fn calc(acc: &Vec<Vec<usize>>, n: usize, r: usize, c: usize) -> usize {
    // 完全な正方形部分
    let mut res = acc[n - 1][n - 1] * (r / n) * (c / n);
    // 右端の縦長部分
    res += acc[n - 1][c % n] * (r / n);
    // 下端の横長部分
    res += acc[r % n][n - 1] * (c / n);
    // 右下の余った部分
    res += acc[r % n][c % n];

    return res;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        queries: [(usize, usize, usize, usize); q],
    };

    let mut acc = vec![vec![0; n]; n];
    for r in 0..n {
        let mut cnt = 0;
        for c in 0..n {
            if p[r][c] == 'B' {
                cnt += 1;
            }
            acc[r][c] = cnt;
        }
    }

    for c in 0..n {
        for r in 1..n {
            acc[r][c] += acc[r - 1][c];
        }
    }

    for &(a, b, c, d) in queries.iter() {
        let mut ans = calc(&acc, n, c, d);
        if a > 0 && b > 0 {
            ans += calc(&acc, n, a - 1, b - 1);
        }
        if a > 0 {
            ans -= calc(&acc, n, a - 1, d);
        }
        if b > 0 {
            ans -= calc(&acc, n, c, b - 1);
        }
        println!("{}", ans);
    }
}
