use proconio::{fastout, input, marker::*};

fn rec(rest: &Vec<(usize, usize)>, now_i: usize, sum: usize, x: usize) -> bool {
    if now_i >= rest.len() {
        return false;
    }

    let (coin_val, coin_num) = rest[now_i];
    for i in 0..=coin_num {
        let new_sum = sum + coin_val * i;
        if new_sum > x {
            return false;
        } else if new_sum == x {
            return true;
        } else {
            if rec(rest, now_i + 1, new_sum, x) {
                return true;
            }
        }
    }

    false
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        mut ab: [(usize, usize); n],
    };

    ab.sort_by(|a, b| b.0.cmp(&a.0));
    if rec(&ab, 0, 0, x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
