use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    };

    for r in 0..h {
        for c in 0..(w - 1) {
            if s[r][c] == 'T' && s[r][c + 1] == 'T' {
                s[r][c] = 'P';
                s[r][c + 1] = 'C';
            }
        }
        println!("{}", s[r].iter().collect::<String>());
    }
}
