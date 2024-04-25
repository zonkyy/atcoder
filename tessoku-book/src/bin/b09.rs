use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };
    const UPPER: usize = 1509;

    let mut v = vec![vec![0; UPPER + 1]; UPPER + 1];
    for (l, u, r, d) in abcd {
        v[u][l] += 1;
        v[d][r] += 1;
        v[d][l] -= 1;
        v[u][r] -= 1;
    }

    for r in 0..(UPPER + 1) {
        for c in 0..UPPER {
            v[r][c + 1] += v[r][c];
        }
    }
    for c in 0..(UPPER + 1) {
        for r in 0..UPPER {
            v[r + 1][c] += v[r][c];
        }
    }

    let mut ans = 0;
    for r in 0..=UPPER {
        for c in 0..=UPPER {
            if v[r][c] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
