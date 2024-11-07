use std::{cmp, collections::BTreeSet};

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut buf = Vec::new();
    let mut next = 'A';
    for &c in s.iter() {
        match (next, c) {
            (_, 'A') => {
                buf.push(c);
                next = 'B';
            }
            ('B', 'B') => {
                buf.push(c);
                next = 'C';
            }
            ('C', 'C') => {
                buf.pop();
                buf.pop();
                if buf.len() == 0 {
                    next = 'A';
                } else if buf.last().unwrap() == &'B' {
                    next = 'C';
                } else {
                    next = 'B';
                }
            }
            _ => {
                print!("{}", buf.iter().join(""));
                buf.clear();
                print!("{}", c);
                next = 'A';
            }
        };
    }
    println!("{}", buf.iter().join(""));
}
