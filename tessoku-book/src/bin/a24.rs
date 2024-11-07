use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = Vec::new();
    for i in a.iter() {
        let idx = match dp.binary_search(i) {
            Ok(v) => v,
            Err(v) => v,
        };

        if idx >= dp.len() {
            dp.push(*i);
        } else {
            dp[idx] = *i;
        }
    }

    println!("{}", dp.len());
}
