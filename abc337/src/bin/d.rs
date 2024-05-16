use std::cmp;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };
    const UPPER: usize = 2 * 10usize.pow(5) + 9;

    let mut ans = UPPER;
    for r in 0..h {
        let mut left = 0;

        'start: loop {
            let mut right = left + k - 1;
            if right >= w {
                break;
            }
            if let Some(idx) = s[r][left..=right].iter().rposition(|&c| c == 'x') {
                left = left + idx + 1;
                continue 'start;
            }

            let mut dotcnt = s[r][left..=right].iter().filter(|&&c| c == '.').count();
            ans = ans.min(dotcnt);

            // x でない限り k 個を 1 つずらす
            while right + 1 < w {
                // right
                right += 1;
                if s[r][right] == 'x' {
                    left = right;
                    continue 'start;
                }
                if s[r][right] == '.' {
                    dotcnt += 1;
                }

                // left
                if s[r][left] == '.' {
                    dotcnt -= 1;
                }
                left += 1;

                ans = ans.min(dotcnt);
            }

            left = right;

            if left == w - 1 {
                break;
            }
        }
    }

    // 転置
    let mut t = vec![vec!['.'; h]; w];
    for r in 0..h {
        for c in 0..w {
            t[c][r] = s[r][c];
        }
    }

    for c in 0..w {
        let mut left = 0;

        'start: loop {
            let mut right = left + k - 1;
            if right >= h {
                break;
            }
            if let Some(idx) = t[c][left..=right].iter().rposition(|&c| c == 'x') {
                left = left + idx + 1;
                continue 'start;
            }

            let mut dotcnt = t[c][left..=right].iter().filter(|&&c| c == '.').count();
            ans = ans.min(dotcnt);

            // x でない限り k 個を 1 つずらす
            while right + 1 < h {
                // right
                right += 1;
                if t[c][right] == 'x' {
                    left = right;
                    continue 'start;
                }
                if t[c][right] == '.' {
                    dotcnt += 1;
                }

                // left
                if t[c][left] == '.' {
                    dotcnt -= 1;
                }
                left += 1;

                ans = ans.min(dotcnt);
            }

            left = right;

            if left == h - 1 {
                break;
            }
        }
    }

    if ans == UPPER {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
