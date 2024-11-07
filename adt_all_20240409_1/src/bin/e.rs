use num::Rational64;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };

    let mut v = Vec::new();
    for (i, (a, b)) in ab.iter().enumerate() {
        v.push((i + 1, Rational64::new(*a, *a + *b)));
    }
    v.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "{}",
        v.iter()
            .map(|(i, _)| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
