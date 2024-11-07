use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut cnt = vec![0; n];
    let mut now_max_cnt = 0;
    let mut now_max_cand = 0;
    for &i in a.iter() {
        cnt[i] += 1;
        if cnt[i] > now_max_cnt {
            now_max_cnt = cnt[i];
            now_max_cand = i;
        } else if cnt[i] == now_max_cnt {
            now_max_cand = now_max_cand.min(i);
        }

        println!("{}", now_max_cand + 1);
    }
}
