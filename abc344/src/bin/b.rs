use itertools::Itertools;
use proconio::{fastout, input, marker::*};
use std::io;

#[fastout]
fn main() {
    let mut buffer = String::new();
    let mut v = Vec::new();
    while let Ok(n) = io::stdin().read_line(&mut buffer) {
        if n == 0 {
            break;
        }
        v.push(buffer.trim().to_string());
        buffer.clear();
    }

    println!("{}", v.iter().rev().join("\n"));
}
