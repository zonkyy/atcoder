use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    const LIMIT: usize = 1000000;

    let mut ans = vec![true; LIMIT + 1];
    let mut idx = 2;
    while idx < 1005 {
        if !ans[idx] {
            idx += 1;
            continue;
        }

        for i in (idx + 1)..=LIMIT {
            if i % idx == 0 {
                ans[i] = false;
            }
        }

        idx += 1;
    }

    for i in 2..=n {
        if ans[i] {
            println!("{}", i);
        }
    }
}
