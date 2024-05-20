use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        field: [Chars; n],
    };

    let mut row_other_cnt = vec![vec![0; n]; n];
    let mut col_other_cnt = vec![vec![0; n]; n];
    for a in 0..n {
        let mut row_cnt = 0;
        let mut col_cnt = 0;
        for b in 0..n {
            if field[a][b] == 'o' {
                row_cnt += 1;
            }
            if field[b][a] == 'o' {
                col_cnt += 1;
            }
        }

        for b in 0..n {
            if field[a][b] == 'o' && row_cnt > 0 {
                row_other_cnt[a][b] = row_cnt - 1;
            }
            if field[b][a] == 'o' && col_cnt > 0 {
                col_other_cnt[b][a] = col_cnt - 1;
            }
        }
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 0..n {
            ans += row_other_cnt[r][c] * col_other_cnt[r][c];
        }
    }

    println!("{}", ans);
}
