use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        field: [Chars; h],
    };

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for r in 0..h {
        for c in 0..w {
            if r == 0 && c == 0 {
                dp[r][c] = 1;
                continue;
            }
            if field[r][c] == '#' {
                continue;
            }

            let mut v = 0;
            if r != 0 && field[r - 1][c] == '.' {
                v += dp[r - 1][c];
            }
            if c != 0 && field[r][c - 1] == '.' {
                v += dp[r][c - 1];
            }
            dp[r][c] = v;
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
