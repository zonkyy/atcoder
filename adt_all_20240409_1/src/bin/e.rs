use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };

    let mut ab = ab
        .iter()
        .enumerate()
        .map(|(i, (a, b))| (i + 1, a, a + b))
        .collect::<Vec<_>>();
    ab.sort_by(|a, b| (b.1 * a.2).cmp(&(a.1 * b.2)));
    println!(
        "{}",
        ab.iter()
            .map(|(i, _, _)| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
