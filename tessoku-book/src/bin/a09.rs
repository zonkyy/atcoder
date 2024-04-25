use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); n],
    };
    const UPPER: usize = 1509;

    let mut v = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        v[a][b] += 1;
        v[c + 1][d + 1] += 1;
        v[c + 1][b] -= 1;
        v[a][d + 1] -= 1;
    }

    for r in 0..(h + 1) {
        for c in 0..w {
            v[r][c + 1] += v[r][c];
        }
    }
    for c in 0..(w + 1) {
        for r in 0..h {
            v[r + 1][c] += v[r][c];
        }
    }

    for r in 0..h {
        for c in 0..w {
            print!("{}", v[r][c]);
            if c < w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
