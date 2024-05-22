use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    };
    a.sort();

    let mut acc = vec![0; n];
    for i in 1..n {
        acc[i] = a[i] - a[i - 1];
    }

    for i in 1..n {
        acc[i] += acc[i - 1];
    }

    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    while r < n {
        if acc[r] - acc[l] < m {
            ans = ans.max(r - l);
            r += 1;
        } else {
            l += 1;
        }
    }

    println!("{}", ans + 1);
}
