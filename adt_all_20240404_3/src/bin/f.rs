use std::collections::VecDeque;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n],
    };
    f.sort();
    f.reverse();

    let mut ans = 0;
    for chunk in f.chunks(d) {
        let s = chunk.iter().sum::<usize>();
        if s <= p {
            ans += s;
        } else {
            ans += p;
        }
    }

    println!("{:?}", ans);
}
