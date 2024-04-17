use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let mut ans = a.clone();
    for i in 0..n {
        ans[0][i] = '0';
        ans[n - 1][i] = '0';
        ans[i][0] = '0';
        ans[i][n - 1] = '0';
    }

    for i in 1..n {
        ans[0][i] = a[0][i - 1];
        ans[i][n - 1] = a[i - 1][n - 1];
        ans[n - 1][i - 1] = a[n - 1][i];
        ans[i - 1][0] = a[i][0];
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
