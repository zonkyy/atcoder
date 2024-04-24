use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        q: usize,
        queries: [(usize, usize); q],
    };

    let mut v = Vec::new();
    for (l, r) in queries {
        if l == 1 {
            v.push(r);
        } else if l == 2 {
            println!("{}", v[v.len() - r]);
        }
    }
}
