use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        q: usize,
        queries: [(usize, usize); q],
    };

    let mut v = Vec::new();
    for (a, b) in queries {
        if a == 1 {
            v.push(b);
        } else {
            println!("{}", v[v.len() - b]);
        }
    }
}
