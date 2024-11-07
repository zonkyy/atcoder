use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [[char; n]; m],
    };
    let BIT_MAX = 2_i32.pow(10) as usize;

    let coupons = a
        .iter()
        .map(|bits| usize::from_str_radix(&bits.iter().collect::<String>(), 2).unwrap())
        .collect::<Vec<usize>>();

    let mut dp = vec![vec![usize::MAX; BIT_MAX]; m + 1];
    dp[0][0] = 0;

    for r in 0..m {
        for c in 0..BIT_MAX {
            if dp[r][c] != usize::MAX {
                dp[r + 1][c] = dp[r + 1][c].min(dp[r][c]);
                dp[r + 1][c | coupons[r]] = dp[r + 1][c | coupons[r]].min(dp[r][c] + 1);
            }
        }
    }

    let ans = dp[m][(2_i32.pow(n as u32) as usize) - 1];
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }

    // for r in 0..(m + 1) {
    //     for c in 0..10 {
    //         if dp[r][c] == usize::MAX {
    //             print!("x ");
    //         } else {
    //             print!("{:?} ", dp[r][c]);
    //         }
    //     }
    //     println!();
    // }
}
