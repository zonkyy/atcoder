use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    };
    let mut v = vec![0; 30];
    for i in 0..n {
        v[i] = a[i];
    }

    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for i in 1..=15 {
        for comb in (0..15).combinations(i) {
            let mut tmp1 = 0;
            let mut tmp2 = 0;
            for &idx in comb.iter() {
                tmp1 += v[idx];
                tmp2 += v[idx + 15];
            }
            v1.push(tmp1);
            v2.push(tmp2);
        }
    }

    v1.sort();
    v2.sort();
    for &i in v1.iter() {
        if v2.binary_search(&(k - i)).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
