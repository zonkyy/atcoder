use std::vec;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut ab: [(usize, usize); n],
    };

    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        for j in 0..=x {
            for k in 0..=ab[i].1 {
                let price = j + ab[i].0 * k;
                if dp[i][j] && price <= x {
                    dp[i + 1][price] = true;
                }
            }
        }
    }

    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}

// // 間に合わない dfs 解答
// fn rec(rest: &Vec<(usize, usize)>, now_i: usize, sum: usize, x: usize) -> bool {
//     if now_i >= rest.len() {
//         return false;
//     }

//     let (coin_val, coin_num) = rest[now_i];
//     for i in 0..=coin_num {
//         let new_sum = sum + coin_val * i;
//         if new_sum > x {
//             return false;
//         } else if new_sum == x {
//             return true;
//         } else {
//             if rec(rest, now_i + 1, new_sum, x) {
//                 return true;
//             }
//         }
//     }

//     false
// }

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         x: usize,
//         mut ab: [(usize, usize); n],
//     };

//     ab.sort_by(|a, b| b.0.cmp(&a.0));
//     if rec(&ab, 0, 0, x) {
//         println!("Yes");
//     } else {
//         println!("No");
//     }
// }
