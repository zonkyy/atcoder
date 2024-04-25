use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    };
    const UPPER: usize = 1509;

    let mut v = vec![vec![0; UPPER + 1]; UPPER + 1];
    for (x, y) in xy {
        v[y][x] += 1;
    }

    for ri in 0..(UPPER + 1) {
        for ci in 0..UPPER {
            v[ri][ci + 1] += v[ri][ci];
        }
    }

    for ci in 0..(UPPER + 1) {
        for ri in 0..UPPER {
            v[ri + 1][ci] += v[ri][ci];
        }
    }

    for (ci1, ri1, ci2, ri2) in abcd {
        let ans = v[ri2][ci2] - v[ri1 - 1][ci2] - v[ri2][ci1 - 1] + v[ri1 - 1][ci1 - 1];
        println!("{}", ans);
    }
}
