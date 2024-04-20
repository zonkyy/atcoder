use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    };

    let mut acc = vec![vec![0; w + 1]];
    for r in 0..h {
        let mut row = vec![0];
        for c in 0..w {
            row.push(row[c] + x[r][c]);
        }
        acc.push(row);
    }

    for r in 1..(h + 1) {
        for c in 0..(w + 1) {
            acc[r][c] += acc[r - 1][c];
        }
    }

    for (a, b, c, d) in abcd {
        let ans = acc[c][d] + acc[a - 1][b - 1] - acc[c][b - 1] - acc[a - 1][d];
        println!("{}", ans);
    }
}
