use std::collections::{BTreeSet, VecDeque};

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    const UPPER: usize = 3 * 10usize.pow(5) + 9;
    let b: BTreeSet<usize> = a.iter().cloned().collect();
    let mut dq: VecDeque<usize> = b.iter().cloned().collect();
    for _ in 0..(n - b.len()) {
        dq.push_back(UPPER);
    }

    let mut series = 1;
    loop {
        if dq.len() >= 1 && *dq.front().unwrap() == series {
            dq.pop_front();
            series += 1;
        } else if dq.len() >= 2 {
            dq.pop_back();
            dq.pop_back();
            series += 1;
        } else {
            break;
        }
    }

    println!("{}", series - 1);
}
