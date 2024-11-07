use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    };

    for r in 0..n {
        for c in 0..n {
            if a[r][c] != b[r][c] {
                println!("{} {}", r + 1, c + 1);
            }
        }
    }
}
