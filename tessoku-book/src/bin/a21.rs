use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    };
    let mut dp = vec![vec![0; n + 2]; n + 2];

    for r in (1..=n) {
        for c in (1..=n).rev() {
            // ブロック (r, c) をどう作るか
            // → 左端 = ブロック (r-1, c) を取り除く / 右端 = ブロック (r, c+1) を取り除く
            dp[r][c] = cmp::max(
                del_block(&pa, r, c, r - 1) + dp[r - 1][c],
                del_block(&pa, r, c, c + 1) + dp[r][c + 1],
            );
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = ans.max(dp[i][i]);
    }
    println!("{}", ans);
}

// l〜r 現在の範囲、deleted 取り除くブロック
fn del_block(pa: &Vec<(usize, usize)>, l: usize, r: usize, deleted: usize) -> usize {
    if deleted < 1 || pa.len() < deleted {
        return 0;
    }

    let (del_p, del_a) = pa[deleted - 1];
    if del_p < l || r < del_p {
        return 0;
    } else {
        return del_a;
    }
}
