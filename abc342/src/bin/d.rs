use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

pub fn factorization(mut n: usize) -> HashMap<usize, usize> {
    let mut map = HashMap::new();

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }

    if n != 0 && n != 1 {
        *map.entry(n).or_insert(0) += 1;
    }

    map
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    const UPPER: usize = 2 * 10usize.pow(5) + 9;

    // 平方数に必要な数に分類して個数をカウント
    let mut v = vec![0; UPPER];
    let mut zerocnt = 0;
    for i in a {
        if i == 0 {
            zerocnt += 1;
            continue;
        }

        let f = factorization(i);
        let mut g = 1;
        for (k, v) in f.iter() {
            if v % 2 != 0 {
                g *= k;
            }
        }
        v[g] += 1;
    }

    let mut ans = 0;

    // 組を数える
    for i in 1..UPPER {
        if v[i] >= 2 {
            ans += v[i] * (v[i] - 1) / 2;
        }
    }

    // 0 を掛けたらなんでも平方数
    if zerocnt > 0 {
        // 0 同士の組
        ans += zerocnt * (zerocnt - 1) / 2;
        // 0 以外と 0 の組
        ans += zerocnt * (n - zerocnt);
    }

    println!("{}", ans);
}
