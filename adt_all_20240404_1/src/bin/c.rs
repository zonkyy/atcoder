use proconio::{fastout, input, marker::*};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        mut pq: [char; 2],
    };

    pq.sort();

    let m = HashMap::from([
        ('A', 0),
        ('B', 3),
        ('C', 4),
        ('D', 8),
        ('E', 9),
        ('F', 14),
        ('G', 23),
    ]);

    println!("{:?}", m[&pq[1]] - m[&pq[0]]);
}
