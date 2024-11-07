use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    let mut ans: u128 = 0;
    for &i in a.iter() {
        ans += i as u128 * (n - 1) as u128;
    }

    a.sort();
    let mut l = 0;
    let mut r = n - 1;
    let mut m_cnt = 0;
    while l < r {
        if a[l] + a[r] >= 10usize.pow(8) {
            m_cnt += r - l;
            r -= 1;
        } else {
            l += 1;
        }
    }

    println!("{}", ans - 10u128.pow(8) * m_cnt as u128);
}
