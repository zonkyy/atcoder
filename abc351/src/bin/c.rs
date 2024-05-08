use std::collections::VecDeque;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut v = VecDeque::new();
    for i in a {
        v.push_back(i);
        while v.len() >= 2 {
            let x = v.pop_back().unwrap();
            let y = v.pop_back().unwrap();
            if x == y {
                v.push_back(x + 1);
            } else {
                v.push_back(y);
                v.push_back(x);
                break;
            }
        }
    }

    println!("{}", v.len());
}
